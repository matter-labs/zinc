use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::pairing::ff::{PrimeField, PrimeFieldRepr};
use franklin_crypto::eddsa;
use serde_json::json;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

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
    pub fn execute(&self) {
        let params = AltJubjubBn256::new();

        let private_key_hex = std::fs::read_to_string(&self.key).unwrap();
        let bytes = hex::decode(private_key_hex.trim()).unwrap();
        let private_key = eddsa::PrivateKey::<Bn256>::read(bytes.as_slice()).unwrap();

        let message = if &self.message_path.to_string_lossy() == "-" {
            let mut message = Vec::new();
            std::io::stdin().read_to_end(&mut message).unwrap();
            message
        } else {
            std::fs::read(&self.message_path).unwrap()
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

        let signature_json = serde_json::to_string_pretty(&value).unwrap();
        println!("{}", signature_json)
    }
}

pub fn fr_into_hex<Fr: PrimeField>(fr: Fr) -> String {
    let mut buffer = Vec::<u8>::new();

    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");

    let num = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &buffer);
    format!("0x{}", num.to_str_radix(16))
}
