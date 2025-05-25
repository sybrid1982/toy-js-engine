#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    NumberLiteral(f64),
    Identifier(String),
    Prefix(Operator, Box<Expression>),
    Operation(Box<Expression>, Operator, Box<Expression>)
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(String, Expression),
    ExpressionStatement(Expression)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal
}