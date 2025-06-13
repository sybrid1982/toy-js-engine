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
        // TODO: changes made to variables that were in the parent env should be trickled up.
        // rough idea -> maybe variables need to contain a flag as to whether they belonged to the parent block,
        // and if we change one with that flag, we change it in the parent as well
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