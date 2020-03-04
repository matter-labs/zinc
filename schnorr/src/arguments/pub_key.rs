use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::{eddsa, jubjub};
use std::io::Read;
use structopt::StructOpt;
use crate::arguments::fr_into_hex;
use serde_json::json;

#[derive(StructOpt)]
#[structopt(name = "gen-key", about = "recover public key from private key")]
pub struct PubKeyCommand {}

impl PubKeyCommand {
    pub fn execute(&self) {
        let params = AltJubjubBn256::new();
        let p_g = jubjub::FixedGenerators::SpendingKeyGenerator;

        let mut private_key_hex = String::new();
        std::io::stdin()
            .read_to_string(&mut private_key_hex)
            .unwrap();
        let bytes = hex::decode(private_key_hex.trim()).unwrap();
        let private_key = eddsa::PrivateKey::<Bn256>::read(bytes.as_slice()).unwrap();

        let public_key = eddsa::PublicKey::from_private(&private_key, p_g, &params);
        let (x, y) = {
            let (x, y) = public_key.0.into_xy();
            (fr_into_hex(x), fr_into_hex(y))
        };

        let public_key_json = json!({ "x": x, "y": y });
        let public_key_text = serde_json::to_string_pretty(&public_key_json).unwrap();
        println!("{}", public_key_text);
    }
}
