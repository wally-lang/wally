mod lexer;
mod token;
mod parser;
mod interpreter;

use crate::lexer::lex;
use crate::parser::{parse, dump_ast};
use crate::interpreter::Interpreter;
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
    let statements = parse(tokens);

    println!("{:#?}", statements);

    dump_ast(&statements);    

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements);

    println!("{:#?}", interpreter);
}
