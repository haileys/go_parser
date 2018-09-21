extern crate go_parser;

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;
use std::rc::Rc;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Lex(go_parser::LexError),
    Parse(go_parser::ParseError),
}

fn main() -> Result<(), Error> {
    let args = env::args_os().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("usage: {} <file>", args[0].to_string_lossy());
        process::exit(1);
    }

    let path = PathBuf::from(&args[1]);

    let source = fs::read_to_string(&path).map_err(Error::Io)?;

    let tokens = go_parser::lex(Rc::new(path), &source).map_err(Error::Lex)?;

    let ast = go_parser::parse(tokens).map_err(Error::Parse)?;

    println!("{:#?}", ast);

    Ok(())
}
