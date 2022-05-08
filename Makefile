log:
	wrangler tail | jq '.'

pub:
	wrangler build
	wrangler publish

test-get:
	curl https://cloudflare-rust-kv-example.paul-asvb.workers.dev/kv/mykey

test-post:
	curl -X POST  https://webrtc-session.paul-asvb.workers.dev/create \
   -H 'Content-Type: application/json' \
   -d '{"data":[]}'

test: test-post test-get
	echo "tested"