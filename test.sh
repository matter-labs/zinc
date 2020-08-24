#!/usr/bin/env bash

set -Cex

#curl -v -X POST -H 'Content-Type: application/json' --data '@zinc-server/test/main.json' 'localhost/api/v1/template?account_id=666&name=test&version=0.1.0'
#curl -v -X POST -H 'Content-Type: application/json' --data '@zinc-server/test/new.json' 'localhost/api/v1/contract?template_id=666&account_id=777'
#curl -v -X POST -H 'Content-Type: application/json' --data '@zinc-server/test/inc.json' 'localhost/api/v1/contract/call?template_id=666&account_id=777&method=inc'
#curl -v -X POST -H 'Content-Type: application/json' --data '@zinc-server/test/inc.json' 'localhost/api/v1/contract/call?template_id=666&account_id=777&method=inc'
#curl -v -X POST -H 'Content-Type: application/json' --data '@zinc-server/test/inc.json' 'localhost/api/v1/contract/call?template_id=666&account_id=777&method=inc'
#curl -v -X POST -H 'Content-Type: application/json' --data '@zinc-server/test/dec.json' 'localhost/api/v1/contract/call?template_id=666&account_id=777&method=dec'
curl -v -X POST -H 'Content-Type: application/json' --data '@zinc-server/test/inc.json' 'localhost/api/v1/contract/call?template_id=666&account_id=777&method=inc'
