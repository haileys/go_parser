use std::process::{exit, Command};

const RAGEL_SOURCE: &'static str = "src/lex/scan.rl";

fn main() {
    println!("cargo:rerun-if-changed={}", RAGEL_SOURCE);

    let code = Command::new("/Users/charlie/code/ragel-rust/ragel/ragel")
        .args(&["--host-lang=Rust", "-o", "src/lex/scan.rs", RAGEL_SOURCE])
        .status()
        .unwrap()
        .code()
        .unwrap_or(1);

    exit(code);
}
