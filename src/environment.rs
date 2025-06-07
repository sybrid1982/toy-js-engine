use std::collections::HashMap;
use crate::ast::ExpressionResult;
use crate::function::Function;

#[derive(Clone)]
pub struct Environment {
    pub variables: HashMap<String, (bool, ExpressionResult)>,
    pub functions: HashMap<String, Function>       
}

impl Environment {
    pub fn new() -> Self {
        Environment { variables: HashMap::new(), functions: HashMap::new() }
    }

    pub fn get_variable(&self, identifier: &str) -> Option<ExpressionResult> {
        return match self.variables.get(identifier) {
            Some((_, value)) => Some(value.clone()),
            None => None
        };
    }

    pub fn define_variable(&mut self, identifier: String, value: ExpressionResult) {
        self.variables.insert(identifier, (false, value));
    }

    pub fn set_variable(&mut self, identifier: String, value: ExpressionResult) {
        let inherited = match self.variables.get(&identifier) {
            Some((inherited, _)) => inherited.clone(),
            None => false
        };
        self.variables.insert(identifier, (inherited, value));
    }

    pub fn has_variable(&mut self, identifier: String) -> bool {
        self.variables.contains_key(&identifier)
    }

    pub fn is_variable_greater_scope(&self, identifier: String) -> bool {
        return match self.variables.get(&identifier) {
            Some((inherited, _)) => inherited.clone(),
            None => false
        };
    }

    pub fn get_function(&self, identifier: &str) -> Option<Function> {
        self.functions.get(identifier).cloned()
    }

    pub fn set_function(&mut self, identifier: String, value: Function) {
        self.functions.insert(identifier, value);
    }

    pub fn has_function(&mut self, identifier: String) -> bool {
        self.functions.contains_key(&identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_set_new_variable() {
        let mut env = Environment::new();
        env.set_variable("x".to_string(), ExpressionResult::Number(5.0));
        assert_eq!(env.get_variable("x"), Option::Some(ExpressionResult::Number(5.0)));
    }

    #[test]
    fn it_should_modify_existing_variable() {
        let mut env = Environment::new();
        env.set_variable("x".to_string(), ExpressionResult::Number(5.0));
        assert_eq!(env.get_variable("x"), Option::Some(ExpressionResult::Number(5.0)));
        env.set_variable("x".to_string(), ExpressionResult::Number(2.0));
        assert_eq!(env.get_variable("x"), Option::Some(ExpressionResult::Number(2.0)));
    }

    #[test]
    fn it_should_return_true_on_has_if_variable_defined() {
        let mut env = Environment::new();
        env.set_variable("x".to_string(), ExpressionResult::Number(5.0));
        assert_eq!(env.has_variable("x".to_string()), true);
    }

    #[test]
    fn it_should_return_false_on_has_if_variable_undefined() {
        let mut env = Environment::new();
        assert_eq!(env.has_variable("x".to_string()), false);
    }
}