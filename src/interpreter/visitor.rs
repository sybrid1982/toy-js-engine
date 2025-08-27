use crate::ast::{Expression, ExpressionResult, Operator, PrefixOperator, Statement, Node};
use crate::environment::Environment;
use crate::interpreter::errors::{InterpreterError, InterpreterErrorKind, SyntaxErrorKind};

/// Trait for visiting AST nodes.
/// 
/// Statements return `Option<ExpressionResult>` to allow early returns,
/// while expressions return a `Result<ExpressionResult, String>` to surface runtime errors.
pub trait NodeVisitor {
    fn visit_statement(&mut self, statement: &Statement) -> Option<ExpressionResult>;
    fn visit_expression(&mut self, expression: &Expression) -> Result<ExpressionResult, String>;
}

pub struct Evaluator<'a> {
    pub env: &'a mut Environment
}

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Self { env }
    }

    fn handle_operation_expression(
        &mut self,
        left_hand: &Expression,
        operator: &Operator,
        right_hand: &Expression
    ) -> Result<ExpressionResult, String> {
        match operator {
            // currently these operators always treat both sides as boolean
            Operator::And | Operator::Or => {
                return self.handle_and_or_with_short_circuiting(left_hand, operator, right_hand);
            }
            // currently these operators treat both sides as numbers, and if either side is not a number, return false
            Operator::LessThan | Operator::GreaterThan => {
                return self.handle_comparators(left_hand, operator, right_hand);
            }
            // this really only makes sense for values that can be coerced to numbers, and will either return a number or NaN
            Operator::Multiply | Operator::Divide | Operator::Subtract | Operator::Modulo => {
                if let Ok(left_result) = left_hand.accept(self) {
                    if let Ok(right_result) = right_hand.accept(self)  {
                        if let Ok(left_as_num) = left_result.coerce_to_number() {
                            if let Ok(right_as_num) = right_result.coerce_to_number() {
                                if *operator == Operator::Multiply {
                                    return Ok(ExpressionResult::Number(left_as_num * right_as_num));
                                } else if *operator == Operator::Divide {
                                    return Ok(ExpressionResult::Number(left_as_num / right_as_num));
                                } else if *operator == Operator::Subtract {
                                    return Ok(ExpressionResult::Number(left_as_num - right_as_num));
                                } else if *operator == Operator::Modulo {
                                    return Ok(ExpressionResult::Number(left_as_num % right_as_num));
                                }
                            }
                        }
                    }
                }
                return Err(InterpreterError {
                    kind: InterpreterErrorKind::NaN,
                }
                .to_string());
            }
            Operator::Add => {
                if let Ok(left_result) = left_hand.accept(self) {
                    if let Ok(right_result) = right_hand.accept(self)  {
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
                            return Err(InterpreterError {
                                kind: InterpreterErrorKind::NaN,
                            }
                            .to_string());
                        }
                    }
                }
                return Err("Could not complete request".to_string());
            }
            Operator::Equal => {
                if let Ok(left_result) = left_hand.accept(self) {
                    if let Ok(right_result) = right_hand.accept(self)  {
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
                            return Err(InterpreterError {
                                kind: InterpreterErrorKind::NaN,
                            }
                            .to_string());
                        }
                        // at this point both sides must be strings, check if the strings are the same
                        return Ok(ExpressionResult::Boolean(
                            left_result.coerce_to_string() == right_result.coerce_to_string(),
                        ));
                    }
                }
                return Err("Could not complete request".to_string());
            }
            Operator::Exponentiation => {
                if let Ok(right_result) = right_hand.accept(self)  {
                    if let Ok(right_value) = right_result.coerce_to_number() {
                        if let Ok(left_result) = left_hand.accept(self) {
                            if let Ok(left_value) = left_result.coerce_to_number() {
                                let value = left_value.powf(right_value);
                                return Ok(ExpressionResult::Number(value));
                            }
                        }
                    }
                }
                return Err(InterpreterError {
                    kind: InterpreterErrorKind::NaN,
                }
                .to_string());
            }
        }
    }

    fn handle_comparators(
        &mut self,
        left_hand: &Expression,
        operator: &Operator,
        right_hand: &Expression,
    ) -> Result<ExpressionResult, String> {
        let left_result = left_hand.accept(self);
        let right_result = right_hand.accept(self);
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
        &mut self,
        left_hand: &Expression,
        operator: &Operator,
        right_hand: &Expression,
    ) -> Result<ExpressionResult, String> {
        if let Ok(left_result) = left_hand.accept(self) {
            let left_bool = left_result.coerce_to_bool();
            if *operator == Operator::And && left_bool == false {
                // short circuit, don't eval right hand side, just return false
                return Ok(ExpressionResult::Boolean(false));
            }
            if *operator == Operator::Or && left_bool == true {
                // short circuit, don't eval right hand side, just return true
                return Ok(ExpressionResult::Boolean(true));
            } else {
                if let Ok(right_result) = right_hand.accept(self) {
                    let right_bool = right_result.coerce_to_bool();
                    if *operator == Operator::And {
                        return Ok(ExpressionResult::Boolean(left_bool && right_bool));
                    } else {
                        return Ok(ExpressionResult::Boolean(left_bool || right_bool));
                    }
                }
            }
        }
        Err(InterpreterError {
            kind: InterpreterErrorKind::SyntaxError(None),
        }
        .to_string())
    }

    fn handle_prefix_expression(
        &mut self,
        operator: &PrefixOperator,
        expression: &Expression,
    ) -> Result<ExpressionResult, String> {
        let result = expression.accept(self);
        if let Ok(value) = result {
            match operator {
                PrefixOperator::Negative | PrefixOperator::Positive => {
                    let sign = if *operator == PrefixOperator::Negative {
                        -1.0
                    } else {
                        1.0
                    };
                    let coersion = value.coerce_to_number();
                    if let Ok(number) = coersion {
                        return Ok(ExpressionResult::Number(sign * number));
                    } else {
                        return Err(InterpreterError {
                            kind: InterpreterErrorKind::NaN,
                        }
                        .to_string());
                    }
                }
                PrefixOperator::Not => {
                    let bool = value.coerce_to_bool();
                    Ok(ExpressionResult::Boolean(!bool))
                }
                PrefixOperator::Decrement | PrefixOperator::Increment => match expression {
                    Expression::Identifier(identifier) => {
                        return self
                            .modify_variable_and_return_new_value(operator.clone(), identifier.clone());
                    }
                    _ => {
                        return Err(InterpreterError {
                            kind: InterpreterErrorKind::SyntaxError(Some(
                                SyntaxErrorKind::InvalidLeftSidePrefix,
                            )),
                        }
                        .to_string())
                    }
                },
            }
        } else {
            return Err(InterpreterError {
                kind: InterpreterErrorKind::SyntaxError(None),
            }
            .to_string());
        }
    }

    fn modify_variable_and_return_new_value(
        &mut self,
        operator: PrefixOperator,
        identifier: String,
    ) -> Result<ExpressionResult, String> {
        let stored_value = self.env.get_variable(&identifier);
        match stored_value {
            Some(previous_value) => {
                if let Ok(previous_value_as_number) = previous_value.coerce_to_number() {
                    let new = if operator == PrefixOperator::Decrement {
                        ExpressionResult::Number(previous_value_as_number - 1.0)
                    } else {
                        ExpressionResult::Number(previous_value_as_number + 1.0)
                    };
                    self.env.set_variable(identifier.clone(), new.clone());
                    return Ok(new);
                }
                return Err(InterpreterError {
                    kind: InterpreterErrorKind::NaN,
                }
                .to_string());
            }
            None => {
                return Err(InterpreterError {
                    kind: InterpreterErrorKind::ReferenceError(identifier.clone()),
                }
                .to_string());
            }
        }
    }
}

impl<'a> NodeVisitor for Evaluator<'a> {
        fn visit_statement(&mut self, statement: &Statement) -> Option<ExpressionResult> {
        let repeat_statement = statement.clone();
        match statement {
            Statement::Let(identifier, expression) => {
                let result = expression.accept(self);
                match result {
                    Ok(val) => {
                        self.env.define_variable(identifier.clone(), val);
                    }
                    Err(error) => {
                        println!("{:#?}", error);
                    }
                }
                return None;
            }
            Statement::ExpressionStatement(expression) => {
                let result = expression.accept(self);
                if let Ok(value) = result {
                    println!("{}", value)
                } else if let Err(error) = result {
                    println!("{:#?}", error)
                }
                return None;
            }
            Statement::ReturnStatement(return_expression) => {
                if let Some(expression) = return_expression {
                    let result = expression.accept(self);
                    if let Ok(value) = result {
                        return Some(value);
                    }
                }
                Some(ExpressionResult::Undefined)
            }
            Statement::ConditionalStatement(condition, block, next_conditional) => {
                if let Ok(expression_result) = condition.accept(self) {
                    if expression_result.coerce_to_bool() {
                        let mut block_env = self.env.create_child_env();
                        let _block_result = block.execute_block(&mut block_env);
                        self.env.merge_child_env(block_env);
                    } else if let Some(next_conditional_statement) = &**next_conditional {
                        return next_conditional_statement.accept(self);
                    }
                }
                return None;
            }
            Statement::While(inner_conditional) => {
                match &**inner_conditional {
                    Statement::ConditionalStatement(condition, block, _next_conditional) => {
                        if let Ok(expression_result) = condition.accept(self) {
                            if expression_result.coerce_to_bool() {
                                let mut block_env = self.env.create_child_env();
                                let _block_result = block.execute_block(&mut block_env);
                                self.env.merge_child_env(block_env);
                                return self.visit_statement(&repeat_statement);
                            }
                        }
                        return None;
                    }
                    _ => panic!("while statement should only contain conditional statement"),
                }
            }
            _ => None, // Function declarations are hoisted, so shouldn't reach here
        }
    }

    fn visit_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<ExpressionResult, String> {
        match expression {
            Expression::NumberLiteral(n) => Ok(ExpressionResult::Number(*n)),
            Expression::Identifier(identifier) => match self.env.get_variable(identifier) {
                Some(value) => Ok(value),
                None => Err(InterpreterError {
                    kind: InterpreterErrorKind::ReferenceError(identifier.clone()),
                }
                .to_string()),
            },
            Expression::Boolean(is_true) => {
                if *is_true {
                    Ok(ExpressionResult::Boolean(true))
                } else {
                    Ok(ExpressionResult::Boolean(false))
                }
            }
            Expression::String(string) => Ok(ExpressionResult::String(string.clone())),
            Expression::Prefix(operator, expression) => {
                self.handle_prefix_expression(operator, expression)
            }
            Expression::Operation(left_hand, operator, right_hand) => {
                self.handle_operation_expression(left_hand, operator, right_hand)
            }
            Expression::Assignment(left_hand, right_hand) => match &**left_hand {
                Expression::Identifier(identifier) => {
                    if self.env.has_variable(identifier.clone()) {
                        let result = right_hand.accept(self);
                        if let Ok(value) = &result {
                            self.env.set_variable(identifier.clone(), value.clone());
                        }
                        result
                    } else {
                        Err(InterpreterError {
                            kind: InterpreterErrorKind::ReferenceError(identifier.clone()),
                        }
                        .to_string())
                    }
                }
                _ => Err(InterpreterError {
                    kind: InterpreterErrorKind::SyntaxError(Some(
                        SyntaxErrorKind::LeftSideAssignmentMustBeIdentifier,
                    )),
                }
                .to_string()),
            },
            Expression::Call(callee, arguments) => match &**callee {
                Expression::Identifier(identifier) => {
                    if let Some(function) = self.env.get_function(identifier) {
                        return function.call(arguments.clone(), self.env);
                    }
                    return Err(format!("Function {} not defined", identifier));
                }
                _ => {
                    return Err("Either not implemented or not valid".into());
                }
            },
        }
    }
}
