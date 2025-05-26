use environment::Environment;
use interpreter::eval_statements;
use lexer::tokenize;
use parser::Parser;

mod lexer;
mod ast;
mod parser;
mod environment;
mod interpreter;

fn main() {
    let mut env = Environment::new();
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdin().read_line(&mut input).unwrap();

        let tokens = tokenize(&input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        eval_statements(statements, &mut env);
    }
}
