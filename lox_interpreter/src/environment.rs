use std::collections::{hash_map::Entry, HashMap};

use crate::{
    core::{errors::LoxError, location::Location},
    value::Value,
};

pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
    index: usize,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        let scopes_list: Vec<HashMap<String, Value>> = vec![HashMap::new()];
        Environment {
            scopes: scopes_list,
            index: 0,
        }
    }

    pub fn new_child_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.index += 1;
    }

    pub fn destroy_child_scope(&mut self) {
        match self.scopes.pop() {
            Some(_) => (),
            _ => panic!("Internal error popping the child scope."),
        }
        self.index -= 1;
    }

    pub fn define(&mut self, key: String, value: Value) {
        let current_scope = &mut self.scopes[self.index];
        current_scope.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        for i in (0..=self.index).rev() {
            let current_scope = &self.scopes[i];
            let value = current_scope.get(key);
            if let Some(x) = value {
                return Some(x.clone());
            }
        }
        None
    }

    pub fn assign(&mut self, key: &str, value: Value, location: Location) -> Result<Value, LoxError> {
        for i in (0..=self.index).rev() {
            let current_scope = &mut self.scopes[i];
            if let Entry::Occupied(mut e) = current_scope.entry(key.to_string()) {
                e.insert(value.clone());
                return Ok(value);
            }
        }
        Err(LoxError::RuntimeError(
            location,
            format!("Undefined variable: {}", key),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("foo", Some(Value::Boolean(true)))]
    #[case("bar", None)]
    #[case("baz", Some(Value::Nil))]
    fn test_set_and_get_variable(#[case] name: &str, #[case] value: Option<Value>) {
        let mut env = Environment::new();
        match &value {
            Some(x) => env.define(name.to_string(), x.clone()),
            None => {}
        }
        assert_eq!(value, env.get(name));
    }

    #[test]
    fn test_get_variable_from_enclosing_env() {
        let mut env = Environment::new();
        env.define("foo".to_string(), Value::Number(42.0));
        env.new_child_scope();
        assert_eq!(Some(Value::Number(42.0)), env.get("foo"))
    }

    #[test]
    fn test_get_nonexistant_variable_from_enclosing_env() {
        let mut env = Environment::new();
        env.new_child_scope();
        assert_eq!(None, env.get("foo"))
    }

    #[test]
    fn test_variable_assignment() {
        let mut env = Environment::new();
        env.define("foo".to_string(), Value::Number(12.1));
        let result = env.assign(
            "foo",
            Value::Number(45.0),
            Location::Line("testfile.lox".to_string(), 85),
        );
        assert_eq!(result, Ok(Value::Number(45.0)));
        assert_eq!(env.get("foo"), Some(Value::Number(45.0)));
    }

    #[test]
    fn test_enclosed_variable_assignment() {
        let mut env = Environment::new();
        env.define("foo".to_string(), Value::Number(42.0));
        env.new_child_scope();
        let result = env.assign(
            "foo",
            Value::Number(45.0),
            Location::Line("testfile.lox".to_string(), 85),
        );
        assert_eq!(result, Ok(Value::Number(45.0)));
        assert_eq!(env.get("foo"), Some(Value::Number(45.0)));
    }
}
