#!/usr/bin/env bash

set -Cex

curl -v -X POST -H 'Content-Type: application/json' --data '@zinc-server/test/post.json' 'localhost/api/v1/contract?contract_id=888&name=test3&version=0.1.0'
curl -v -X POST -H 'Content-Type: application/json' --data '@zinc-server/test/inc.json' 'localhost/api/v1/contract/call?contract_id=888&method=inc'
