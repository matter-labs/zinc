//!
//! The Zandbox main integration test.
//!

use std::process::Command;
use std::fs::File;

use num_old::BigUint;
use serde_json::json;
use tokio::runtime::Runtime;

use crate::database::client::Client as DatabaseClient;
use std::io::Write;

static MANIFEST_PATH: &str = "../zinc-examples/curve/";

static OWNER_ADDRESS: &str = "0x36615cf349d7f6344891b1e7ca7c72883f5dc049";

#[test]
fn ok_curve() {
    let mut runtime = Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME);

    let provider = zksync::Provider::new(zksync::Network::Localhost);

    let database_client = runtime
        .block_on(DatabaseClient::new(
            zinc_const::postgresql::HOST.to_owned(),
            zinc_const::postgresql::PORT,
            zinc_const::postgresql::USER.to_owned(),
            zinc_const::postgresql::PASSWORD.to_owned(),
            zinc_const::postgresql::DATABASE.to_owned(),
        ))
        .expect("Database client initialization");
    runtime
        .block_on(database_client.delete_fields())
        .expect("Database contract storage deleting");
    runtime
        .block_on(database_client.delete_contracts())
        .expect("Database contracts deleting");

    let output = Command::new(zinc_const::app_name::ZARGO)
        .arg("publish")
        .arg("--manifest-path")
        .arg(MANIFEST_PATH)
        .arg("--instance")
        .arg("test")
        .output()
        .expect("Zargo publish process waiting");
    let stdout = String::from_utf8_lossy(output.stdout.as_slice());
    let address = stdout
        .lines()
        .find(|line| line.contains("Address"))
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .expect("Zargo output format is invalid")
        })
        .expect("Zargo output format is invalid");
    assert!(output.status.success(), "Zargo publish process failure");

    let witness_new_path = format!("{}/data/witness_{}.json", MANIFEST_PATH, "new");
    File::create(witness_new_path).expect("Constructor input file creating").write_all(serde_json::to_string_pretty(&json!({
        "_amplifier": "100",
    })).expect(zinc_const::panic::DATA_CONVERSION).as_bytes()).expect("Constructor input file writing");

    let witness_deposit_path = format!("{}/data/witness_{}.json", MANIFEST_PATH, "deposit");
    File::create(witness_deposit_path).expect("Deposit input file creating").write_all(serde_json::to_string_pretty(&json!({
        "tx": [
            {
                "sender": OWNER_ADDRESS,
                "recipient": address,
                "token_id": "ETH",
                "amount": "1.0_E18"
            },
            {
                "sender": OWNER_ADDRESS,
                "recipient": address,
                "token_id": "DAI",
                "amount": "1.0_E18"
            }
        ]
    })).expect(zinc_const::panic::DATA_CONVERSION).as_bytes()).expect("Deposit input file writing");

    let witness_swap_path = format!("{}/data/witness_{}.json", MANIFEST_PATH, "swap");
    File::create(witness_swap_path).expect("Swap input file creating").write_all(serde_json::to_string_pretty(&json!({
        "tx": {
            "sender": OWNER_ADDRESS,
            "recipient": address,
            "token_id": "ETH",
            "amount": "0.1_E18"
        },
        "withdraw_address": OWNER_ADDRESS,
        "withdraw_token_id": "DAI",
        "min_withdraw": "0.01_E18"
    })).expect(zinc_const::panic::DATA_CONVERSION).as_bytes()).expect("Swap input file writing");

    let status = Command::new(zinc_const::app_name::ZARGO)
        .arg("call")
        .arg("--manifest-path")
        .arg(MANIFEST_PATH)
        .arg("--address")
        .arg(address)
        .arg("--method")
        .arg("deposit")
        .spawn()
        .expect("Zargo deposit call process spawning")
        .wait()
        .expect("Zargo deposit call process waiting");
    assert!(status.success(), "Zargo deposit call process failure");

    let account_info = runtime.block_on(provider.account_info(address["0x".len()..].parse().expect(zinc_const::panic::DATA_CONVERSION))).expect("Account info getting");
    assert_eq!(account_info.committed.balances.get("ETH").expect("Balance error").0, BigUint::from(1_000_000_000_000_000_000_u64), "ETH deposit has failed");
    assert_eq!(account_info.committed.balances.get("DAI").expect("Balance error").0, BigUint::from(1_000_000_000_000_000_000_u64), "DAI deposit has failed");

    let status = Command::new(zinc_const::app_name::ZARGO)
        .arg("call")
        .arg("--manifest-path")
        .arg(MANIFEST_PATH)
        .arg("--address")
        .arg(address)
        .arg("--method")
        .arg("swap")
        .spawn()
        .expect("Zargo swap call process spawning")
        .wait()
        .expect("Zargo swap call process waiting");
    assert!(status.success(), "Zargo swap call process failure");

    let mut account_info = runtime.block_on(provider.account_info(address["0x".len()..].parse().expect(zinc_const::panic::DATA_CONVERSION))).expect("Account info getting");
    assert_eq!(account_info.committed.balances.get("ETH").expect("Balance error").0, BigUint::from(1_100_000_000_000_000_000_u64), "ETH client-side swap has failed");
    let after_first_swap = account_info.committed.balances.remove("DAI").expect("Balance error").0;
    assert!(after_first_swap < BigUint::from(1_000_000_000_000_000_000_u64), "DAI contract-side swap has failed, as the token amount has not decreased");

    let status = Command::new(zinc_const::app_name::ZARGO)
        .arg("call")
        .arg("--manifest-path")
        .arg(MANIFEST_PATH)
        .arg("--address")
        .arg(address)
        .arg("--method")
        .arg("swap")
        .spawn()
        .expect("Zargo swap call process spawning")
        .wait()
        .expect("Zargo swap call process waiting");
    assert!(status.success(), "Zargo swap call process failure");

    let mut account_info = runtime.block_on(provider.account_info(address["0x".len()..].parse().expect(zinc_const::panic::DATA_CONVERSION))).expect("Account info getting");
    assert_eq!(account_info.committed.balances.get("ETH").expect("Balance error").0, BigUint::from(1_200_000_000_000_000_000_u64), "ETH client-side swap has failed");
    let after_second_swap = account_info.committed.balances.remove("DAI").expect("Balance error").0;
    assert!(after_second_swap < after_first_swap, "DAI contract-side swap has failed, as the token amount has not decreased");

    let status = Command::new(zinc_const::app_name::ZARGO)
        .arg("call")
        .arg("--manifest-path")
        .arg(MANIFEST_PATH)
        .arg("--address")
        .arg(address)
        .arg("--method")
        .arg("swap")
        .spawn()
        .expect("Zargo swap call process spawning")
        .wait()
        .expect("Zargo swap call process waiting");
    assert!(status.success(), "Zargo swap call process failure");

    let mut account_info = runtime.block_on(provider.account_info(address["0x".len()..].parse().expect(zinc_const::panic::DATA_CONVERSION))).expect("Account info getting");
    assert_eq!(account_info.committed.balances.get("ETH").expect("Balance error").0, BigUint::from(1_300_000_000_000_000_000_u64), "ETH client-side swap has failed");
    let after_third_swap = account_info.committed.balances.remove("DAI").expect("Balance error").0;
    assert!(after_third_swap < after_second_swap, "DAI contract-side swap has failed, as the token amount has not decreased");
}
