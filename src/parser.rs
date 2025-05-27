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

    fn peek_at(&self, position: usize) -> &Token {
        self.tokens.get(position).unwrap_or(&Token::EOF)
    }

    fn remove_wrapping_parens(&mut self) {
        if self.peek_at(0) == &Token::LeftParen {
            self.tokens.remove(0);
        }
        if self.peek_at(self.tokens.len() - 1) == &Token::RightParen {
            self.tokens.pop();
        }
    }

    fn extract_subset(&mut self, start: usize, end: usize) -> Parser {
        let mut before = self.tokens[0..start].to_vec();
        let subset = self.tokens[start..end + 1].to_vec();
        let after = self.tokens[end + 1..self.tokens.len()].to_vec();

        before.append(&mut after.to_vec());
        self.tokens = before;
        Parser::new(subset)
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
        self.advance();
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
        let mut expr = self.parse_sub_expressions();
        while matches!(self.peek(), Token::Star | Token::Slash) {
            let operator = match self.advance() {
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                _ => unreachable!(),
            };
            let right = self.parse_sub_expressions();
            expr = Expression::Operation(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    
    fn parse_sub_expressions(&mut self) -> Expression {
        // Want to handle expressions in parentheses
        // for example, a + b * c and (a + b) * c should have different results
        // in the first case, the current logic holds.  we first find b * c, make that the
        // left expression, and then can put in a second expression with the right side
        // as a, and the left side as b * c, with the operator as +

        // in the second case, we would want to instead first box a + b as an expression, and then
        // use that as the left side of the second expression, with the higher order expression being
        // box<a + b>, *, and c.

        // a sub expression has higher priority than parsing factors, but lower than a unary (I think?)
        let mut sub_level = 0;
        match self.peek() {
            Token::LeftParen => {
                let left_paren_position = self.position;
                sub_level = sub_level + 1;
                let mut parser_position = left_paren_position;
                while sub_level > 0 {
                    parser_position += 1;
                    match self.peek_at(parser_position) {
                        Token::LeftParen => {
                            sub_level += 1;
                        },
                        Token::RightParen => {
                            sub_level -= 1;
                        },
                        Token::EOF => {
                            sub_level = 0;
                        },
                        _ => { }
                    }
                }
                let mut sublevel_parser= self.extract_subset(left_paren_position, parser_position);
                sublevel_parser.remove_wrapping_parens();
                return sublevel_parser.parse_expression();
            },
            _ => self.parse_unary()
        }
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
    // Want to handle expressions in parentheses
    // for example, a + b * c and (a + b) * c should have different results
    // in the first case, the current logic holds.  we first find b * c, make that the
    // left expression, and then can put in a second expression with the right side
    // as a, and the left side as b * c, with the operator as +

    // in the second case, we would want to instead first box a + b as an expression, and then
    // use that as the left side of the second expression, with the higher order expression being
    // box<a + b>, *, and c.

    // a sub expression has higher priority than parsing factors, but lower than a unary (I think?)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_interpret_numbers_as_number_expressions() {
        let tokens = vec![Token::Number(24.0), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result[0], Statement::ExpressionStatement(Expression::NumberLiteral(24.0)));
    }

    #[test]
    fn it_should_interpret_number_plus_number_as_operation() {
        let tokens = vec![Token::Number(5.0), Token::Plus, Token::Number(3.0), Token::Semicolon];
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
        let tokens = vec![Token::Number(5.0), Token::Star, Token::Number(3.0), Token::Semicolon];
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
        let tokens = vec![Token::Number(5.0), Token::Plus, Token::Number(2.0), Token::Star, Token::Number(3.0), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(
            Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Add,
                Box::new(
                    Expression::Operation(
                        Box::new(Expression::NumberLiteral(2.0)),
                        Operator::Multiply,
                        Box::new(Expression::NumberLiteral(3.0))
                    )
                ),
            ),
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_let_assignment() {
        let tokens = vec![Token::Let, Token::Ident(String::from("my_var")), Token::Assign, Token::Number(5.0), Token::Star, Token::Number(3.0), Token::Semicolon];
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
            Token::Let, Token::Ident(String::from("my_var")), Token::Assign, Token::Number(5.0), Token::Star, Token::Number(3.0), Token::Semicolon,   // let my_var = 5 & 3;
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

    #[test]
    fn it_should_handle_simple_math_wrapped_in_parentheses() {
        let tokens = vec![Token::LeftParen, Token::Number(5.0), Token::Plus, Token::Number(3.0), Token::RightParen, Token::Semicolon];
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
    fn it_should_obey_order_of_operations_with_parens() {
        let tokens = vec![Token::LeftParen, Token::Number(5.0), Token::Plus, Token::Number(2.0), Token::RightParen, Token::Star, Token::Number(3.0), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(
            Expression::Operation(
                Box::new(
                    Expression::Operation(
                        Box::new(Expression::NumberLiteral(5.0)),
                        Operator::Add,
                        Box::new(Expression::NumberLiteral(2.0)),
                    ),
                ),
                Operator::Multiply,
                Box::new(Expression::NumberLiteral(3.0))
            ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_tokens_without_semicolon() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(
            Expression::Operation(
                Box::new(Expression::NumberLiteral(1.0)),
                Operator::Add,
                Box::new(Expression::NumberLiteral(2.0)),
            ),
        );
        assert_eq!(result[0], expected);
    }
}