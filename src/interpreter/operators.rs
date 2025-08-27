use crate::ast::{ExpressionResult, Operator};
use crate::environment::Environment;
use crate::interpreter::errors::{InterpreterError, InterpreterErrorKind};

pub trait BinaryOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        env: &mut Environment
    ) -> Result<ExpressionResult, String>;
}

/// AddOperator is the more complicated than the other arithmetic operators
/// because it also handles string concatenation on top of numeric addition.
pub struct AddOperator;
impl BinaryOperator for AddOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment
    ) -> Result<ExpressionResult, String> {
        if matches!(left, ExpressionResult::String(_))
            || matches!(right, ExpressionResult::String(_))
        {
            let new_string = left.coerce_to_string() + &right.coerce_to_string();
            Ok(ExpressionResult::String(new_string))
        } else {
            let left_num = left.coerce_to_number();
            let right_num = right.coerce_to_number();
            if let (Ok(l), Ok(r)) = (left_num, right_num) {
                Ok(ExpressionResult::Number(l + r))
            } else {
                Err(InterpreterError {
                    kind: InterpreterErrorKind::NaN,
                }
                .to_string())
            }
        }
    }
}

pub struct SubtractOperator;
impl BinaryOperator for SubtractOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        if let (Ok(l), Ok(r)) = (left.coerce_to_number(), right.coerce_to_number()) {
            Ok(ExpressionResult::Number(l - r))
        } else {
            Err(InterpreterError {
                kind: InterpreterErrorKind::NaN,
            }
            .to_string())
        }
    }
}

pub struct MultiplyOperator;
impl BinaryOperator for MultiplyOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        if let (Ok(l), Ok(r)) = (left.coerce_to_number(), right.coerce_to_number()) {
            Ok(ExpressionResult::Number(l * r))
        } else {
            Err(InterpreterError {
                kind: InterpreterErrorKind::NaN,
            }
            .to_string())
        }
    }
}

/// Division needs special handling for division by zero
pub struct DivideOperator;
impl BinaryOperator for DivideOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        if let (Ok(l), Ok(r)) = (left.coerce_to_number(), right.coerce_to_number()) {
            if r.abs() < f64::EPSILON {
                Err(InterpreterError {
                    kind: InterpreterErrorKind::DivisionByZero
                }
                .to_string())
            } else {
                Ok(ExpressionResult::Number(l / r))
            }
        } else {
            Err(InterpreterError {
                kind: InterpreterErrorKind::NaN,
            }
            .to_string())
        }
    }
}

pub struct ModuloOperator;
impl BinaryOperator for ModuloOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        if let (Ok(l), Ok(r)) = (left.coerce_to_number(), right.coerce_to_number()) {
            Ok(ExpressionResult::Number(l % r))
        } else {
            Err(InterpreterError {
                kind: InterpreterErrorKind::NaN,
            }
            .to_string())
        }
    }
}

pub struct ExponentiationOperator;
impl BinaryOperator for ExponentiationOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        if let (Ok(l), Ok(r)) = (left.coerce_to_number(), right.coerce_to_number()) {
            Ok(ExpressionResult::Number(l.powf(r)))
        } else {
            Err(InterpreterError {
                kind: InterpreterErrorKind::NaN,
            }
            .to_string())
        }
    }
}

pub struct EqualOperator;
impl BinaryOperator for EqualOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        if matches!(left, ExpressionResult::Boolean(_))
            || matches!(right, ExpressionResult::Boolean(_))
        {
            return Ok(ExpressionResult::Boolean(
                left.coerce_to_bool() == right.coerce_to_bool(),
            ));
        }

        if matches!(left, ExpressionResult::Number(_))
            || matches!(right, ExpressionResult::Number(_))
        {
            let left_num = left.coerce_to_number();
            let right_num = right.coerce_to_number();
            if let (Ok(l), Ok(r)) = (left_num, right_num) {
                return Ok(ExpressionResult::Boolean(l == r));
            } else {
                return Err(InterpreterError {
                    kind: InterpreterErrorKind::NaN,
                }
                .to_string());
            }
        }

        Ok(ExpressionResult::Boolean(
            left.coerce_to_string() == right.coerce_to_string(),
        ))
    }
}

pub struct LessThanOperator;
impl BinaryOperator for LessThanOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        if let (Ok(l), Ok(r)) = (left.coerce_to_number(), right.coerce_to_number()) {
            Ok(ExpressionResult::Boolean(l < r))
        } else {
            Ok(ExpressionResult::Boolean(false))
        }
    }
}

pub struct GreaterThanOperator;
impl BinaryOperator for GreaterThanOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        if let (Ok(l), Ok(r)) = (left.coerce_to_number(), right.coerce_to_number()) {
            Ok(ExpressionResult::Boolean(l > r))
        } else {
            Ok(ExpressionResult::Boolean(false))
        }
    }
}

pub struct AndOperator;
impl BinaryOperator for AndOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        Ok(ExpressionResult::Boolean(
            left.coerce_to_bool() && right.coerce_to_bool(),
        ))
    }
}

pub struct OrOperator;
impl BinaryOperator for OrOperator {
    fn apply(
        &self,
        left: ExpressionResult,
        right: ExpressionResult,
        _env: &mut Environment,
    ) -> Result<ExpressionResult, String> {
        Ok(ExpressionResult::Boolean(
            left.coerce_to_bool() || right.coerce_to_bool(),
        ))
    }
}

pub fn get_operator_strategy(operator: Operator) -> Box<dyn BinaryOperator> {
    match operator {
        Operator::Add => Box::new(AddOperator),
        Operator::Subtract => Box::new(SubtractOperator),
        Operator::Multiply => Box::new(MultiplyOperator),
        Operator::Divide => Box::new(DivideOperator),
        Operator::Modulo => Box::new(ModuloOperator),
        Operator::Equal => Box::new(EqualOperator),
        Operator::LessThan => Box::new(LessThanOperator),
        Operator::GreaterThan => Box::new(GreaterThanOperator),
        Operator::And => Box::new(AndOperator),
        Operator::Or => Box::new(OrOperator),
        Operator::Exponentiation => Box::new(ExponentiationOperator),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_operator_should_concatenate_strings() {
        let left = ExpressionResult::String("Hello, ".into());
        let right = ExpressionResult::String("world!".into());
        let operator = AddOperator;
        let result = operator.apply(left, right, &mut Environment::new()).unwrap();
        assert_eq!(result, ExpressionResult::String("Hello, world!".into()));
    }

    #[test]
    fn add_operator_should_add_numbers() {
        let left = ExpressionResult::Number(5.0);
        let right = ExpressionResult::Number(3.0);
        let operator = AddOperator;
        let result = operator.apply(left, right, &mut Environment::new()).unwrap();
        assert_eq!(result, ExpressionResult::Number(8.0));
    }

    #[test]
    fn add_operator_should_handle_string_and_number() {
        let left = ExpressionResult::String("The answer is ".into());
        let right = ExpressionResult::Number(42.0);
        let operator = AddOperator;
        let result = operator.apply(left, right, &mut Environment::new()).unwrap();
        assert_eq!(result, ExpressionResult::String("The answer is 42".into()));
    }

    #[test]
    fn divide_operator_should_handle_division_by_zero() {     
        let left = ExpressionResult::Number(10.0);
        let right = ExpressionResult::Number(0.0);
        let operator = DivideOperator;
        let result = operator.apply(left, right, &mut Environment::new());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            InterpreterError {
                kind: InterpreterErrorKind::DivisionByZero
            }
            .to_string()
        );
    }
}