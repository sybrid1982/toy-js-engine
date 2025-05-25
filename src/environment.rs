use std::collections::HashMap;

pub struct Environment {
    pub variables: HashMap<String, f64>
}

impl Environment {
    pub fn new() -> Self {
        Environment { variables: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }

    pub fn set(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }
}