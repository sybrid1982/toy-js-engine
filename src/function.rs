use crate::ast::{Block, Expression, ExpressionResult};
use crate::environment::Environment;
use crate::interpreter::{eval_expression};

// A Function consists of its arguments, and block to be executed after setting the environment up from arguments
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    arguments: Vec<Expression>,
    block: Block
}

impl Function {
    pub fn new(arguments: Vec<Expression>, block: Block) -> Self {
        Function {
            arguments,
            block
        }
    }

    pub fn call(&self, arguments: Vec<Expression>, parent_env: &mut Environment) -> Result<ExpressionResult, String> {
        if self.arguments.len() != arguments.len() {
            return Err(format!("Argument mismatch, function expected {} arguments, recieved {}", self.arguments.len(), arguments.len()));
        }
        let mut block_env = parent_env.create_child_env();
        // load arguments into block environment
        for (index, argument) in self.arguments.iter().enumerate() {
            match argument {
                Expression::Identifier(identifier) => {
                    let result = eval_expression(arguments[index].clone(), &mut block_env);
                    if let Ok(val) = result {
                        block_env.define_variable(identifier.to_string(), val)
                    }
                },
                _ => return Err("SyntaxError: Argument declaration should be of identifier type".to_string())
            }
        }
        let result = self.block.execute_block(&mut block_env);
        parent_env.merge_child_env(block_env);
        return result;
    }
}

#[cfg(test)]
mod function_tests {
    use super::*;

    #[test]
    fn it_should_throw_error_when_call_with_less_parameters() {
        let argument = Expression::Identifier("x".into());
        let block = Block::new(vec![]);
        let function = Function::new(vec![argument], block);
        let mut env = Environment::new();
        let result = function.call(vec![], &mut env);
        assert_eq!(result, Err("Argument mismatch, function expected 1 arguments, recieved 0".into()));
    }

    #[test]
    fn it_should_throw_error_when_call_with_more_parameters() {
        let argument = Expression::Identifier("x".into());
        let block = Block::new(vec![]);
        let function = Function::new(vec![], block);
        let mut env = Environment::new();
        let result = function.call(vec![argument], &mut env);
        assert_eq!(result, Err("Argument mismatch, function expected 0 arguments, recieved 1".into()));
    }
}