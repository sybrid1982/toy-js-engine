use crate::{
    ast::{Block, Expression, Operator, PrefixOperator, Statement},
    interpreter::errors::{ParserError, ParserErrorKind, SyntaxErrorKind},
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

    fn peek(&mut self) -> &Token {
        self.skip_new_lines();
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

    fn peek_keep_white_space(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn skip_new_lines(&mut self) {
        if self.peek_keep_white_space() == &Token::NewLine {
            while self.peek_keep_white_space() == &Token::NewLine {
                self.position += 1;
                if self.position > self.tokens.len() {
                    break;
                }
            }
        }
    }

    fn expect(&mut self, expected: &Token) -> bool {
        if self.peek() == expected {
            self.position += 1;
            return true;
        }
        return false;
    }

    fn expect_next_n(&mut self, expected: Vec<Token>) -> bool {
        for (index, expected_token) in expected.iter().enumerate() {
            if self.peek_at(self.position + index) != expected_token {
                return false;
            }
        }
        self.position += expected.len();
        true
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

    fn unexpected_token(&self) -> ParserError {
        let next_token = self.peek_keep_white_space();
        let error = match next_token {
            Token::Ident(name) => SyntaxErrorKind::UnexpectedIdentifier(name.clone()),
            _ => SyntaxErrorKind::UnexpectedToken(next_token.clone())
        };
        ParserError {
            kind: ParserErrorKind::SyntaxError(Some(error)),
        }
    }

    pub fn parse(&mut self) -> Vec<Result<Statement, ParserError>> {
        let mut statements: Vec<Result<Statement, ParserError>> = vec![];

        while !matches!(self.peek(), Token::EOF) && self.position < self.tokens.len() {
            statements.push(self.parse_statement())
        }
        statements
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.peek() {
            Token::Let => self.parse_let(),
            Token::Function => self.parse_function(),
            Token::Return => Ok(self.parse_return()),
            Token::If => self.parse_conditional(),
            Token::While => self.parse_while(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let(&mut self) -> Result<Statement, ParserError> {
        self.advance();
        if let Token::Ident(name) = self.advance() {
            if self.expect(&Token::Equals) {
                let expr = self.parse_expression();
                self.expect(&Token::Semicolon);
                Ok(Statement::Let(name.clone(), expr))
            } else {
                Err(self.unexpected_token())
            }
        } else {
            Err(self.unexpected_token())
        }
    }

    fn parse_function(&mut self) -> Result<Statement, ParserError> {
        self.advance();

        if let Token::Ident(name) = self.advance() {
            if self.expect(&Token::LeftParen) {
                // building arguments
                let arguments = self.parse_arguments();
                if let Ok(block) = self.parse_block() {
                    return Ok(Statement::FunctionDeclaration(name, arguments, block));
                }
            }
        }
        Err(self.unexpected_token())
    }

    fn parse_while(&mut self) -> Result<Statement, ParserError> {
        self.advance(); // clear the while token

        let condition = self.parse_paren_wrapped_expression()?;
        let block = self.parse_block()?;

        Ok(Statement::While(Box::new(Statement::ConditionalStatement(
            condition,
            block,
            Box::new(None),
        ))))
    }

    fn parse_conditional(&mut self) -> Result<Statement, ParserError> {
        let mut conditional_expression = Expression::Boolean(true);
        if self.expect(&Token::If) {
            let condition = self.parse_paren_wrapped_expression()?;
            conditional_expression = condition;
        }
        let block = self.parse_block()?;

        let mut else_conditional = None;
        while self.peek() == &Token::NewLine {
            self.advance();
        }

        if self.expect(&Token::Else) {
            else_conditional = Some(self.parse_conditional()?);
        }
        Ok(Statement::ConditionalStatement(
            conditional_expression,
            block,
            Box::new(else_conditional),
        ))
    }

    fn parse_paren_wrapped_expression(&mut self) -> Result<Expression, ParserError> {
        if self.expect(&Token::LeftParen) {
            let conditional_expression = self.parse_expression();
            if !self.expect(&Token::RightParen) {
                return Err(self.unexpected_token());
            }
            return Ok(conditional_expression);
        }

        return Err(self.unexpected_token());
    }

    fn parse_arguments(&mut self) -> Vec<Expression> {
        let mut arguments = vec![];
        while !self.expect(&Token::RightParen) {
            if self.peek() == &Token::Comma {
                self.advance();
            };
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
            self.expect(&Token::Semicolon);
            return Statement::ReturnStatement(Some(expression));
        }
        Statement::ReturnStatement(None)
    }

    fn parse_block(&mut self) -> Result<Block, ParserError> {
        if self.expect(&Token::LeftCurlyBrace) {
            // building the block
            let mut block_statements = vec![];
            while !self.expect(&Token::RightCurlyBrace) {
                if matches!(
                    self.peek(),
                    Token::Semicolon | Token::Comma | Token::NewLine
                ) {
                    self.advance();
                    continue;
                }
                let statement_result = self.parse_statement();
                match statement_result {
                    Ok(statement) => block_statements.push(statement),
                    Err(error) => return Err(error),
                }
                self.skip_new_lines();
            }
            return Ok(Block::new(block_statements));
        }
        Err(self.unexpected_token())
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        if matches!(
            self.peek(),
            Token::Semicolon | Token::Comma | Token::NewLine
        ) {
            self.advance();
        }
        if self.peek() == &Token::EOF {
            return Err(self.unexpected_token())
        }
        let expression = self.parse_expression();
        self.expect(&Token::Semicolon);
        Ok(Statement::ExpressionStatement(expression))
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_assignment()
    }

    // priority level 2
    fn parse_assignment(&mut self) -> Expression {
        let mut expr: Expression = self.parse_logical_or();

        if self.expect_next_n(vec![Token::Star, Token::Equals]) {
            expr = self.create_operator_and_assign(Operator::Multiply, &mut expr);
        } else if self.expect_next_n(vec![Token::Slash, Token::Equals]) {
            expr = self.create_operator_and_assign(Operator::Divide, &mut expr);
        } else if self.expect_next_n(vec![Token::Plus, Token::Equals]) {
            expr = self.create_operator_and_assign(Operator::Add, &mut expr);
        } else if self.expect_next_n(vec![Token::Minus, Token::Equals]) {
            expr = self.create_operator_and_assign(Operator::Subtract, &mut expr);
        } else if self.peek() == &Token::Equals && self.peek_at(self.position + 1) != &Token::Equals
        {
            self.advance();
            let right = self.parse_logical_or();
            expr = Expression::Assignment(Box::new(expr), Box::new(right));
        }
        expr
    }

    fn create_operator_and_assign(
        &mut self,
        operator: Operator,
        expr: &mut Expression,
    ) -> Expression {
        let right = self.parse_logical_or();
        Expression::Assignment(
            Box::new(expr.clone()),
            Box::new(Expression::Operation(
                Box::new(expr.clone()),
                operator,
                Box::new(right),
            )),
        )
    }

    fn parse_left_associative<LF, OF>(
        &mut self,
        lower_fn: LF,
        mut op_fn: OF,
    ) -> Expression
    where
        LF: Fn(&mut Parser) -> Expression,
        OF: Fn(&mut Parser, Expression) -> Option<Expression>,
    {
        let mut expr = lower_fn(self);
        while let Some(new_expr) = op_fn(self, expr.clone()) {
            expr = new_expr;
        }
        expr
    }

    // priority level 3
    fn parse_logical_or(&mut self) -> Expression {
        self.parse_left_associative(Parser::parse_logical_and, |parser, left| {
            if parser.peek() == &Token::Pipe && parser.peek_at(parser.position + 1) == &Token::Pipe {
                parser.advance();
                parser.advance();
                let right = parser.parse_logical_and();
                Some(Expression::Operation(Box::new(left), Operator::Or, Box::new(right)))
            } else {
                None
            }
        })    
    }

    // priority level 4
    fn parse_logical_and(&mut self) -> Expression {
        self.parse_left_associative(Parser::parse_equality, |parser, left| {
            if parser.peek() == &Token::Ampersand
                && parser.peek_at(parser.position + 1) == &Token::Ampersand
            {
                parser.advance();
                parser.advance();
                let right = parser.parse_equality();
                Some(Expression::Operation(Box::new(left), Operator::And, Box::new(right)))
            } else {
                None
            }
        })    
    }

    // Priority level 8
    fn parse_equality(&mut self) -> Expression {
        self.parse_left_associative(Parser::parse_comparator, |parser, left| {
            if parser.expect_next_n(vec![Token::Equals, Token::Equals]) {
                let right = parser.parse_comparator();
                Some(Expression::Operation(Box::new(left), Operator::Equal, Box::new(right)))
            } else if parser.expect_next_n(vec![Token::ExclamationMark, Token::Equals]) {
                let right = parser.parse_comparator();
                let operation =
                    Expression::Operation(Box::new(left), Operator::Equal, Box::new(right));
                Some(Expression::Prefix(PrefixOperator::Not, Box::new(operation)))
            } else {
                None
            }
        })
    }

    /// priority level 9
    fn parse_comparator(&mut self) -> Expression {
        self.parse_left_associative(Parser::parse_term, |parser, left| {
            if matches!(parser.peek(), Token::LeftChevron | Token::RightChevron) {
                let operator = match parser.advance() {
                    Token::LeftChevron => Operator::LessThan,
                    Token::RightChevron => Operator::GreaterThan,
                    _ => unreachable!(),
                };
                let include_equality = parser.expect(&Token::Equals);
                let right = parser.parse_term();
                let mut expr =
                    Expression::Operation(Box::new(left.clone()), operator, Box::new(right.clone()));
                if include_equality {
                    let equal_expression =
                        Expression::Operation(Box::new(left), Operator::Equal, Box::new(right));
                    expr = Expression::Operation(
                        Box::new(expr),
                        Operator::Or,
                        Box::new(equal_expression),
                    );
                }
                Some(expr)
            } else {
                None
            }
        })
    }


    /// priority level 11
    fn parse_term(&mut self) -> Expression {
        self.parse_left_associative(Parser::parse_factor, |parser, left| {
            if matches!(parser.peek(), Token::Plus | Token::Minus)
                && parser.peek_at(parser.position + 1) != &Token::Equals
            {
                let operator = match parser.advance() {
                    Token::Plus => Operator::Add,
                    Token::Minus => Operator::Subtract,
                    _ => unreachable!(),
                };
                let right = parser.parse_factor();
                Some(Expression::Operation(Box::new(left), operator, Box::new(right)))
            } else {
                None
            }
        })
    }
    

    /// priority level 12
    fn parse_factor(&mut self) -> Expression {
        self.parse_left_associative(Parser::parse_exponentiation, |parser, left| {
            if matches!(parser.peek(), Token::Star | Token::Slash | Token::Percent)
                && !matches!(
                    parser.peek_at(parser.position + 1),
                    &Token::Equals | &Token::Star
                )
            {
                let operator = match parser.advance() {
                    Token::Star => Operator::Multiply,
                    Token::Slash => Operator::Divide,
                    Token::Percent => Operator::Modulo,
                    _ => unreachable!(),
                };
                let right = parser.parse_exponentiation();
                Some(Expression::Operation(Box::new(left), operator, Box::new(right)))
            } else {
                None
            }
        })
    }

    /// priority level 13
    fn parse_exponentiation(&mut self) -> Expression {
        self.parse_left_associative(Parser::parse_unary, |parser, left| {
            if parser.expect_next_n(vec![Token::Star, Token::Star]) {
                let right = parser.parse_exponentiation();
                Some(Expression::Operation(Box::new(left), Operator::Exponentiation, Box::new(right)))
            } else {
                None
            }
        })
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
                let expr = match self.peek() {
                    Token::LeftParen => {
                        self.advance(); // get rid of the left paren
                        let arguments = self.parse_arguments();
                        Expression::Call(
                            Box::new(Expression::Identifier(name.clone())),
                            arguments,
                        )
                    }
                    _ => Expression::Identifier(name.clone()),
                };
                expr
            }
            Token::Boolean(is_true) => Expression::Boolean(is_true),
            Token::DoubleQuote => {
                let expr = match self.advance() {
                    Token::String(string) => Expression::String(string),
                    _ => Expression::NumberLiteral(0.0), // not sure how we'd get here right now, just returning 0
                };
                // if this isn't a DoubleQuote, we have an issue, but the parser just parses currently
                if self.peek() == &Token::DoubleQuote {
                    self.advance();
                }
                expr
            }
            _ => Expression::NumberLiteral(0.0), // fallback
        }
    }
}

pub fn separate_out_statements_and_parser_errors(
    statement_results: Vec<Result<Statement, ParserError>>,
) -> (Vec<Statement>, Vec<ParserError>) {
    let mut statements = vec![];
    let mut parser_errors = vec![];
    for statement_result in statement_results {
        match statement_result {
            Ok(statement) => statements.push(statement),
            Err(err) => parser_errors.push(err),
        }
    }
    (statements, parser_errors)
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
            Ok(Statement::ExpressionStatement(Expression::NumberLiteral(24.0)))
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
        assert_eq!(result[1], Ok(next_expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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
        assert_eq!(result[0], Ok(expected));
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

        let expected = Statement::ReturnStatement(Some(Expression::Operation(
            Box::new(Expression::Identifier('a'.into())),
            Operator::Add,
            Box::new(Expression::NumberLiteral(3.0)),
        )));
        assert_eq!(result[0], Ok(expected));
    }

    #[test]
    fn it_should_handle_single_argument_functions() {
        let tokens = vec![
            Token::NewLine,
            Token::Function,
            Token::Ident("add_three".into()),
            Token::LeftParen,
            Token::Ident("a".into()),
            Token::RightParen,
            Token::LeftCurlyBrace,
            Token::Return,
            Token::Ident("a".into()),
            Token::Plus,
            Token::Number(3.0),
            Token::Semicolon,
            Token::RightCurlyBrace,
            Token::NewLine,
            Token::Ident("add_three".into()),
            Token::LeftParen,
            Token::Number(4.0),
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
            Token::Ident("add".into()),
            Token::LeftParen,
            Token::Ident("a".into()),
            Token::Comma,
            Token::Ident("b".into()),
            Token::RightParen,
            Token::LeftCurlyBrace,
            Token::Return,
            Token::Ident("a".into()),
            Token::Plus,
            Token::Ident("b".into()),
            Token::Semicolon,
            Token::RightCurlyBrace,
            Token::NewLine,
            Token::Ident("add".into()),
            Token::LeftParen,
            Token::Number(4.0),
            Token::Comma,
            Token::Number(3.0),
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
            Token::RightCurlyBrace,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        let expected = Statement::ConditionalStatement(
                Expression::Operation(
                    Box::new(Expression::NumberLiteral(2.0)),
                    Operator::Equal,
                    Box::new(Expression::String("2".into()))
                ),
                Block::new(vec![Statement::ExpressionStatement(
                    Expression::NumberLiteral(6.0)
                )]),
                Box::new(None)
            );

        assert_eq!(
            result[0],
            Ok(expected)
        );
    }

    #[test]
    fn it_should_handle_star_equals() {
        let tokens = vec![
            Token::Let,
            Token::Ident("x".into()),
            Token::Equals,
            Token::Number(2.0),
            Token::Semicolon,
            Token::Ident("x".into()),
            Token::Star,
            Token::Equals,
            Token::Number(4.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.len(), 2);

        let expected = Statement::ExpressionStatement(Expression::Assignment(
                Box::new(Expression::Identifier("x".into())),
                Box::new(Expression::Operation(
                    Box::new(Expression::Identifier("x".into())),
                    Operator::Multiply,
                    Box::new(Expression::NumberLiteral(4.0))
                ))
            ));

        assert_eq!(
            result[1],
            Ok(expected)
        )
    }

    #[test]
    fn it_should_handle_slash_equals() {
        let tokens = vec![
            Token::Let,
            Token::Ident("x".into()),
            Token::Equals,
            Token::Number(2.0),
            Token::Semicolon,
            Token::Ident("x".into()),
            Token::Slash,
            Token::Equals,
            Token::Number(4.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.len(), 2);

        let expected = Statement::ExpressionStatement(Expression::Assignment(
                Box::new(Expression::Identifier("x".into())),
                Box::new(Expression::Operation(
                    Box::new(Expression::Identifier("x".into())),
                    Operator::Divide,
                    Box::new(Expression::NumberLiteral(4.0))
                ))
            ));

        assert_eq!(
            result[1],
            Ok(expected)
        )
    }

    #[test]
    fn it_should_handle_plus_equals() {
        let tokens = vec![
            Token::Let,
            Token::Ident("x".into()),
            Token::Equals,
            Token::Number(2.0),
            Token::Semicolon,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Equals,
            Token::Number(4.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.len(), 2);

        let expected = Statement::ExpressionStatement(Expression::Assignment(
                Box::new(Expression::Identifier("x".into())),
                Box::new(Expression::Operation(
                    Box::new(Expression::Identifier("x".into())),
                    Operator::Add,
                    Box::new(Expression::NumberLiteral(4.0))
                ))
            ));

        assert_eq!(
            result[1],
            Ok(expected)
        )
    }

    #[test]
    fn it_should_handle_minus_equals() {
        let tokens = vec![
            Token::Let,
            Token::Ident("x".into()),
            Token::Equals,
            Token::Number(2.0),
            Token::Semicolon,
            Token::Ident("x".into()),
            Token::Minus,
            Token::Equals,
            Token::Number(4.0),
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.len(), 2);

        let expected = Statement::ExpressionStatement(Expression::Assignment(
                Box::new(Expression::Identifier("x".into())),
                Box::new(Expression::Operation(
                    Box::new(Expression::Identifier("x".into())),
                    Operator::Subtract,
                    Box::new(Expression::NumberLiteral(4.0))
                ))
            ));

        assert_eq!(
            result[1],
            Ok(expected)
        )
    }

    #[test]
    fn it_should_handle_else() {
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
            Token::RightCurlyBrace,
            Token::NewLine,
            Token::Else,
            Token::NewLine,
            Token::LeftCurlyBrace,
            Token::Number(4.0),
            Token::RightCurlyBrace,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        let first_conditional_expression = Expression::Operation(
            Box::new(Expression::NumberLiteral(2.0)),
            Operator::Equal,
            Box::new(Expression::String("2".into())),
        );

        let first_block = Block::new(vec![Statement::ExpressionStatement(
            Expression::NumberLiteral(6.0),
        )]);

        let expected = Statement::ConditionalStatement(
                first_conditional_expression,
                first_block,
                Box::new(Some(Statement::ConditionalStatement(
                    Expression::Boolean(true),
                    Block::new(vec![Statement::ExpressionStatement(
                        Expression::NumberLiteral(4.0)
                    )]),
                    Box::new(None)
                )))
            );


        assert_eq!(
            result[0],
            Ok(expected)
        );
    }

    #[test]
    fn it_should_parse_while() {
        let tokens = vec![
            Token::While,
            Token::LeftParen,
            Token::Ident("x".into()),
            Token::LeftChevron,
            Token::Number(3.0),
            Token::RightParen,
            Token::LeftCurlyBrace,
            Token::Plus,
            Token::Plus,
            Token::Ident("x".into()),
            Token::RightCurlyBrace,
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        let conditional_expression = Expression::Operation(
            Box::new(Expression::Identifier("x".into())),
            Operator::LessThan,
            Box::new(Expression::NumberLiteral(3.0)),
        );

        let block = Block::new(vec![Statement::ExpressionStatement(Expression::Prefix(
            PrefixOperator::Increment,
            Box::new(Expression::Identifier("x".into())),
        ))]);

        let while_expression = Statement::While(Box::new(Statement::ConditionalStatement(
            conditional_expression,
            block,
            Box::new(None),
        )));

        assert_eq!(result[0], Ok(while_expression));
    }

    #[test]
    fn it_should_throw_parser_error_missing_right_paren() {
        let tokens = vec![
            Token::If,
            Token::LeftParen,
            Token::Ident("x".into()),
            Token::LeftCurlyBrace,
            Token::Ident("x".into()),
            Token::RightCurlyBrace,
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result[0], Err(ParserError { kind: ParserErrorKind::SyntaxError(Some(SyntaxErrorKind::UnexpectedToken(Token::LeftCurlyBrace)))}))
    }

    #[test]
    fn it_should_throw_parser_error_missing_right_curly_brace() {
        let tokens = vec![
            Token::If,
            Token::LeftParen,
            Token::Ident("x".into()),
            Token::RightParen,
            Token::LeftCurlyBrace,
            Token::Ident("x".into()),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result[0], Err(ParserError { kind: ParserErrorKind::SyntaxError(Some(SyntaxErrorKind::UnexpectedToken(Token::EOF)))}))
    }
}
