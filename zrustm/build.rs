extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let out_dir = match std::env::var_os("CARGO_SHELL_COMPLETE_OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = build_arguments();
    app.gen_completions("zrustm", Shell::Bash, out_dir);
}
