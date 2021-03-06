# session storage

session storage for webrtc sessions implemented on cloudflare worker with cloudflare KV-store

[DEMO](https://webrtc-session.paul-asvb.workers.dev)

[worker docs](https://docs.rs/worker/latest/worker/)
## Prerequisities
rust toolchain and: 
```bash
cargo install wrangler
```

## dev

```bash
cargo test
wrangler dev
```

## release
```bash
wrangler build 
wrangler publish
```

## log
```bash
wrangler tail | jq '.'
```

## publish
```bash
make pub
```

## Add kv store
```bash
 wrangler kv:namespace create "namespace_name"
 ```
 and copy request answer into wrangler.toml

 ## Initalisation



Initalized using [`workers-rs`](https://github.com/wrangler copycloudflare/workers-rs).

This template is designed for compiling Rust to WebAssembly and publishing the resulting worker to 
Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).
