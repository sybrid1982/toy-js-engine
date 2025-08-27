use crate::ast::{Expression, ExpressionResult, Statement, Node};
use crate::environment::Environment;
use crate::function::Function;
use crate::interpreter::visitor::Evaluator;

pub fn process_statements(
    mut statements: Vec<Statement>,
    env: &mut Environment,
) -> ExpressionResult {
    hoist(&mut statements, env);
    eval_statements(statements, env)
}

pub fn eval_statements(statements: Vec<Statement>, env: &mut Environment) -> ExpressionResult {
    for statement in statements {
        let result = eval_statement(statement, env);
        if let Some(value) = &result {
            return value.clone();
        }
    }
    ExpressionResult::Undefined
}

// Function declarations should be parsed
// If I wanted to support declaring variables with var, I'd also need to hoist those to match how var works
pub fn hoist(statements: &mut Vec<Statement>, env: &mut Environment) {
    for statement in &mut *statements {
        match statement {
            Statement::FunctionDeclaration(identifier, arguments, block) => {
                let function = Function::new(arguments.clone(), block.clone());
                env.set_function(identifier.clone(), function);
            }
            _ => {}
        }
    }
    *statements = statements
        .iter()
        .filter(|statement| !matches!(statement, Statement::FunctionDeclaration(_, _, _)))
        .map(|statement| statement.clone())
        .collect::<Vec<Statement>>();
}

pub fn eval_statement(statement: Statement, env: &mut Environment) -> Option<ExpressionResult> {
    let mut evaluator = Evaluator::new(env);
    statement.accept(&mut evaluator)
}

pub fn eval_expression(
    expression: Expression,
    env: &mut Environment,
) -> Result<ExpressionResult, String> {
    let mut evaluator = Evaluator::new(env);
    expression.accept(&mut evaluator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Operator, PrefixOperator};

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

    #[test]
    fn test_short_circuit_false_and() {
        let expression = Expression::Operation(
            Box::new(Expression::Boolean(false)),
            Operator::And,
            Box::new(Expression::Prefix(
                PrefixOperator::Increment,
                Box::new(Expression::Identifier("x".into())),
            )),
        );
        let mut env = Environment::new();
        env.define_variable("x".into(), ExpressionResult::Number(0.0));
        let _res = eval_expression(expression, &mut env);
        assert_eq!(
            ExpressionResult::Number(0.0),
            env.get_variable("x").unwrap()
        );
    }

    #[test]
    fn test_short_circuit_true_or() {
        let expression = Expression::Operation(
            Box::new(Expression::Boolean(true)),
            Operator::Or,
            Box::new(Expression::Prefix(
                PrefixOperator::Increment,
                Box::new(Expression::Identifier("x".into())),
            )),
        );
        let mut env = Environment::new();
        env.define_variable("x".into(), ExpressionResult::Number(0.0));
        let _res = eval_expression(expression, &mut env);
        assert_eq!(
            ExpressionResult::Number(0.0),
            env.get_variable("x").unwrap()
        );
    }
}
