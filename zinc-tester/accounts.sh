#!/usr/bin/env bash

set -Cex

node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- wallets add '0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110'
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- deposit 100 ETH '0x36615Cf349d7F6344891B1e7CA7C72883F5dc049'
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- deposit 100 DAI '0x36615Cf349d7F6344891B1e7CA7C72883F5dc049'
cargo run --release --bin key-changer -- --private-key '0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110'

node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- wallets add '0xac1e735be8536c6534bb4f17f06f6afc73b2b5ba84ac2cfb12f7461b20c0bbe3'
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- deposit 100 ETH '0xa61464658AfeAf65CccaaFD3a512b69A83B77618'
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- deposit 100 DAI '0xa61464658AfeAf65CccaaFD3a512b69A83B77618'
cargo run --release --bin key-changer -- --private-key '0xac1e735be8536c6534bb4f17f06f6afc73b2b5ba84ac2cfb12f7461b20c0bbe3'

node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- wallets add '0xd293c684d884d56f8d6abd64fc76757d3664904e309a0645baf8522ab6366d9e'
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- deposit 100 ETH '0x0D43eB5B8a47bA8900d84AA36656c92024e9772e'
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- deposit 100 DAI '0x0D43eB5B8a47bA8900d84AA36656c92024e9772e'
cargo run --release --bin key-changer -- --private-key '0xd293c684d884d56f8d6abd64fc76757d3664904e309a0645baf8522ab6366d9e'

node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- wallets add '0x850683b40d4a740aa6e745f889a6fdc8327be76e122f5aba645a5b02d0248db8'
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- deposit 100 ETH '0xA13c10C0D5bd6f79041B9835c63f91de35A15883'
node "${ZKSYNC_HOME}/infrastructure/zcli/build/index.js" -- deposit 100 DAI '0xA13c10C0D5bd6f79041B9835c63f91de35A15883'
cargo run --release --bin key-changer -- --private-key '0x850683b40d4a740aa6e745f889a6fdc8327be76e122f5aba645a5b02d0248db8'
