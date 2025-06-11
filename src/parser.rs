use crate::{
    ast::{Block, Expression, Operator, PrefixOperator, Statement},
    lexer::Token,
};

/// Operator precedence (taken from https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_precedence#precedence_and_associativity)
/// This is per specification, not actually how the engine works.  Many of the operators listed here are unimplemented.
/// This precedence is generally held, except in certain cases (short circuiting).  For instance, a || (b + c) will not evaluate the b + c side if a is true.
/// 18: grouping
/// 17: access and call
/// 16: new (without argument list)
/// 15: postfix operators (EX: x++, y--)
/// 14: prefix operators (EX: ++x, --y, !x, +x, -x, typeof x, void x, delete x, await x)
/// 13: exponentiation (right to left associativity, x ** y)
/// 12: multiplicative operators
/// 11: additive operators
/// 10: bitwise shift
/// 9: relational operators (<, <=, >, >=)
/// 8: equality operators (==, !=, ===, !==)
/// 7: bitwise AND
/// 6: bitwise XOR (x ^ y)
/// 5: bitwise OR
/// 4: logical AND
/// 3: logical OR and nullish coalescing (ie x ?? y)
/// 2: assignment operations (=, *=, -=, ??=, etc), ternary operator, arrow, yield, spread
/// 1: comma

pub struct Parser {
    pub tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
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
            return true;
        }
        return false;
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

    // Instead of returning options, we should be returning results, because a return of None
    // from these top level functions (parse lets, parse function, parse conditional, parse return)
    // represent an error in the syntax - instead of the expected tokens, different tokens have appeared instead

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek() {
            Token::Let => self.parse_let(),
            Token::Function => self.parse_function(),
            Token::Return => Some(self.parse_return()),
            Token::NewLine => {
                self.advance();
                return None
            },
            Token::If => self.parse_conditional(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let(&mut self) -> Option<Statement> {
        self.advance();
        if let Token::Ident(name) = self.advance() {
            if self.expect(&Token::Equals) {
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

    fn parse_function(&mut self) -> Option<Statement> {
        self.advance();

        if let Token::Ident(name) = self.advance() {
            if self.expect(&Token::LeftParen) {
                // building arguments
                let arguments = self.parse_arguments();
                if let Some(block) = self.parse_block() {
                    return Some(Statement::FunctionDeclaration(name, arguments, block));
                }
            }
        }
        None
    }

    fn parse_conditional(&mut self) -> Option<Statement> {
        self.advance();

        if self.expect(&Token::LeftParen) {
            let expression = self.parse_expression();
            if self.expect(&Token::RightParen) {
                let block = self.parse_block();
                return Some(Statement::ConditionalStatement(expression, block?))
            }
        }
        None
    }

    fn parse_arguments(&mut self) -> Vec<Expression> {
        let mut arguments = vec![];
        while !self.expect(&Token::RightParen) {
            if self.peek() == &Token::Comma { self.advance(); };
            let argument = self.parse_expression();
            // When defining a function's parameters, these should only be Identifiers
            // But as we are reusing this when we call a function, this is fine
            // The interpreter is left to decide if a mistake has been made
            arguments.push(argument)
        }
        arguments
    }
    
    fn parse_return(&mut self) -> Statement {
        self.advance(); // get rid of that return token
        if !matches!(self.peek(), Token::Semicolon | Token::NewLine) {
            let expression = self.parse_expression();
            return Statement::ReturnStatement(Some(expression))
        }
        Statement::ReturnStatement(None)
    }

    fn parse_block(&mut self) -> Option<Block> {
        if self.expect(&Token::LeftCurlyBrace) {
            // building the block
            let mut function_statements = vec![];
            while !self.expect(&Token::RightCurlyBrace) {
                if let Some(statement) = self.parse_statement() {
                    function_statements.push(statement)
                }
            }
            return Some(Block::new(function_statements));
        }
        None
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        if matches!(self.peek(), Token::Semicolon | Token::RightCurlyBrace | Token::Comma | Token::NewLine) {
            self.advance();
            return None;
        }
        let expression = self.parse_expression();
        self.expect(&Token::Semicolon);
        Some(Statement::ExpressionStatement(expression))
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_assignment()
    }

    // priority level 2
    fn parse_assignment(&mut self) -> Expression {
        let mut expr = self.parse_logical_or();
        if self.peek() == &Token::Equals && self.peek_at(self.position + 1) != &Token::Equals {
            self.advance();
            let right = self.parse_logical_or();
            expr = Expression::Assignment(Box::new(expr), Box::new(right));
        }
        expr
    }

    // priority level 3
    fn parse_logical_or(&mut self) -> Expression {
        let mut expr = self.parse_logical_and();
        while self.peek() == &Token::Pipe && self.peek_at(self.position + 1) == &Token::Pipe {
            self.advance();
            self.advance(); // consume both pipes
            let right = self.parse_logical_and();
            expr = Expression::Operation(Box::new(expr), Operator::Or, Box::new(right));
        }
        expr
    }

    // priority level 4
    fn parse_logical_and(&mut self) -> Expression {
        let mut expr = self.parse_equality();
        while self.peek() == &Token::Ampersand
            && self.peek_at(self.position + 1) == &Token::Ampersand
        {
            self.advance();
            self.advance(); // consume both ampersands
            let right = self.parse_equality();
            expr = Expression::Operation(Box::new(expr), Operator::And, Box::new(right));
        }
        expr
    }

    // Priority level 8
    fn parse_equality(&mut self) -> Expression {
        let mut expr = self.parse_comparator();
        if self.peek() == &Token::Equals {
            match self.peek_at(self.position + 1) {
                Token::Equals => {
                    self.advance();
                    self.advance(); // consume both ampersands
                    let right = self.parse_comparator();
                    expr = Expression::Operation(Box::new(expr), Operator::Equal, Box::new(right));
                }
                _ => {}
            }
        }
        expr
    }

    /// priority level 9
    fn parse_comparator(&mut self) -> Expression {
        let mut expr = self.parse_term();
        while matches!(self.peek(), Token::LeftChevron | Token::RightChevron) {
            let operator = match self.advance() {
                Token::LeftChevron => Operator::LessThan,
                Token::RightChevron => Operator::GreaterThan,
                _ => unreachable!(),
            };
            let right = self.parse_term();
            expr = Expression::Operation(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    /// priority level 11
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

    /// priority level 12
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

    /// priority level 14
    fn parse_unary(&mut self) -> Expression {
        match self.peek() {
            Token::Minus | Token::Plus => {
                let token = self.advance();
                if self.peek() == &token {
                    self.advance();
                    let right = self.parse_unary();
                    match token {
                        Token::Minus => {
                            return Expression::Prefix(PrefixOperator::Decrement, Box::new(right))
                        }
                        Token::Plus => {
                            return Expression::Prefix(PrefixOperator::Increment, Box::new(right))
                        }
                        _ => unreachable!(),
                    }
                }
                let right = self.parse_unary();
                let prefix = match token {
                    Token::Minus => PrefixOperator::Negative,
                    Token::Plus => PrefixOperator::Positive,
                    _ => unreachable!(),
                };
                Expression::Prefix(prefix, Box::new(right))
            }
            Token::ExclamationMark => {
                self.advance();
                let right = self.parse_unary();
                Expression::Prefix(PrefixOperator::Not, Box::new(right))
            }
            _ => self.parse_sub_expression(),
        }
    }

    /// priority level 18
    fn parse_sub_expression(&mut self) -> Expression {
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
                        }
                        Token::RightParen => {
                            sub_level -= 1;
                        }
                        Token::EOF => {
                            sub_level = 0;
                        }
                        _ => {}
                    }
                }
                let mut sublevel_parser = self.extract_subset(left_paren_position, parser_position);
                sublevel_parser.remove_wrapping_parens();
                return sublevel_parser.parse_expression();
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Expression {
        match self.advance() {
            Token::Number(n) => Expression::NumberLiteral(n),
            Token::Ident(name) => {
                return match self.peek() {
                    Token::LeftParen => {
                        self.advance();     // get rid of the left paren
                        let arguments = self.parse_arguments();
                        return Expression::Call(Box::new(Expression::Identifier(name.clone())), arguments)
                    },
                    _ => Expression::Identifier(name.clone())
                }
            },
            Token::Boolean(is_true) => Expression::Boolean(is_true),
            Token::DoubleQuote => {
                let result = match self.advance() {
                    Token::String(string) => Expression::String(string),
                    _ => Expression::NumberLiteral(0.0), // not sure how we'd get here right now, just returning 0
                };
                // if this isn't a DoubleQuote, we have an issue, but the parser just parses currently
                if self.peek() == &Token::DoubleQuote {
                    self.advance();
                }
                return result;
            },
            _ => Expression::NumberLiteral(0.0), // fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_interpret_numbers_as_number_expressions() {
        let tokens = vec![Token::Number(24.0), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(
            result[0],
            Statement::ExpressionStatement(Expression::NumberLiteral(24.0))
        );
    }

    #[test]
    fn it_should_interpret_number_plus_number_as_operation() {
        let tokens = vec![
            Token::Number(5.0),
            Token::Plus,
            Token::Number(3.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(5.0)),
            Operator::Add,
            Box::new(Expression::NumberLiteral(3.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_interpret_number_times_number_as_operation() {
        let tokens = vec![
            Token::Number(5.0),
            Token::Star,
            Token::Number(3.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(5.0)),
            Operator::Multiply,
            Box::new(Expression::NumberLiteral(3.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_obey_order_of_operations() {
        let tokens = vec![
            Token::Number(5.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Star,
            Token::Number(3.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(5.0)),
            Operator::Add,
            Box::new(Expression::Operation(
                Box::new(Expression::NumberLiteral(2.0)),
                Operator::Multiply,
                Box::new(Expression::NumberLiteral(3.0)),
            )),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_let_assignment() {
        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("my_var")),
            Token::Equals,
            Token::Number(5.0),
            Token::Star,
            Token::Number(3.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::Let(
            String::from("my_var"),
            Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Multiply,
                Box::new(Expression::NumberLiteral(3.0)),
            ),
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_let_assignment_to_second_variable() {
        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("my_var")),
            Token::Equals,
            Token::Number(5.0),
            Token::Star,
            Token::Number(3.0),
            Token::Semicolon, // let my_var = 5 & 3;
            Token::Let,
            Token::Ident(String::from("my_other_var")),
            Token::Equals,
            Token::Ident(String::from("my_var")),
            Token::Semicolon, // let my_other_var = my_var
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::Let(
            String::from("my_var"),
            Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Multiply,
                Box::new(Expression::NumberLiteral(3.0)),
            ),
        );
        let next_expected = Statement::Let(
            String::from("my_other_var"),
            Expression::Identifier(String::from("my_var")),
        );
        assert_eq!(result[0], expected);
        assert_eq!(result[1], next_expected);
    }

    #[test]
    fn it_should_handle_simple_math_wrapped_in_parentheses() {
        let tokens = vec![
            Token::LeftParen,
            Token::Number(5.0),
            Token::Plus,
            Token::Number(3.0),
            Token::RightParen,
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(5.0)),
            Operator::Add,
            Box::new(Expression::NumberLiteral(3.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_obey_order_of_operations_with_parens() {
        let tokens = vec![
            Token::LeftParen,
            Token::Number(5.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RightParen,
            Token::Star,
            Token::Number(3.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Add,
                Box::new(Expression::NumberLiteral(2.0)),
            )),
            Operator::Multiply,
            Box::new(Expression::NumberLiteral(3.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_negate_with_parens() {
        let tokens = vec![
            Token::Minus,
            Token::LeftParen,
            Token::Number(5.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RightParen,
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Prefix(
            PrefixOperator::Negative,
            Box::new(Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Add,
                Box::new(Expression::NumberLiteral(2.0)),
            )),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_tokens_without_semicolon() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(1.0)),
            Operator::Add,
            Box::new(Expression::NumberLiteral(2.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_less_than() {
        let tokens = vec![
            Token::Number(1.0),
            Token::LeftChevron,
            Token::Number(2.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(1.0)),
            Operator::LessThan,
            Box::new(Expression::NumberLiteral(2.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_greater_than() {
        let tokens = vec![
            Token::Number(1.0),
            Token::RightChevron,
            Token::Number(2.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(1.0)),
            Operator::GreaterThan,
            Box::new(Expression::NumberLiteral(2.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_double_equals() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Equals,
            Token::Equals,
            Token::Number(2.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(1.0)),
            Operator::Equal,
            Box::new(Expression::NumberLiteral(2.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_double_ampersand() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Ampersand,
            Token::Ampersand,
            Token::Number(2.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(1.0)),
            Operator::And,
            Box::new(Expression::NumberLiteral(2.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_double_pipe() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Pipe,
            Token::Pipe,
            Token::Number(2.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Operation(
            Box::new(Expression::NumberLiteral(1.0)),
            Operator::Or,
            Box::new(Expression::NumberLiteral(2.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_exclamation_mark_as_prefix() {
        let tokens = vec![Token::ExclamationMark, Token::Number(0.0)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Prefix(
            PrefixOperator::Not,
            Box::new(Expression::NumberLiteral(0.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_double_exclamation_mark_as_prefix() {
        let tokens = vec![
            Token::ExclamationMark,
            Token::ExclamationMark,
            Token::Number(0.0),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Prefix(
            PrefixOperator::Not,
            Box::new(Expression::Prefix(
                PrefixOperator::Not,
                Box::new(Expression::NumberLiteral(0.0)),
            )),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_double_plus_as_prefix() {
        let tokens = vec![Token::Plus, Token::Plus, Token::Number(0.0)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Prefix(
            PrefixOperator::Increment,
            Box::new(Expression::NumberLiteral(0.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_double_minus_as_prefix() {
        let tokens = vec![Token::Minus, Token::Minus, Token::Number(4.0)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Prefix(
            PrefixOperator::Decrement,
            Box::new(Expression::NumberLiteral(4.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_assignment() {
        let tokens = vec![
            Token::Ident("x".to_string()),
            Token::Equals,
            Token::Number(4.0),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected = Statement::ExpressionStatement(Expression::Assignment(
            Box::new(Expression::Identifier("x".to_string())),
            Box::new(Expression::NumberLiteral(4.0)),
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_parse_out_a_function() {
        let tokens = vec![
            Token::Function,
            Token::Ident("fake_function".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurlyBrace,
            Token::Number(5.0),
            Token::Plus,
            Token::Number(5.0),
            Token::RightCurlyBrace,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        let expected_block_statements =
            vec![Statement::ExpressionStatement(Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Add,
                Box::new(Expression::NumberLiteral(5.0)),
            ))];
        let expected = Statement::FunctionDeclaration(
            "fake_function".to_string(),
            vec![],
            Block::new(expected_block_statements),
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_parse_out_a_function_call() {
        let tokens = vec![
            Token::Ident("fake_function".to_string()),
            Token::LeftParen,
            Token::RightParen,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        let expected = Statement::ExpressionStatement(Expression::Call(
            Box::new(Expression::Identifier("fake_function".to_string())),
            vec![],
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_parse_out_a_function_call_with_argument() {
        let tokens = vec![
            Token::Ident("fake_function".to_string()),
            Token::LeftParen,
            Token::Number(3.0),
            Token::RightParen,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        let expected = Statement::ExpressionStatement(Expression::Call(
            Box::new(Expression::Identifier("fake_function".to_string())),
            vec![Expression::NumberLiteral(3.0)],
        ));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_parse_out_a_return_with_expression() {
        let tokens = vec![
            Token::Return,
            Token::Ident("a".into()),
            Token::Plus,
            Token::Number(3.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        let expected = Statement::ReturnStatement(
            Some(Expression::Operation(
                Box::new(Expression::Identifier('a'.into())),
                Operator::Add,
                Box::new(Expression::NumberLiteral(3.0))
            ))
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn it_should_handle_single_argument_functions() {
        let tokens = vec![
                Token::NewLine,
                Token::Function,
                Token::Ident(
                    "add_three".into(),
                ),
                Token::LeftParen,
                Token::Ident(
                    "a".into(),
                ),
                Token::RightParen,
                Token::LeftCurlyBrace,
                Token::Return,
                Token::Ident(
                    "a".into(),
                ),
                Token::Plus,
                Token::Number(
                    3.0,
                ),
                Token::Semicolon,
                Token::RightCurlyBrace,
                Token::NewLine,
                Token::Ident(
                    "add_three".into(),
                ),
                Token::LeftParen,
                Token::Number(
                    4.0,
                ),
                Token::RightParen,
                Token::Semicolon,
                Token::NewLine,
                Token::EOF,
        ];
        let mut parser = Parser::new(tokens);

        let result = parser.parse();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn it_should_handle_two_arguments() {
        let tokens = vec![
                Token::NewLine,
                Token::Function,
                Token::Ident(
                    "add".into(),
                ),
                Token::LeftParen,
                Token::Ident(
                    "a".into(),
                ),
                Token::Comma,
                Token::Ident(
                    "b".into(),
                ),
                Token::RightParen,
                Token::LeftCurlyBrace,
                Token::Return,
                Token::Ident(
                    "a".into(),
                ),
                Token::Plus,
                Token::Ident(
                    "b".into(),
                ),
                Token::Semicolon,
                Token::RightCurlyBrace,
                Token::NewLine,
                Token::Ident(
                    "add".into(),
                ),
                Token::LeftParen,
                Token::Number(
                    4.0,
                ),
                Token::Comma,
                Token::Number(
                    3.0
                ),
                Token::RightParen,
                Token::Semicolon,
                Token::NewLine,
                Token::EOF,
        ];
        let mut parser = Parser::new(tokens);

        let result = parser.parse();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn it_should_handle_simple_if_statement() {
        let tokens = vec![
            Token::If,
            Token::LeftParen,
            Token::Number(2.0),
            Token::Equals,
            Token::Equals,
            Token::DoubleQuote,
            Token::String("2".into()),
            Token::DoubleQuote,
            Token::RightParen,
            Token::LeftCurlyBrace,
            Token::Number(6.0),
            Token::Semicolon,
            Token::RightCurlyBrace
        ];
        let mut parser = Parser::new(tokens);

        let result = parser.parse();
        assert_eq!(result[0], Statement::ConditionalStatement(
            Expression::Operation(
                Box::new(Expression::NumberLiteral(2.0)),
                Operator::Equal,
                Box::new(Expression::String("2".into()))
            ),
            Block::new(vec![Statement::ExpressionStatement(Expression::NumberLiteral(6.0))])
        ));
    }
}
