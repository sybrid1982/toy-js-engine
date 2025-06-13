use crate::ast::{Expression, ExpressionResult, Operator, PrefixOperator, Statement};
use crate::environment::Environment;
use crate::interpreter::errors::{InterpreterError, InterpreterErrorKind, SyntaxErrorKind};
use crate::function::Function;

pub fn eval_statements(statements: Vec<Statement>, env: &mut Environment) -> ExpressionResult {
    for statement in statements {
        let result = eval_statement(statement, env);
        if let Some(value) = &result {
            return value.clone();
        }
    }
    ExpressionResult::Undefined
}

pub fn eval_statement(statement: Statement, env: &mut Environment) -> Option<ExpressionResult> {
    match statement {
        Statement::Let(identifier, expression) => {
            let result = eval_expression(expression, env);
            match result {
                Ok(val) => {
                    env.set_variable(identifier, val);
                }
                Err(error) => {
                    println!("{:#?}", error);
                }
            }
            return None;
        }
        Statement::ExpressionStatement(expression) => {
            let result = eval_expression(expression, env);
            if let Ok(value) = result {
                println!("{}", value)
            } else if let Err(error) = result {
                println!("{:#?}", error)
            }
            return None;
        }
        Statement::FunctionDeclaration(identifier, arguments, block) => {
            let function = Function::new(arguments, block);
            env.set_function(identifier, function);
            return None;
        }
        Statement::ReturnStatement(return_expression) => {
            if let Some(expression) = return_expression {
                let result = eval_expression(expression, env);
                if let Ok(value) = result {
                    return Some(value);
                }
            }
            // TODO: should be bubbling the error up or something, instead of returning none
            // so adjust eval_statement to return Result<Option<ExpressionResult>>?45
            Some(ExpressionResult::Undefined)
        }
        Statement::ConditionalStatement(condition, block) => {
            if let Ok(expression_result) = eval_expression(condition, env) {
                if expression_result.coerce_to_bool() {
                    let mut block_env = env.clone();
                    let block_result = block.execute_block(&mut block_env);
                }
            }
            return None;
        }
    }
}

pub fn eval_expression(
    expression: Expression,
    env: &mut Environment,
) -> Result<ExpressionResult, String> {
    match expression {
        Expression::NumberLiteral(n) => Ok(ExpressionResult::Number(n)),
        Expression::Identifier(identifier) => match env.get_variable(&identifier) {
                        Some(value) => Ok(value),
                        None => Err(InterpreterError { kind: InterpreterErrorKind::ReferenceError(identifier.clone())}.to_string()),
            },
        Expression::Boolean(is_true) => {
                if is_true {
                    Ok(ExpressionResult::Boolean(true))
                } else {
                    Ok(ExpressionResult::Boolean(false))
                }
            }
        Expression::String(string) => Ok(ExpressionResult::String(string)),
        Expression::Prefix(operator, expression) => {
                handle_prefix_expression(env, operator, expression)
            }
        Expression::Operation(left_hand, operator, right_hand) => {
                handle_operation_expression(env, left_hand, operator, right_hand)
            }
        Expression::Assignment(left_hand, right_hand) => match *left_hand {
                Expression::Identifier(identifier) => {
                    if env.has_variable(identifier.clone()) {
                        let result = eval_expression(*right_hand, env);
                        if let Ok(value) = &result {
                            env.set_variable(identifier, value.clone());
                        }
                        result
                    } else {
                        Err(InterpreterError { kind: InterpreterErrorKind::ReferenceError(identifier.clone())}.to_string())
                    }
                }
                _ => {
                    Err(InterpreterError { kind: InterpreterErrorKind::SyntaxError(Some(SyntaxErrorKind::LeftSideAssignmentMustBeIdentifier))}.to_string())
                }
            },
        Expression::Call(callee, arguments) => {
            match *callee {
                Expression::Identifier(identifier) => {
                    if let Some(function) = env.get_function(&identifier) {
                        return function.call(arguments, env);
                    }
                    return Err(format!("Function {} not defined", identifier));
                },
                _ => {
                    return Err("Either not implemented or not valid".into());
                }
            }
        }
    }
}

fn handle_operation_expression(
    env: &mut Environment,
    left_hand: Box<Expression>,
    operator: Operator,
    right_hand: Box<Expression>,
) -> Result<ExpressionResult, String> {
    match operator {
        // currently these operators always treat both sides as boolean
        Operator::And | Operator::Or => {
            return handle_and_or_with_short_circuiting(env, &left_hand, &operator, &right_hand);
        }
        // currently these operators treat both sides as numbers, and if either side is not a number, return false
        Operator::LessThan | Operator::GreaterThan => {
            return handle_comparators(env, &left_hand, &operator, &right_hand);
        }
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
            return Err(InterpreterError { kind: InterpreterErrorKind::NaN }.to_string());
        }
        Operator::Add => {
            if let Ok(left_result) = eval_expression(*left_hand, env) {
                if let Ok(right_result) = eval_expression(*right_hand, env) {
                    // if either side is a string, convert both sides to string and concatenate
                    if matches!(left_result, ExpressionResult::String(_))
                        || matches!(right_result, ExpressionResult::String(_))
                    {
                        let new_string =
                            left_result.coerce_to_string() + &right_result.coerce_to_string();
                        return Ok(ExpressionResult::String(new_string));
                    } else {
                        // otherwise convert to number and add
                        // If either side can't convert, return NaN
                        let left_num_res = left_result.coerce_to_number();
                        let right_num_res = right_result.coerce_to_number();
                        if let Ok(left_num) = left_num_res {
                            if let Ok(right_num) = right_num_res {
                                return Ok(ExpressionResult::Number(left_num + right_num));
                            }
                        }
                        return Err(InterpreterError { kind: InterpreterErrorKind::NaN }.to_string());
                    }
                }
            }
            return Err("Could not complete request".to_string());
        }
        Operator::Equal => {
            if let Ok(left_result) = eval_expression(*left_hand, env) {
                if let Ok(right_result) = eval_expression(*right_hand, env) {
                    // if either side is a boolean, then check other side for truthiness
                    if matches!(left_result, ExpressionResult::Boolean(_))
                        || matches!(right_result, ExpressionResult::Boolean(_))
                    {
                        return Ok(ExpressionResult::Boolean(
                            left_result.coerce_to_bool() == right_result.coerce_to_bool(),
                        ));
                    }
                    // if either side is a number, then try coercion to number
                    if matches!(left_result, ExpressionResult::Number(_))
                        || matches!(right_result, ExpressionResult::Number(_))
                    {
                        let left_num_res = left_result.coerce_to_number();
                        let right_num_res = right_result.coerce_to_number();
                        if let Ok(left_num) = left_num_res {
                            if let Ok(right_num) = right_num_res {
                                return Ok(ExpressionResult::Boolean(left_num == right_num));
                            }
                        }
                        return Err(InterpreterError { kind: InterpreterErrorKind::NaN }.to_string());
                    }
                    // at this point both sides must be strings, check if the strings are the same
                    return Ok(ExpressionResult::Boolean(
                        left_result.coerce_to_string() == right_result.coerce_to_string(),
                    ));
                }
            }
            return Err("Could not complete request".to_string());
        }
    }
}

fn handle_comparators(
    env: &mut Environment,
    left_hand: &Box<Expression>,
    operator: &Operator,
    right_hand: &Box<Expression>,
) -> Result<ExpressionResult, String> {
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
    return Ok(ExpressionResult::Boolean(false));
}

fn handle_and_or_with_short_circuiting(
    env: &mut Environment,
    left_hand: &Box<Expression>,
    operator: &Operator,
    right_hand: &Box<Expression>,
) -> Result<ExpressionResult, String> {
    if let Ok(left_result) = eval_expression(*left_hand.clone(), env) {
        let left_bool = left_result.coerce_to_bool();
        if *operator == Operator::And && left_bool == false {
            // short circuit, don't eval right hand side, just return false
            return Ok(ExpressionResult::Boolean(false));
        }
        if *operator == Operator::Or && left_bool == true {
            // short circuit, don't eval right hand side, just return true
            return Ok(ExpressionResult::Boolean(true));
        } else {
            if let Ok(right_result) = eval_expression(*right_hand.clone(), env) {
                let right_bool = right_result.coerce_to_bool();
                if *operator == Operator::And {
                    return Ok(ExpressionResult::Boolean(left_bool && right_bool));
                } else {
                    return Ok(ExpressionResult::Boolean(left_bool || right_bool));
                }
            }
        }
    }
    Err(InterpreterError { kind: InterpreterErrorKind::SyntaxError(None)}.to_string())
}

fn handle_prefix_expression(
    env: &mut Environment,
    operator: PrefixOperator,
    expression: Box<Expression>,
) -> Result<ExpressionResult, String> {
    let result = eval_expression(*expression.clone(), env);
    if let Ok(value) = result {
        match operator {
            PrefixOperator::Negative | PrefixOperator::Positive => {
                let sign = if operator == PrefixOperator::Negative {
                    -1.0
                } else {
                    1.0
                };
                let coersion = value.coerce_to_number();
                if let Ok(number) = coersion {
                    return Ok(ExpressionResult::Number(sign * number));
                } else {
                    return Err(InterpreterError { kind: InterpreterErrorKind::NaN }.to_string());
                }
            }
            PrefixOperator::Not => {
                let bool = value.coerce_to_bool();
                Ok(ExpressionResult::Boolean(!bool))
            }
            PrefixOperator::Decrement | PrefixOperator::Increment => {
                match *expression {
                    Expression::Identifier(identifier) => {
                        return modify_variable_and_return_new_value(env, operator, identifier);
                    }
                    _ => {
                        return Err(InterpreterError { kind: InterpreterErrorKind::SyntaxError(Some(SyntaxErrorKind::InvalidLeftSidePrefix))}.to_string())
                    }
                }
            }
        }
    } else {
        return Err(InterpreterError { kind: InterpreterErrorKind::SyntaxError(None)}.to_string())
    }
}

fn modify_variable_and_return_new_value(
    env: &mut Environment,
    operator: PrefixOperator,
    identifier: String,
) -> Result<ExpressionResult, String> {
    let stored_value = env.get_variable(&identifier);
    match stored_value {
        Some(previous_value) => {
            if let Ok(previous_value_as_number) = previous_value.coerce_to_number() {
                let new = if operator == PrefixOperator::Decrement {
                    ExpressionResult::Number(previous_value_as_number - 1.0)
                } else {
                    ExpressionResult::Number(previous_value_as_number + 1.0)
                };
                env.set_variable(identifier.clone(), new.clone());
                return Ok(new);
            }
            return Err(InterpreterError { kind: InterpreterErrorKind::NaN }.to_string());
        }
        None => {
            return Err(InterpreterError { kind: InterpreterErrorKind::ReferenceError(identifier.clone())}.to_string());
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
        assert_eq!(result, ExpressionResult::Number(15.0));
    }
}
