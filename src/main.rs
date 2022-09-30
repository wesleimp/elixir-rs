mod lexer;

use clap::Parser;
use lexer::Lexer;
use std::{fs, io::Write, path::PathBuf};

#[derive(Parser)]
struct Opt {
    path: PathBuf,
}

fn main() {
    let opts = Opt::parse();
    let content = fs::read_to_string(opts.path).unwrap();

    let mut lexer = Lexer::new(&content);
    let mut f = fs::File::create("tokens.txt").unwrap();
    while !lexer.is_done() {
        let token = lexer.next().unwrap();
        println!("{:?}", token);
        writeln!(f, "{:?}", token).unwrap();
    }
}
