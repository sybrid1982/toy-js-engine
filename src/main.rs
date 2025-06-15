use environment::Environment;
use lexer::tokenize;
use parser::Parser;
use crate::{interpreter::{process_statements}, parser::separate_out_statements_and_parser_errors};

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
        let statement_results = parser.parse();

        let (statements, parser_errors) = separate_out_statements_and_parser_errors(statement_results);

        if parser_errors.len() > 0 {
            for error in parser_errors {
                println!("{}", error)
            }
        } else {
            process_statements(statements, &mut env);
        }
    }
}


