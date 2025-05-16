pub enum Expression {
    Number(f64),
    Identifier(String),
    Binary(Box<Expression>, Operator, Box<Expression>)
}

pub enum Statement {
    Let(String, Expression),
    ExpressionStatement(Expression)
}

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}