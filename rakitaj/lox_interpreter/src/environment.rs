use std::collections::HashMap;

use crate::{value::Value};

pub struct Environment {
    values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new() }
    }

    pub fn define(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        match self.values.get(key) {
            Some(x) => Some((*x).clone()),
            None => None
        }
    } 
}