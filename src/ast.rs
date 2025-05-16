#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    NumberLiteral(f64),
    Identifier(String),
    Operation(Box<Expression>, Operator, Box<Expression>)
}

impl Expression {
    pub fn is_number_literal(&self) -> bool {
        match self {
            Expression::NumberLiteral(_) => {
                true
            },
            _ => {
                false
            }
        }
    }
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
}