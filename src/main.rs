mod lexer;

use clap::Parser;
use lexer::Lexer;
use std::{fs, path::PathBuf};

#[derive(Parser)]
struct Opt {
    path: PathBuf,
}

fn main() {
    let opts = Opt::parse();
    let content = fs::read_to_string(opts.path).unwrap();

    let mut lexer = Lexer::new(&content);
    while !lexer.is_done() {
        println!("{:?}", lexer.next().unwrap())
    }
}
