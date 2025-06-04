use crate::ast::{Expression, Operator, PrefixOperator, Statement, ExpressionResult};
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
                Err(error) => println!("{:#?}", error)
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

fn eval_expression(expression: Expression, env: &mut Environment) -> Result<ExpressionResult, String> {
    match expression {
        Expression::NumberLiteral(n) => Ok(ExpressionResult::Number(n)),
        Expression::Identifier(name) => match env.get(&name) {
            Some(value) => Ok(value),
            None => Err(reference_error(&name))
        }
        Expression::Boolean(is_true) => if is_true { Ok(ExpressionResult::Boolean(true)) } else { Ok(ExpressionResult::Boolean(false)) },
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
                        if let Ok(value) = &result {
                            env.set(identifier, value.clone());
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

fn handle_operation_expression(env: &mut Environment, left_hand: Box<Expression>, operator: Operator, right_hand: Box<Expression>) -> Result<ExpressionResult, String> {
    match operator {
        // currently these operators always treat both sides as boolean
        Operator::And | Operator::Or => {
            return handle_and_or_with_short_circuiting(env, &left_hand, &operator, &right_hand);
        },
        // currently these operators treat both sides as numbers, and if either side is not a number, return false
        Operator::LessThan | Operator::GreaterThan => {
            return handle_comparators(env, &left_hand, &operator, &right_hand);
        },
        // this really only makes sense for values that can be coerced to numbers, and will either return a number or NaN
        Operator::Multiply | Operator::Divide | Operator::Subtract => {
            if let Ok(left_result) = eval_expression(*left_hand, env) {
                if let Ok(right_result) = eval_expression(*right_hand, env) {
                    if let Ok(left_as_num) = left_result.coerce_to_number() {
                        if let Ok(right_as_num) = right_result.coerce_to_number() {
                            if operator == Operator::Multiply {
                                return Ok(ExpressionResult::Number(left_as_num * right_as_num));
                            } else if operator == Operator::Divide {
                                return Ok(ExpressionResult::Number(left_as_num / right_as_num));
                            } else if operator == Operator::Subtract {
                                return Ok(ExpressionResult::Number(left_as_num - right_as_num));
                            }
                        }
                    }
                }
            }
            return Err("NaN".to_string());
        },
        Operator::Add => {
            if let Ok(left_result) = eval_expression(*left_hand, env) {
                if let Ok(right_result) = eval_expression(*right_hand, env) {
                    // if either side is a string, convert both sides to string and concatenate
                    if matches!(left_result, ExpressionResult::String(_)) || matches!(right_result, ExpressionResult::String(_)) {
                        let new_string = left_result.coerce_to_string() + &right_result.coerce_to_string();
                        return Ok(ExpressionResult::String(new_string));
                    } else {
                        // otherwise convert to number and add
                        // If either side can't convert, return NaN
                        let left_num_res = left_result.coerce_to_number();
                        let right_num_res = right_result.coerce_to_number();
                        if let Ok(left_num) = left_num_res {
                            if let Ok(right_num) = right_num_res {
                                return Ok(ExpressionResult::Number(left_num + right_num))
                            }
                        }
                        return Err("NaN".to_string());
                    }
                }
            }
            return Err("Could not complete request".to_string())
        },
        Operator::Equal => {
            if let Ok(left_result) = eval_expression(*left_hand, env) {
                if let Ok(right_result) = eval_expression(*right_hand, env) {
                    // if either side is a boolean, then check other side for truthiness
                    if matches!(left_result, ExpressionResult::Boolean(_)) || matches!(right_result, ExpressionResult::Boolean(_)) {
                        return Ok(ExpressionResult::Boolean(left_result.coerce_to_bool() == right_result.coerce_to_bool()))
                    }
                    // if either side is a number, then try coercion to number
                    if matches!(left_result, ExpressionResult::Number(_)) || matches!(right_result, ExpressionResult::Number(_)) {
                        let left_num_res = left_result.coerce_to_number();
                        let right_num_res = right_result.coerce_to_number();
                        if let Ok(left_num) = left_num_res {
                            if let Ok(right_num) = right_num_res {
                                return Ok(ExpressionResult::Boolean(left_num == right_num))
                            }
                        }
                        return Err("NaN".to_string());
                    }
                    // at this point both sides must be strings, check if the strings are the same
                    return Ok(ExpressionResult::Boolean(left_result.coerce_to_string() == right_result.coerce_to_string()))
                }
            }
            return Err("Could not complete request".to_string())
        }
    }
}

fn handle_comparators(env: &mut Environment, left_hand: &Box<Expression>, operator: &Operator, right_hand: &Box<Expression>) -> Result<ExpressionResult, String> {
    let left_result = eval_expression(*left_hand.clone(), env);
    let right_result = eval_expression(*right_hand.clone(), env);
    if let Ok(left_expression_result) = left_result {
        if let Ok(right_expression_result) = right_result {
            if let Ok(left_num) = left_expression_result.coerce_to_number() {
                if let Ok(right_num) = right_expression_result.coerce_to_number() {
                    if *operator == Operator::LessThan {
                        return Ok(ExpressionResult::Boolean(left_num < right_num));
                    } else {
                        return Ok(ExpressionResult::Boolean(left_num > right_num));
                    }
                }
            }
        } 
    }
    return Ok(ExpressionResult::Boolean(false))
}

fn handle_and_or_with_short_circuiting(env: &mut Environment, left_hand: &Box<Expression>, operator: &Operator, right_hand: &Box<Expression>) -> Result<ExpressionResult, String> {
    if let Ok(left_result) = eval_expression(*left_hand.clone(), env) {
        let left_bool = left_result.coerce_to_bool();
        if *operator == Operator::And && left_bool == false {
            // short circuit, don't eval right hand side, just return false
            return Ok(ExpressionResult::Boolean(false))
        }
        if *operator == Operator::Or && left_bool == true {
            // short circuit, don't eval right hand side, just return true
            return Ok(ExpressionResult::Boolean(true))
        } else {
            if let Ok(right_result) = eval_expression(*right_hand.clone(), env) {
                let right_bool = right_result.coerce_to_bool();
                if *operator == Operator::And {
                    return Ok(ExpressionResult::Boolean(left_bool && right_bool))
                } else {
                    return Ok(ExpressionResult::Boolean(left_bool || right_bool))
                }
            }
        }
    }
    Err(syntax_error(None))
}

fn handle_prefix_expression(env: &mut Environment, operator: PrefixOperator, expression: Box<Expression>) -> Result<ExpressionResult, String> {
    let result = eval_expression(*expression.clone(), env);
    if let Ok(value) = result {
        match operator {
            PrefixOperator::Negative | PrefixOperator::Positive => {
                let sign = if operator == PrefixOperator::Negative { -1.0 } else { 1.0 };
                let coersion = value.coerce_to_number();
                if let Ok(number) = coersion {
                    return Ok(ExpressionResult::Number(sign * number))
                } else {
                    return Err("NaN".to_string())
                }
            },
            PrefixOperator::Not => {
                let bool = value.coerce_to_bool();
                Ok(ExpressionResult::Boolean(!bool))
            },
            PrefixOperator::Decrement | PrefixOperator::Increment => {
                match *expression {
                    Expression::Identifier(ident) => {
                        return modify_variable_and_return_new_value(env, operator, ident);
                    },
                    _ => {
                        // TODO: Modify code so eval_expression returns result, and this is an error return value
                        return Err(syntax_error(Some("Invalid left-hand side expression in prefix operation")));
                    }
                }
            }
        }
    } else {
        return Err(syntax_error(None));
    }
}

fn modify_variable_and_return_new_value(env: &mut Environment, operator: PrefixOperator, ident: String) -> Result<ExpressionResult, String> {
    let stored_value = env.get(&ident);
    match stored_value {
        Some(previous_value) => {
            if let Ok(previous_value_as_number) = previous_value.coerce_to_number() {
                let new = if operator == PrefixOperator::Decrement { ExpressionResult::Number(previous_value_as_number - 1.0) } else { ExpressionResult::Number(previous_value_as_number + 1.0) };
                env.set(ident.clone(), new.clone());
                return Ok(new)
            }
            return Err("NaN".to_string());
        },
        None => {
            return Err(reference_error(&ident));
        }
    }
}

fn reference_error(identifier: &str) -> String {
    format!("Uncaught ReferenceError: {} is not defined", identifier).to_string()
}

fn syntax_error(error: Option<&str>) -> String {
    match error {
        Some(error_text) => format!("Uncaught SyntaxError: {}", error_text),
        None => "Uncaught SyntaxError".to_string()
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
        assert_eq!(result, ExpressionResult::Number(15.0));
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
        assert_eq!(result, ExpressionResult::Number(8.0));
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
        assert_eq!(result, ExpressionResult::Number(10.0));
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
        assert_eq!(result, ExpressionResult::Number(-5.0));
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
        assert_eq!(result, ExpressionResult::Boolean(true));
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
        assert_eq!(result, ExpressionResult::Boolean(true));
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
        assert_eq!(result, ExpressionResult::Boolean(false));
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
        assert_eq!(result, ExpressionResult::Boolean(true));
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
        assert_eq!(result, ExpressionResult::Boolean(false));
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
        assert_eq!(result, ExpressionResult::Boolean(false));
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
        assert_eq!(result, ExpressionResult::Boolean(false));
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
        assert_eq!(result, ExpressionResult::Boolean(true));
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
        assert_eq!(result, ExpressionResult::Boolean(true));
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
        assert_eq!(result, ExpressionResult::Boolean(true));
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
        assert_eq!(result, ExpressionResult::Boolean(false));
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
        assert_eq!(env.get("x").unwrap_or(ExpressionResult::Number(-255.0)), ExpressionResult::Number(3.0));
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
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
        assert_eq!(result, ExpressionResult::Boolean(false));
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
        assert_eq!(result, ExpressionResult::Boolean(true));
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
        assert_eq!(result, ExpressionResult::Boolean(false));
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
        assert_eq!(env.get("x").unwrap(), ExpressionResult::Number(3.0));
        let result = eval_expression(expression, &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Number(2.0));
        assert_eq!(env.get("x").unwrap(), ExpressionResult::Number(2.0));

        let expression = match &statements[2] {
            Statement::ExpressionStatement(expression) => {
                expression.clone()
            },
            _ => Expression::NumberLiteral(-255.0)
        };
        let result = eval_expression(expression, &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
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
        assert_eq!(stored_value, ExpressionResult::Number(4.0));
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