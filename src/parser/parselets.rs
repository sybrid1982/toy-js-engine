use crate::{ast::{Expression, Statement}, interpreter::errors::ParserError, lexer::Token, parser::Parser};
use std::{collections::HashMap, rc::Rc};

pub trait StatementParselet {
    fn parse(
        &self,
        parser: &mut Parser,
    ) -> Result<Statement, ParserError>;
}

struct LetParselet;
impl StatementParselet for LetParselet {
    fn parse(
        &self,
        parser: &mut Parser
    ) -> Result<Statement, ParserError> {
        parser.advance();
        if let Token::Ident(name) = parser.advance() {
            if parser.expect(&Token::Equals) {
                let expr = parser.parse_expression();
                parser.expect(&Token::Semicolon);
                Ok(Statement::Let(name.clone(), expr))
            } else {
                Err(parser.unexpected_token())
            }
        } else {
            Err(parser.unexpected_token())
        }
    }
}

struct FunctionParselet;
impl StatementParselet for FunctionParselet {
    fn parse(
        &self,
        parser: &mut Parser
    ) -> Result<Statement, ParserError> {
        parser.advance();

        if let Token::Ident(name) = parser.advance() {
            if parser.expect(&Token::LeftParen) {
                // building arguments
                let arguments = parser.parse_arguments();
                if let Ok(block) = parser.parse_block() {
                    return Ok(Statement::FunctionDeclaration(name, arguments, block));
                }
            }
        }
        Err(parser.unexpected_token())
    }
}

struct ReturnParselet;
impl StatementParselet for ReturnParselet {
    fn parse(&self, parser: &mut Parser) -> Result<Statement, ParserError> {
        parser.advance(); // get rid of that return token
        if !matches!(parser.peek(), Token::Semicolon | Token::NewLine) {
            let expression = parser.parse_expression();
            parser.expect(&Token::Semicolon);
            return Ok(Statement::ReturnStatement(Some(expression)));
        }
        Ok(Statement::ReturnStatement(None))
    }
}

struct IfParselet;
impl StatementParselet for IfParselet {
    fn parse(&self, parser: &mut Parser) -> Result<Statement, ParserError> {
        let mut conditional_expression = Expression::Boolean(true);
        if parser.expect(&Token::If) {
            let condition = parser.parse_paren_wrapped_expression()?;
            conditional_expression = condition;
        }
        
        let block = if parser.peek() == &Token::LeftCurlyBrace { 
            parser.parse_block()?
        } else {
            parser.parse_statement()?.into_block()
        };

        let mut else_conditional = None;
        while parser.peek() == &Token::NewLine {
            parser.advance();
        }

        if parser.expect(&Token::Else) {
            else_conditional = Some(self.parse(parser)?);
        }
        Ok(Statement::ConditionalStatement(
            conditional_expression,
            block,
            Box::new(else_conditional),
        ))
    }
}

struct WhileParselet;
impl StatementParselet for WhileParselet {
    fn parse(&self, parser: &mut Parser) -> Result<Statement, ParserError> {
        parser.advance(); // clear the while token

        let condition = parser.parse_paren_wrapped_expression()?;
        let block = parser.parse_block()?;

        Ok(Statement::While(Box::new(Statement::ConditionalStatement(
            condition,
            block,
            Box::new(None),
        ))))
    }
}

struct StatementExpressionParselet;
impl StatementParselet for StatementExpressionParselet {
    fn parse(
        &self,
        parser: &mut Parser
    ) -> Result<Statement, ParserError> {
        if matches!(
            parser.peek(),
            Token::Semicolon | Token::Comma | Token::NewLine
        ) {
            parser.advance();
        }
        if parser.peek() == &Token::EOF {
            return Err(parser.unexpected_token())
        }
        let expression = parser.parse_expression();
        parser.expect(&Token::Semicolon);
        Ok(Statement::ExpressionStatement(expression))
    }
}

/// Factory for statement parselets, dispatching based on token type.
/// 
/// `parselets` is a HashMap that maps tokens to their corresponding statement parselet,
/// enabling dynamic dispatch of parsing logic for different statement types.
/// 
/// `default` is the fallback parselet used when no specific parselet is registered for a token.
pub struct ParseletFactory {
    /// Maps tokens to their corresponding statement parselet for dispatch.
    parselets: HashMap<Token, Rc<dyn StatementParselet>>,
    /// Fallback parselet used when no specific parselet is found for a token.
    default: Rc<dyn StatementParselet>
}

impl ParseletFactory {
    pub fn new() -> Self {
        ParseletFactory { 
            parselets: Self::register_statement_parselets(),
            default: Rc::new(StatementExpressionParselet)
        }
    }

    fn register_statement_parselets() -> HashMap<Token, Rc<dyn StatementParselet>> {
        let mut map: HashMap<Token, Rc<dyn StatementParselet>> = HashMap::new();
        map.insert(Token::Let, Rc::new(LetParselet));
        map.insert(Token::Function, Rc::new(FunctionParselet));
        map.insert(Token::Return, Rc::new(ReturnParselet));
        map.insert(Token::If, Rc::new(IfParselet));
        map.insert(Token::While, Rc::new(WhileParselet));
        map
    }

    pub fn get_parselet(&self, token: &Token) -> Rc<dyn StatementParselet>{
        let parselet = self.parselets.get(token);
        match parselet {
            Some(p) => p.clone(),
            None => self.default.clone()
        }
    }
}

