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
        Expression::Boolean(is_true) => if is_true { 1.0 } else { 0.0 },
        Expression::Prefix(operator, expression) => {
            let val = eval_expression(*expression, env);
            match operator {
                Operator::Subtract => {
                    return -1.0 * val;
                },
                Operator::Not => {
                    if val == 0.0 {
                        return 1.0 
                    } else { 
                        return 0.0
                    }
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
                // Currently treating logic operators like math that only returns 0 or 1
                Operator::LessThan => if left < right { 1.0 } else { 0.0 },
                Operator::GreaterThan => if left > right { 1.0 } else { 0.0 },
                Operator::Equal => if left == right { 1.0 } else { 0.0 },
                // In Javascript, 0 is falsy and all other numbers are truthy
                Operator::And => if left != 0.0 && right != 0.0 { 1.0 } else { 0.0 },
                Operator::Or => if left != 0.0 || right != 0.0 { 1.0 } else { 0.0 },
                _ => return left
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

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::Parser;

    #[test]
    fn line_without_semicolon() {
        let input = "3 + 5";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 8.0);
    }

    #[test]
    fn math_with_parentheses() {
        let input = "(3 + 2) * (3 - 1);";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 10.0);
    }

    #[test]
    fn negation_of_parentheses() {
        let input = "-(3+2);";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), -5.0);
    }

    #[test]
    fn testing_less_than() {
        let input = "1 < 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 1.0);
    }

    #[test]
    fn testing_less_than_with_math_true() {
        let input = "1 < 1 + 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 1.0);
    }

    #[test]
    fn testing_less_than_with_math_false() {
        let input = "1 + 2 < 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 0.0);
    }

    #[test]
    fn testing_and_true_true() {
        let input = "true && true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 1.0);
    }

    #[test]
    fn testing_and_true_false() {
        let input = "true && false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 0.0);
    }

    #[test]
    fn testing_and_false_true() {
        let input = "false && true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 0.0);
    }

    #[test]
    fn testing_and_false_false() {
        let input = "false && false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 0.0);
    }

    #[test]
    fn testing_or_true_true() {
        let input = "true || true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 1.0);
    }

    #[test]
    fn testing_or_true_false() {
        let input = "true || false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 1.0);
    }

    #[test]
    fn testing_or_false_true() {
        let input = "false || true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 1.0);
    }

    #[test]
    fn testing_or_false_false() {
        let input = "false || false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        assert_eq!(eval_expression(expression.clone(), &env), 0.0);
    }

    #[test]
    fn testing_more_complicated_logic() {
        let input = "let x = 3;  x > 2 && true";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => {
                expression.clone()
            },
            _ => Expression::NumberLiteral(-255.0)
        };
        eval_statements(statements, &mut env);
        assert_eq!(env.get("x").unwrap_or(-255.0), 3.0);
        assert_eq!(eval_expression(expression.clone(), &env), 1.0);
    }

    #[test]
    fn testing_logic_with_not_expect_false() {
        let input = "let x = 5; !(x > 3)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => {
                expression.clone()
            },
            _ => Expression::NumberLiteral(-255.0)
        };
        eval_statements(statements, &mut env);
        assert_eq!(eval_expression(expression.clone(), &env), 0.0);
    }

    #[test]
    fn testing_logic_with_not_expect_true() {
        let input = "let x = 1; !(x > 3)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => {
                expression.clone()
            },
            _ => Expression::NumberLiteral(-255.0)
        };
        eval_statements(statements, &mut env);
        assert_eq!(eval_expression(expression.clone(), &env), 1.0);
    }
}