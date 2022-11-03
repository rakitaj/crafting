use std::collections::{HashMap, hash_map::Entry};

use crate::{value::Value, core::{errors::LoxError, location::Location}};

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

    pub fn assign(&mut self, key: String, value: Value, location: Location) -> Result<Value, LoxError> {
        match self.values.entry(key.clone()) {
            Entry::Occupied(mut e) => {
                e.insert(value.clone());
                Ok(value)
            }
            _ => Err(LoxError::RuntimeError(location, format!("Undefined variable: {}", key))),
        } 
    }
}