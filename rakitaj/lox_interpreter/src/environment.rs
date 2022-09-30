use std::collections::HashMap;

use crate::{value::Value};

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
}