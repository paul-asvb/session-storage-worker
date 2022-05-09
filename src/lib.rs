use serde::{Deserialize, Serialize};

use worker::*;

mod utils;

type Sessions = Vec<Session>;
#[derive(Serialize, Deserialize)]
struct Session {
    id: String,
    session: String,
}

#[derive(Serialize, Deserialize)]
struct SessionStore {
    sessions: Sessions,
}

static STORE: &'static str = "webrtc_session";
static NAMESPACE: &'static str = "sessions";

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get_async("/", get_handler)
        .post_async("/", post_handler)
        .post_async("/create", create_handler)
        .get("/version", |_, _| Response::ok("version"))
        .run(req, env)
        .await
}

async fn get_handler(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let store = match ctx.kv(STORE) {
        Ok(s) => s,
        Err(_) => return Response::error("server error: kv not found", 500),
    };

    match store.get(NAMESPACE).json::<SessionStore>().await {
        Ok(Some(sessions)) => Response::from_json(&sessions.sessions),
        Ok(None) => Response::error("No sessions found", 404),
        Err(err) => Response::error(format!("server error: {:?}", err), 500),
    }
}

async fn post_handler(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let store = match ctx.kv(STORE) {
        Ok(s) => s,
        Err(err) => return Response::error(format!("{:?}", err), 204),
    };

    let content: SessionStore = match req.json().await {
        Ok(b) => b,
        _ => return Response::error("body parse error", 400),
    };

    let put = store.put(NAMESPACE, content);
    if put.is_ok() {
        let exc = put.unwrap().execute().await;
        if exc.is_ok() {
            return Response::ok("success");
        } else {
            return Response::error("storage error", 500);
        }
    } else {
        return Response::error("storage error", 500);
    }
}

async fn create_handler(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let store = match ctx.kv(STORE) {
        Ok(s) => s,
        Err(err) => return Response::error(format!("{:?}", err), 204),
    };

    let new_session: Session = match req.json().await {
        Ok(s) => s,
        Err(err) => {
            return Response::error(
                format!("body parse error: {:?} in {:?}", err, req.text().await),
                400,
            )
        }
    };

    let mut session_store = match store.get(NAMESPACE).json::<SessionStore>().await {
        Ok(Some(sessions)) => sessions,
        Ok(None) => return Response::error("No sessions found", 404),
        Err(err) => return Response::error(format!("server error: {:?}", err), 500),
    };

    session_store.sessions.push(new_session);

    let put = store.put(NAMESPACE, session_store);
    if put.is_ok() {
        let exc = put.unwrap().execute().await;
        if exc.is_ok() {
            return Response::ok("success");
        } else {
            return Response::error("storage error", 500);
        }
    } else {
        return Response::error("storage error", 500);
    }
}
