use std::io::Read;
use std::path::PathBuf;

use serde_json::json;
use structopt::StructOpt;

use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::eddsa;

use crate::arguments::{fr_into_hex, Error};

#[derive(StructOpt)]
#[structopt(name = "sign", about = "generate signature")]
pub struct SignCommand {
    #[structopt(short = "k", long = "key", help = "path to private key")]
    key: PathBuf,

    #[structopt(
        short = "m",
        long = "message",
        help = "path to file with message or '-' for stdin"
    )]
    message_path: PathBuf,
}

impl SignCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let params = AltJubjubBn256::new();

        let private_key_hex = std::fs::read_to_string(&self.key)?;
        let bytes = hex::decode(private_key_hex.trim())?;
        let private_key = eddsa::PrivateKey::<Bn256>::read(bytes.as_slice())?;

        let message = if &self.message_path.to_string_lossy() == "-" {
            let mut message = Vec::new();
            std::io::stdin().read_to_end(&mut message)?;
            message
        } else {
            std::fs::read(&self.message_path)?
        };

        let signature = schnorr::generate_signature(&params, &private_key, &message);
        let pub_key = schnorr::recover_public_key(&params, &private_key);

        let r = {
            let (x, y) = signature.r.into_xy();
            (fr_into_hex(x), fr_into_hex(y))
        };

        let s = fr_into_hex(signature.s);

        let pk = {
            let (x, y) = pub_key.0.into_xy();
            (fr_into_hex(x), fr_into_hex(y))
        };

        let value = json!({
            "r": {
                "x": r.0,
                "y": r.1
            },
            "s": s,
            "pk": {
                "x": pk.0,
                "y": pk.1
            }
        });

        let signature_json = serde_json::to_string_pretty(&value).expect("json");
        println!("{}", signature_json);

        Ok(())
    }
}
