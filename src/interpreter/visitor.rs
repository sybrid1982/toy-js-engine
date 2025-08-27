use crate::ast::{Expression, ExpressionResult, Operator, PrefixOperator, Statement, Node};
use crate::environment::Environment;
use crate::interpreter::{
    errors::{InterpreterError, InterpreterErrorKind, SyntaxErrorKind},
    operators::get_operator_strategy,
};

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

    fn evaluate_operation_expression(
        &mut self,
        left_hand: &Expression,
        operator: &Operator,
        right_hand: &Expression,
    ) -> Result<ExpressionResult, String> {
        let left_result = left_hand.accept(self);
        if let Ok(left_value) = left_result {
            // short circuit behavior for logical operators
            if *operator == Operator::And && !left_value.coerce_to_bool() {
                return Ok(ExpressionResult::Boolean(false));
            }
            if *operator == Operator::Or && left_value.coerce_to_bool() {
                return Ok(ExpressionResult::Boolean(true));
            }
            let right_value = right_hand.accept(self)?;
            let strategy = get_operator_strategy(operator.clone());
            strategy.apply(left_value, right_value, self.env)
        } else {
            Err(InterpreterError {
                kind: InterpreterErrorKind::SyntaxError(None),
            }
            .to_string())
        }
    }

    fn evaluate_prefix_expression(
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
                    let coercion = value.coerce_to_number();
                    if let Ok(number) = coercion {
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
                self.evaluate_prefix_expression(operator, expression)
            }
            Expression::Operation(left_hand, operator, right_hand) => {
                self.evaluate_operation_expression(left_hand, operator, right_hand)
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
