use clap::Shell;

include!("src/cli/arguments.rs");

fn main() {
    let out_dir = match std::env::var_os("SHELL_COMPLETION_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = build_arguments();
    app.gen_completions("zinc", Shell::Bash, out_dir);
}
