mod lexer;
mod token;
mod parser;

use crate::lexer::lex;
use crate::parser::{parse, dump_ast};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // read file
    let file = File::open("examples/test.wly").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    let tokens = lex(&contents);

    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    let statements = parse(tokens);

    dump_ast(&statements);    
}
