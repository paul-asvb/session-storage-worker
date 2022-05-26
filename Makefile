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

test-clear:
	curl -X DELETE  https://webrtc-session.paul-asvb.workers.dev \
   -H 'Content-Type: application/json' -v

test-delete:
	curl -X DELETE  https://webrtc-session.paul-asvb.workers.dev/delete \
   -H 'Content-Type: application/json' \
   -d 'myid'

test-create:
	curl -X POST  https://webrtc-session.paul-asvb.workers.dev/session \
   -H 'Content-Type: application/json' \
   -d '{"peer_id":"myid","offer":{"type":"type1","sdp":"sdp_example"}}'

test: test-post test-get
	echo "tested"