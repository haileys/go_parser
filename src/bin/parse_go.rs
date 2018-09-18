extern crate go_parser;

use std::env;
use std::fs;
use std::io;
use std::process;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Lex(go_parser::LexError),
}

fn main() -> Result<(), Error> {
    let args = env::args_os().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("usage: {} <file>", args[0].to_string_lossy());
        process::exit(1);
    }

    let source = fs::read_to_string(&args[1]).map_err(Error::Io)?;

    let tokens = go_parser::lex(&source).map_err(Error::Lex)?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
