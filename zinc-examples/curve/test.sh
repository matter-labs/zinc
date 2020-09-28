#!/usr/bin/env bash

set -ex

node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" wallets add '0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110'
psql -h localhost -U postgres -d zinc <<< 'DELETE FROM zandbox.fields; DELETE FROM zandbox.contracts;'

cat >'./data/witness_new.json' <<EOF
{
  "_amplifier": "100"
}
EOF
export ADDRESS=$(zargo publish --instance test 2>&1 | grep 'Address' | awk -F' ' '{print $2}')
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" account "${ADDRESS}"

cat >'./data/witness_deposit.json' <<EOF
{
  "tx": [
    {
      "sender": "0x36615cf349d7f6344891b1e7ca7c72883f5dc049",
      "recipient": "${ADDRESS}",
      "token_id": "ETH",
      "amount": "1.0_E18"
    },
    {
      "sender": "0x36615cf349d7f6344891b1e7ca7c72883f5dc049",
      "recipient": "${ADDRESS}",
      "token_id": "DAI",
      "amount": "1.0_E18"
    }
  ]
}
EOF
zargo call --method deposit --address "${ADDRESS}"
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" account "${ADDRESS}"

cat >'./data/witness_swap.json' <<EOF
{
  "tx": {
    "sender": "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049",
    "recipient": "${ADDRESS}",
    "token_id": "ETH",
    "amount": "0.5_E18"
  },
  "withdraw_address": "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049",
  "withdraw_token_id": "DAI",
  "min_withdraw": "0.4_E18"
}
EOF
zargo call --method swap --address "${ADDRESS}"
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" account "${ADDRESS}"
