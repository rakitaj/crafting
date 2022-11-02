use std::collections::HashMap;

use crate::{value::Value, core::errors::LoxError};

pub struct Environment {
    values: HashMap<String, Value>
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new() }
    }

    pub fn define(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.values.get(key).cloned()
    }

    pub fn assign(&mut self, key: String, value: Value) -> Result<(), LoxError> {
        if self.values.contains_key(&key) {
            self.values.insert(key, value);
        } else {
            return LoxError::RuntimeError((), format!("Undefined variable: {}", key))
        }
    }
}