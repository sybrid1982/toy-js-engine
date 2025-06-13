use std::collections::{HashMap, HashSet};
use crate::ast::ExpressionResult;
use crate::function::Function;

#[derive(Clone)]
pub struct Environment {
    pub variables: HashMap<String, (bool, ExpressionResult)>,
    pub functions: HashMap<String, Function>,
    modified_inherited_variables: HashSet<String>
}

impl Environment {
    pub fn new() -> Self {
        Environment { variables: HashMap::new(), functions: HashMap::new(), modified_inherited_variables: HashSet::new() }
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
        let inherited = self.is_variable_greater_scope(&identifier);
        if inherited {
            self.modified_inherited_variables.insert(identifier.clone());
        }
        self.variables.insert(identifier, (inherited, value.clone()));
    }

    pub fn has_variable(&mut self, identifier: String) -> bool {
        self.variables.contains_key(&identifier)
    }

    pub fn is_variable_greater_scope(&self, identifier: &String) -> bool {
        return match self.variables.get(identifier) {
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

    pub fn create_child_env(&mut self) -> Environment {
        let mut child_env = self.clone();
        for (inherited, _) in child_env.variables.values_mut() {
            *inherited = true
        }
        return child_env
    }

    pub fn merge_child_env(&mut self, child_env: Environment) {
        for identifier in &child_env.modified_inherited_variables {
            let child_value = child_env.get_variable(&identifier);
            if let Some(result) = child_value {
                self.set_variable(identifier.clone(), result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_set_new_variable() {
        let mut env = Environment::new();
        env.define_variable("x".to_string(), ExpressionResult::Number(5.0));
        assert_eq!(env.get_variable("x"), Option::Some(ExpressionResult::Number(5.0)));
    }

    #[test]
    fn it_should_modify_existing_variable() {
        let mut env = Environment::new();
        env.define_variable("x".to_string(), ExpressionResult::Number(5.0));
        assert_eq!(env.get_variable("x"), Option::Some(ExpressionResult::Number(5.0)));
        env.define_variable("x".to_string(), ExpressionResult::Number(2.0));
        assert_eq!(env.get_variable("x"), Option::Some(ExpressionResult::Number(2.0)));
    }

    #[test]
    fn it_should_return_true_on_has_if_variable_defined() {
        let mut env = Environment::new();
        env.define_variable("x".to_string(), ExpressionResult::Number(5.0));
        assert_eq!(env.has_variable("x".to_string()), true);
    }

    #[test]
    fn it_should_return_false_on_has_if_variable_undefined() {
        let mut env = Environment::new();
        assert_eq!(env.has_variable("x".to_string()), false);
    }
}