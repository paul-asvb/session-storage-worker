use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use worker::*;

mod utils;

type Sessions = HashMap<String, Session>;
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
        .get_async("/", get_sessions)
        .post_async("/", set_sessions)
        .delete_async("/", clear_sessions)
        .options_async("/", options_handler)
        .post_async("/create", create_session)
        .delete_async("/delete", delete_session)
        .get("/version", |_, _| Response::ok("version"))
        .run(req, env)
        .await
}

async fn get_sessions(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let store = match ctx.kv(STORE) {
        Ok(s) => s,
        Err(_) => return Response::error("server error: kv not found", 500),
    };

    match store.get(NAMESPACE).json::<SessionStore>().await {
        Ok(Some(sessions)) => {
            let res = Response::from_json(&sessions.sessions).unwrap();
            Ok(with_cors(res))
        }
        Ok(None) => Response::error("No sessions found", 404),
        Err(err) => Response::error(format!("server error: {:?}", err), 500),
    }
}

async fn set_sessions(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
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
            let res = Response::ok("success").unwrap();
            return Ok(with_cors(res));
        } else {
            return Response::error("storage error", 500);
        }
    } else {
        return Response::error("storage error", 500);
    }
}

async fn clear_sessions(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let store = match ctx.kv(STORE) {
        Ok(s) => s,
        Err(err) => return Response::error(format!("{:?}", err), 204),
    };

    let content = SessionStore {
        sessions: HashMap::new(),
    };

    let put = store.put(NAMESPACE, content);
    if put.is_ok() {
        let exc = put.unwrap().execute().await;
        if exc.is_ok() {
            let res = Response::ok("success").unwrap();
            return Ok(with_cors(res));
        } else {
            return Response::error("storage error", 500);
        }
    } else {
        return Response::error("storage error", 500);
    }
}

async fn create_session(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
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

    session_store
        .sessions
        .insert(new_session.id.clone(), new_session);

    let put = store.put(NAMESPACE, session_store);
    if put.is_ok() {
        let exc = put.unwrap().execute().await;
        if exc.is_ok() {
            let res = Response::ok("success").unwrap();
            return Ok(with_cors(res));
        } else {
            return Response::error("storage error", 500);
        }
    } else {
        return Response::error("storage error", 500);
    }
}

async fn delete_session(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let store = match ctx.kv(STORE) {
        Ok(s) => s,
        Err(err) => return Response::error(format!("{:?}", err), 204),
    };

    let key_to_delete: String = match req.text().await {
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

    session_store.sessions.remove(&key_to_delete);

    let put = store.put(NAMESPACE, session_store);
    if put.is_ok() {
        let exc = put.unwrap().execute().await;
        if exc.is_ok() {
            let res = Response::ok("success").unwrap();
            return Ok(with_cors(res));
        } else {
            return Response::error("storage error", 500);
        }
    } else {
        return Response::error("storage error", 500);
    }
}

async fn options_handler(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Ok(with_cors(Response::ok("success").unwrap()))
}

fn with_cors(res: Response) -> Response {
    let mut headers = Headers::default();
    headers
        .append(&"Access-Control-Allow-Origin".to_string(), &"*".to_string())
        .unwrap();
    headers
        .append(&"Content-type".to_string(), &"application/json".to_string())
        .unwrap();
    headers
        .append(&"Access-Control-Max-Age".to_string(), &"86400".to_string())
        .unwrap();
    res.with_headers(headers)
}
