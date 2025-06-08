use environment::Environment;
use lexer::tokenize;
use parser::Parser;
use crate::interpreter::interpreter::{eval_statements};

mod lexer;
mod ast;
mod parser;
mod environment;
mod interpreter;
mod integration_tests;
mod function;

fn main() {
    let mut env = Environment::new();
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().len() == 0 {
            break;
        }

        let tokens = tokenize(&input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        eval_statements(statements, &mut env);
    }
}
