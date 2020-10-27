//!
//! The Zandbox main integration test.
//!

use std::fs::File;
use std::process::Command;

use num_old::BigUint;
use serde_json::json;

use crate::database::client::Client as DatabaseClient;
use std::io::Write;

static MANIFEST_PATH: &str = "/home/hedgar/src/curve-zinc/";

static POSTGRESQL_URL: &str = "postgres://postgres@localhost/zinc";

static OWNER_ADDRESS: &str = "0x36615cf349d7f6344891b1e7ca7c72883f5dc049";

static TOKEN_ADDRESS_ETH: &str = "0x0000000000000000000000000000000000000000";
static TOKEN_ADDRESS_DAI: &str = "0x3bdfbbfdcf051c6ec5a741cc0fde89e30ff2f824";

#[tokio::test]
#[cfg_attr(not(feature = "integration-tests"), ignore)]
async fn ok_curve() {
    let provider = zksync::Provider::new(zksync::Network::Localhost);

    let database_client = DatabaseClient::new(POSTGRESQL_URL)
        .await
        .expect("Database client initialization");
    database_client
        .delete_fields()
        .await
        .expect("Database contract storage deleting");
    database_client
        .delete_contracts()
        .await
        .expect("Database contracts deleting");

    let output = Command::new(zinc_const::app_name::ZARGO)
        .arg("publish")
        .arg("--manifest-path")
        .arg(MANIFEST_PATH)
        .arg("--instance")
        .arg("test")
        .arg("--deposit-token")
        .arg("ETH")
        .arg("--deposit-amount")
        .arg("0.001_E18")
        .output()
        .expect("Zargo publish process waiting");
    let stdout = String::from_utf8_lossy(output.stdout.as_slice());
    let stderr = String::from_utf8_lossy(output.stderr.as_slice());
    let address = stdout
        .lines()
        .find(|line| line.contains("Address"))
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .expect("Zargo output format is invalid")
        })
        .expect(
            format!(
                "Zargo stdout format is invalid: {}",
                String::from_utf8_lossy(output.stdout.as_slice())
            )
            .as_str(),
        );
    if !output.status.success() {
        panic!("Zargo publish process failure: {}", stderr);
    }

    let input_path = format!(
        "{}/data/{}.{}",
        MANIFEST_PATH,
        zinc_const::file_name::INPUT,
        zinc_const::extension::JSON
    );
    File::create(input_path)
        .expect("Input file creating")
        .write_all(
            serde_json::to_string_pretty(&json!({
              "type": "contract",
              "storage": [
                address,
                [
                  "0",
                  "0"
                ],
                [
                  "0x0",
                  "0x1"
                ],
                "100"
              ],
              "msg": {
                "sender": OWNER_ADDRESS,
                "recipient": address,
                "token_address": TOKEN_ADDRESS_ETH,
                "amount": "1E18"
              },
              "arguments": {
                "get_dx": {
                  "deposit_token_address": TOKEN_ADDRESS_DAI,
                  "withdraw_token_address": TOKEN_ADDRESS_ETH,
                  "to_withdraw": "0.1E18"
                },
                "new": {
                  "_tokens": [
                    TOKEN_ADDRESS_ETH,
                    TOKEN_ADDRESS_DAI
                  ],
                  "_amplifier": "100"
                },
                "get_dy": {
                  "deposit_token_address": TOKEN_ADDRESS_ETH,
                  "withdraw_token_address": TOKEN_ADDRESS_DAI,
                  "to_deposit": "0.1E18"
                },
                "deposit": {},
                "swap": {
                  "withdraw_address": OWNER_ADDRESS,
                  "withdraw_token_address": TOKEN_ADDRESS_DAI,
                  "min_withdraw": "0.05E18"
                }
              }
            }))
            .expect(zinc_const::panic::DATA_CONVERSION)
            .as_bytes(),
        )
        .expect("Input file writing");

    let output = Command::new(zinc_const::app_name::ZARGO)
        .arg("call")
        .arg("--manifest-path")
        .arg(MANIFEST_PATH)
        .arg("--address")
        .arg(address)
        .arg("--method")
        .arg("deposit")
        .output()
        .expect("Zargo deposit call process waiting");
    let stderr = String::from_utf8_lossy(output.stderr.as_slice());
    if !output.status.success() {
        panic!("Zargo deposit call process failure: {}", stderr);
    }

    let account_info = provider
        .account_info(
            address["0x".len()..]
                .parse()
                .expect(zinc_const::panic::DATA_CONVERSION),
        )
        .await
        .expect("Account info getting");
    assert!(
        account_info
            .committed
            .balances
            .get("ETH")
            .expect("Balance error")
            .0
            > BigUint::from(1_000_000_000_000_000_000_u64),
        "ETH deposit has failed"
    );
}
