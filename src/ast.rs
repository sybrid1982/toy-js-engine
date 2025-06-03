#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    NumberLiteral(f64),
    Boolean(bool),
    Identifier(String),
    Prefix(PrefixOperator, Box<Expression>),
    Operation(Box<Expression>, Operator, Box<Expression>),
    // Although this allows the left side to be any expression, the interpreter will only accept Identifier(String) that have been defined
    Assignment(Box<Expression>, Box<Expression>)
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