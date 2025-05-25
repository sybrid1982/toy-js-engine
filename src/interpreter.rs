use crate::ast::{Statement, Expression, Operator};
use crate::environment::Environment;

pub fn eval_statements(statements: Vec<Statement>, env: &mut Environment) {
    for statement in statements {
        match statement {
            Statement::Let(name, expression) => {
                let val = eval_expression(expression, env);
                env.set(name, val);
            }
            Statement::ExpressionStatement(expression) => {
                println!("{}", eval_expression(expression, env))
            }
        }
    }
}

fn eval_expression(expression: Expression, env: &Environment) -> f64 {
    match expression {
        Expression::NumberLiteral(n) => n,
        Expression::Identifier(name) => env.get(&name).unwrap_or(-255.0),
        Expression::Prefix(operator, expression) => {
            let val = eval_expression(*expression, env);
            match operator {
                Operator::Subtract => {
                    return -1.0 * val;
                },
                _ => return val
            }
        },
        Expression::Operation(left_hand, operator, right_hand) => {
            let left = eval_expression(*left_hand, env);
            let right = eval_expression(*right_hand, env);
            match operator {
                Operator::Add => left + right,
                Operator::Subtract => left - right,
                Operator::Multiply => left * right,
                Operator::Divide => left / right,
                Operator::Equal => {
                    println!("syntax error?");
                    right
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_expression_should_do_math() {
        let expression = Expression::Operation(
                Box::new(Expression::NumberLiteral(5.0)),
                Operator::Multiply,
                Box::new(Expression::NumberLiteral(3.0)),
        );
        let env = Environment::new();
        assert_eq!(eval_expression(expression, &env), 15.0);
    }
}