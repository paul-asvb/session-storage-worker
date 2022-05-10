log:
	wrangler tail | jq '.'

pub:
	wrangler build
	wrangler publish

test-get:
	curl https://webrtc-session.paul-asvb.workers.dev/

test-reset:
	curl -X POST  https://webrtc-session.paul-asvb.workers.dev/ \
   -H 'Content-Type: application/json' \
   -d '{"sessions":{}}'

test-delete:
	curl -X DELETE  https://webrtc-session.paul-asvb.workers.dev/delete \
   -H 'Content-Type: application/json' \
   -d 'myid'

test-create:
	curl -X POST  https://webrtc-session.paul-asvb.workers.dev/create \
   -H 'Content-Type: application/json' \
   -d '{"id":"myid","session":"session_string"}'

test: test-post test-get
	echo "tested"