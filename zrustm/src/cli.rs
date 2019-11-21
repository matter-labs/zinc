use clap::*;

pub fn build_arguments() -> App<'static, 'static> {
    let circuit_arg = Arg::with_name("circuit")
        .short("c")
        .long("circuit")
        .value_name("FILE")
        .help("Circuit's bytecode file")
        .required(true)
        .takes_value(true);

    let witness_arg = Arg::with_name("witness")
        .short("w")
        .long("witness")
        .value_name("WITNESS")
        .help("Witness")
        .takes_value(true);

    let output_arg = Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("FILE")
        .help("Output file")
        .required(true)
        .takes_value(true);

    let key_arg = Arg::with_name("key")
        .short("k")
        .long("key")
        .value_name("FILE")
        .help("Proving key file")
        .required(true)
        .takes_value(true);

    let proof_arg = Arg::with_name("proof")
        .short("p")
        .long("proof")
        .value_name("FILE")
        .help("Zero-knowledge proof file")
        .required(true)
        .takes_value(true);

    App::new("zrustm")
        .version("0.1")
        .about("ZRust Virtual Machine")
        .arg(Arg::with_name("version")
            .short("v")
            .help("Prints out the version of zrustm"))
        .subcommand(SubCommand::with_name("exec")
            .about("Executes circuit and prints program's output")
            .arg(circuit_arg.clone())
            .arg(witness_arg.clone()))
        .subcommand(SubCommand::with_name("gen-key")
            .about("Generates proving key for the circuit")
            .arg(circuit_arg.clone())
            .arg(output_arg.clone()))
        .subcommand(SubCommand::with_name("gen-proof")
            .about("Generate zero-knowledge proof for given witness")
            .arg(circuit_arg.clone())
            .arg(witness_arg.clone())
            .arg(key_arg.clone())
            .arg(output_arg.clone()))
        .subcommand(SubCommand::with_name("verify")
            .about("Verifies zero-knowledge proof")
            .arg(key_arg.clone())
            .arg(proof_arg.clone()))
}
