use crate::ast::{Expression, Operator, PrefixOperator, Statement};
use crate::environment::Environment;

pub fn eval_statements(statements: Vec<Statement>, env: &mut Environment) {
    for statement in statements {
        eval_statement(statement, env);
    }
}

fn eval_statement(statement: Statement, env: &mut Environment) {
    match statement {
        Statement::Let(name, expression) => {
            let result = eval_expression(expression, env);
            match result {
                Ok(val) => {
                    env.set(name, val);
                }
                error => println!("{:#?}", error)
            }
        }
        Statement::ExpressionStatement(expression) => {
            let result = eval_expression(expression, env);
            if let Ok(value) = result {
                println!("{}", value)
            } else if let Err(error) = result {
                println!("{:#?}", error)
            }
        }
    }
}

fn eval_expression(expression: Expression, env: &mut Environment) -> Result<f64, String> {
    match expression {
        Expression::NumberLiteral(n) => Ok(n),
        Expression::Identifier(name) => match env.get(&name) {
            Some(value) => Ok(value),
            None => Err(reference_error(&name))
        }
        Expression::Boolean(is_true) => if is_true { Ok(1.0) } else { Ok(0.0) },
        Expression::Prefix(operator, expression) => {
            handle_prefix_expression(env, operator, expression)
        },
        Expression::Operation(left_hand, operator, right_hand) => {
            handle_operation_expression(env, left_hand, operator, right_hand)
        },
        Expression::Assignment(left_hand, right_hand) => {
            match *left_hand {
                Expression::Identifier(identifier) => {
                    if env.has(identifier.clone()) {
                        let result = eval_expression(*right_hand, env);
                        if let Ok(value) = result {
                            env.set(identifier, value);
                        }
                        result
                    } else {
                        Err(reference_error(&identifier))
                    }
                },
                _ => Err("Uncaught SyntaxError: Left side of assignment must be identifier".to_string())
            }
        }
    }
}

fn handle_operation_expression(env: &mut Environment, left_hand: Box<Expression>, operator: Operator, right_hand: Box<Expression>) -> Result<f64, String> {
    let left_result = eval_expression(*left_hand, env);
    let right_result = eval_expression(*right_hand, env);
    if let Ok(left) = left_result {
        if let Ok(right) = right_result {
            match operator {
                Operator::Add => Ok(left + right),
                Operator::Subtract => Ok(left - right),
                Operator::Multiply => Ok(left * right),
                Operator::Divide => Ok(left / right),
                // Currently treating logic operators like math that only returns 0 or 1
                Operator::LessThan => if left < right { Ok(1.0) } else { Ok(0.0) },
                Operator::GreaterThan => if left > right { Ok(1.0) } else { Ok(0.0) },
                Operator::Equal => if left == right { Ok(1.0) } else { Ok(0.0) },
                // In Javascript, 0 is falsy and all other numbers are truthy
                Operator::And => if left != 0.0 && right != 0.0 { Ok(1.0) } else { Ok(0.0) },
                Operator::Or => if left != 0.0 || right != 0.0 { Ok(1.0) } else { Ok(0.0) },
            }
        } else {
            return Err("Uncaught SyntaxError".to_string());
        }
    } else {
        return Err("Uncaught SyntaxError".to_string());
    }
}

fn reference_error(identifier: &str) -> String {
    format!("Uncaught ReferenceError: {} is not defined", identifier).to_string()
}

fn handle_prefix_expression(env: &mut Environment, operator: PrefixOperator, expression: Box<Expression>) -> Result<f64, String> {
    let result = eval_expression(*expression.clone(), env);
    if let Ok(value) = result {
        match operator {
            PrefixOperator::Negative => {
                return Ok(-1.0 * value);
            },
            PrefixOperator::Positive => {
                return Ok(value);
            },
            PrefixOperator::Not => {
                if value == 0.0 {
                    return Ok(1.0) 
                } else { 
                    return Ok(0.0)
                }
            },
            PrefixOperator::Decrement | PrefixOperator::Increment => {
                match *expression {
                    Expression::Identifier(ident) => {
                        modify_variable_and_return_new_value(env, operator, ident)
                    },
                    _ => {
                        // TODO: Modify code so eval_expression returns result, and this is an error return value
                        return Err("Uncaught SyntaxError: Invalid left-hand side expression in prefix operation".to_string());
                    }
                }
            }
        }
    } else {
        return Err("Uncaught SyntaxError".to_string());
    }
}

fn modify_variable_and_return_new_value(env: &mut Environment, operator: PrefixOperator, ident: String) -> Result<f64, String> {
    let stored_value = env.get(&ident);
    match stored_value {
        Some(previous_value) => {
            let new = if operator == PrefixOperator::Decrement { previous_value - 1.0 } else { previous_value + 1.0 };
            env.set(ident.clone(), new);
            return Ok(new)
        },
        None => {
            return Err(reference_error(&ident));
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
        let mut env = Environment::new();
        let result = eval_expression(expression, &mut env).unwrap();
        assert_eq!(result, 15.0);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::Parser;

    fn eval_statement_at_index(statements: &Vec<Statement>, env: &mut Environment, index: usize) {
        let statement = match &statements[index] {
            Statement::Let(identifier, expression) => {
                Statement::Let(identifier.to_string(), expression.clone())
            },
            Statement::ExpressionStatement(expression) => {
                Statement::ExpressionStatement(expression.clone())
            }
        };
        eval_statement(statement, env);
    }

    #[test]
    fn line_without_semicolon() {
        let input = "3 + 5";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn math_with_parentheses() {
        let input = "(3 + 2) * (3 - 1);";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn negation_of_parentheses() {
        let input = "-(3+2);";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn testing_less_than() {
        let input = "1 < 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn testing_less_than_with_math_true() {
        let input = "1 < 1 + 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn testing_less_than_with_math_false() {
        let input = "1 + 2 < 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
                let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn testing_and_true_true() {
        let input = "true && true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
                let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn testing_and_true_false() {
        let input = "true && false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
                let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn testing_and_false_true() {
        let input = "false && true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
                let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn testing_and_false_false() {
        let input = "false && false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
                let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn testing_or_true_true() {
        let input = "true || true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn testing_or_true_false() {
        let input = "true || false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn testing_or_false_true() {
        let input = "false || true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn testing_or_false_false() {
        let input = "false || false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression
            },
            _ => &Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 0.0);
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
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 1.0);
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
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 0.0);
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
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn testing_logic_with_not_not() {
        let input = "let x = 1; !!(x > 3)";
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
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn testing_logic_with_decrement_prefix() {
        let input = "let x = 3; --x; x == 3;";
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
        eval_statement_at_index(&statements, &mut env, 0);
        assert_eq!(env.get("x").unwrap(), 3.0);
        let result = eval_expression(expression, &mut env).unwrap();
        assert_eq!(result, 2.0);
        assert_eq!(env.get("x").unwrap(), 2.0);

        let expression = match &statements[2] {
            Statement::ExpressionStatement(expression) => {
                expression.clone()
            },
            _ => Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression, &mut env).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn testing_reassignment() {
        let input = "let x = 3; x = 4;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();

        eval_statements(statements, &mut env);

        let stored_value = env.get("x").unwrap();
        assert_eq!(stored_value, 4.0);
    }

    #[test]
    fn testing_reference_error() {
        let input = "x = 6;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => {
                expression.clone()
            },
            _ => Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression, &mut env);
        assert!(result.is_err(), "{}", reference_error("x"));
    }
}