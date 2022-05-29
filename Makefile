log:
	wrangler tail | jq '.'

pub:
	wrangler build
	wrangler publish

test-get:
	curl https://webrtc-session.paul-asvb.workers.dev/

test-delete:
	curl -X DELETE  https://webrtc-session.paul-asvb.workers.dev/test_session \
   -H 'Content-Type: application/json'

test-create:
	curl -X POST  https://webrtc-session.paul-asvb.workers.dev/test_session \
   -H 'Content-Type: application/json' \
   -d '{"peer_id":"myid","offer":{"type":"type1","sdp":"sdp_example"}}'

test: test-post test-get
	echo "tested"