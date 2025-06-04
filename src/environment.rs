use std::collections::HashMap;
use crate::ast::ExpressionResult;

pub struct Environment {
    pub variables: HashMap<String, ExpressionResult>
}

impl Environment {
    pub fn new() -> Self {
        Environment { variables: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> Option<ExpressionResult> {
        self.variables.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: ExpressionResult) {
        self.variables.insert(name, value);
    }

    pub fn has(&mut self, name: String) -> bool {
        self.variables.contains_key(&name)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_set_new_variable() {
        let mut env = Environment::new();
        env.set("x".to_string(), ExpressionResult::Number(5.0));
        assert_eq!(env.get("x"), Option::Some(ExpressionResult::Number(5.0)));
    }

    #[test]
    fn it_should_modify_existing_variable() {
        let mut env = Environment::new();
        env.set("x".to_string(), ExpressionResult::Number(5.0));
        assert_eq!(env.get("x"), Option::Some(ExpressionResult::Number(5.0)));
        env.set("x".to_string(), ExpressionResult::Number(2.0));
        assert_eq!(env.get("x"), Option::Some(ExpressionResult::Number(2.0)));
    }

    #[test]
    fn it_should_return_true_on_has_if_variable_defined() {
        let mut env = Environment::new();
        env.set("x".to_string(), ExpressionResult::Number(5.0));
        assert_eq!(env.has("x".to_string()), true);
    }

    #[test]
    fn it_should_return_false_on_has_if_variable_undefined() {
        let mut env = Environment::new();
        assert_eq!(env.has("x".to_string()), false);
    }
}