use crate::{ast::{Expression, Operator, Statement}, lexer::Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        self.position += 1;
        token
    }

    fn expect(&mut self, expected: &Token) -> bool {
        if self.peek() == expected {
            self.advance();
            return true
        }
        return false
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];

        while !matches!(self.peek(), Token::EOF) {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement)
            }
        }
        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek() {
            Token::Let => self.parse_let(),
            _ => self.parse_expression_statement()
        }
    }

    fn parse_let(&mut self) -> Option<Statement> {
        self.advance(); // consume 'let'
        if let Token::Ident(name) = self.advance() {
            if self.expect(&Token::Assign) {
                let expr = self.parse_expression();
                self.expect(&Token::Semicolon);
                Some(Statement::Let(name.clone(), expr))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression();
        self.expect(&Token::Semicolon);
        Some(Statement::ExpressionStatement(expression))    
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expression {
        let mut expr = self.parse_factor();
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let operator = match self.advance() {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Subtract,
                _ => unreachable!(),
            };
            let right = self.parse_factor();
            expr = Expression::Operation(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn parse_factor(&mut self) -> Expression {
        let mut expr = self.parse_unary();
        while matches!(self.peek(), Token::Star | Token::Slash) {
            let operator = match self.advance() {
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                _ => unreachable!(),
            };
            let right = self.parse_unary();
            expr = Expression::Operation(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn parse_unary(&mut self) -> Expression {
        match self.peek() {
            Token::Minus => {
                self.advance();
                let right = self.parse_unary();
                Expression::Prefix(Operator::Subtract, Box::new(right))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Expression {
        match self.advance() {
            Token::Number(n) => Expression::NumberLiteral(n),
            Token::Ident(name) => Expression::Identifier(name.clone()),
            _ => Expression::NumberLiteral(0.0), // fallback
        }
    }
}

/// Scratchpad for thoughts on further work
// fn parse_sub_expressions(&mut self) -> Expression {
//     // Want to handle expressions in parentheses
//     // for example, a + b * c and (a + b) * c should have different results
//     // in the first case, the current logic holds.  we first find b * c, make that the
//     // left expression, and then can put in a second expression with the right side
//     // as a, and the left side as b * c, with the operator as +

//     // in the second case, we would want to instead first box a + b as an expression, and then
//     // use that as the left side of the second expression, with the higher order expression being
//     // box<a + b>, *, and c.

//     // a sub expression has higher priority than parsing factors, but lower than a unary (I think?)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_interpret_numbers_as_number_expressions() {
        let tokens = vec![Token::Number(24f64), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result[0], Statement::ExpressionStatement(Expression::NumberLiteral(24f64)));
    }

    #[test]
    fn it_should_interpret_number_plus_number_as_operation() {
        let tokens = vec![Token::Number(5f64), Token::Plus, Token::Number(3f64), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(
            Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Add,
                Box::new(Expression::NumberLiteral(3.0)),
            ),
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_interpret_number_times_number_as_operation() {
        let tokens = vec![Token::Number(5f64), Token::Star, Token::Number(3f64), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(
            Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Multiply,
                Box::new(Expression::NumberLiteral(3.0)),
            ),
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_obey_order_of_operations() {
        let tokens = vec![Token::Number(5f64), Token::Plus, Token::Number(2f64), Token::Star, Token::Number(3f64), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(
            Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Add,
                Box::new(
                    Expression::Operation(
                        Box::new(Expression::NumberLiteral(2f64)),
                        Operator::Multiply,
                        Box::new(Expression::NumberLiteral(3f64))
                    )
                ),
            ),
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_let_assignment() {
        let tokens = vec![Token::Let, Token::Ident(String::from("my_var")), Token::Assign, Token::Number(5f64), Token::Star, Token::Number(3f64), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::Let(String::from("my_var"), Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Multiply,
                Box::new(Expression::NumberLiteral(3.0)),
            ));
        assert_eq!(result[0], expected);
    }
    
    #[test]
    fn it_should_handle_let_assignment_to_second_variable() {
        let tokens = vec![
            Token::Let, Token::Ident(String::from("my_var")), Token::Assign, Token::Number(5f64), Token::Star, Token::Number(3f64), Token::Semicolon,   // let my_var = 5 & 3;
            Token::Let, Token::Ident(String::from("my_other_var")), Token::Assign, Token::Ident(String::from("my_var")),  Token::Semicolon,             // let my_other_var = my_var
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::Let(String::from("my_var"), Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Multiply,
                Box::new(Expression::NumberLiteral(3.0)),
            ));
        let next_expected = Statement::Let(String::from("my_other_var"), Expression::Identifier(String::from("my_var")));
        assert_eq!(result[0], expected);
        assert_eq!(result[1], next_expected);
    }
}