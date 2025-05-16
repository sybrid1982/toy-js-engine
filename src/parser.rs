use crate::{ast::{Expression, Operator, Statement}, lexer::Token};

pub fn parse_tokens(tokens: &[Token]) -> Vec<Statement> {
    let mut statements: Vec<Statement> = Vec::new();

    let mut current_statement_expressions: Vec<Expression> = Vec::new();

    tokens.iter().for_each(|token| {
        match token {
            Token::Number(value) => {
                current_statement_expressions.push(
                    Expression::NumberLiteral(*value)
                );
            },
            Token::Semicolon => {
                evaluate_current_statement_expressions(&mut statements, &mut current_statement_expressions);
            },
            _ => {

            }
        } 
    });

    statements
}

fn evaluate_current_statement_expressions(statements: &mut Vec<Statement>, current_statement_expressions: &mut Vec<Expression>) {
    if current_statement_expressions.len() == 1 && current_statement_expressions[0].is_number_literal() {
        statements.push(Statement::ExpressionStatement(current_statement_expressions[0].clone()));
    }
    current_statement_expressions.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_interpret_numbers_as_number_expressions() {
        let tokens = vec![Token::Number(24f64), Token::Semicolon];
        let result = parse_tokens(&tokens);
        assert_eq!(result[0], Statement::ExpressionStatement(Expression::NumberLiteral(24f64)));
    }

}