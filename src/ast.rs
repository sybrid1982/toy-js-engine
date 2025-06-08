use std::{fmt::Display, num::ParseFloatError};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    NumberLiteral(f64),
    Boolean(bool),
    Identifier(String),
    String(String),
    Prefix(PrefixOperator, Box<Expression>),
    Operation(Box<Expression>, Operator, Box<Expression>),
    // Although this allows the left side to be any expression, the interpreter will only accept Identifier(String) that have been defined
    Assignment(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(String, Expression),
    FunctionDeclaration(String, Vec<Expression>, Block),
    ExpressionStatement(Expression),
    ReturnStatement(Option<Expression>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    LessThan,
    GreaterThan,
    And,
    Or,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrefixOperator {
    Increment,
    Decrement,
    Negative,
    Positive,
    Not
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionResult {
    Number(f64),
    String(String),
    Boolean(bool),
    Undefined
}

impl Display for ExpressionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.coerce_to_string())
    }
}

impl ExpressionResult {
    pub fn coerce_to_bool(&self) -> bool {
        match self {
            ExpressionResult::Boolean(val) => *val,
            ExpressionResult::Number(val) => *val != 0.0,
            ExpressionResult::String(val) => val.len() > 0,
            ExpressionResult::Undefined => false
        }
    }

    pub fn coerce_to_number(&self) -> Result<f64, ParseFloatError> {
        match self {
            ExpressionResult::Boolean(val) => if *val {Ok(1.0)} else {Ok(0.0)},
            ExpressionResult::Number(val) => Ok(*val),
            ExpressionResult::String(val) => val.parse::<f64>(),
            ExpressionResult::Undefined => "undefined".parse::<f64>()
        }
    }

    pub fn coerce_to_string(&self) -> String {
        match self {
            ExpressionResult::Boolean(val) => if *val { "true".to_string() } else { "false".to_string() },
            ExpressionResult::Number(val) => val.to_string(),
            ExpressionResult::String(val) => val.to_string(),
            ExpressionResult::Undefined => "undefined".to_string()
        }
    }
}

// So far, we've assumed we have to run every statement in order.  However, functions are not run immediately on declaration, and they can be called repeatedly
// and once completed a function should return back to the next statement from where it was called

// A Block is a Vec of statements and a list of blocks this block contains
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Block {
            statements,
        }
    }

    pub fn get_statements(&self) -> Vec<Statement> {
        return self.statements.clone()
    }
}