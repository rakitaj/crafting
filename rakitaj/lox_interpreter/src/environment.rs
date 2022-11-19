use std::collections::{HashMap, hash_map::Entry};

use crate::{value::Value, core::{errors::LoxError, location::Location}};

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new_global()
    }
}

impl Environment {
    pub fn new_global() -> Self {
        Environment { values: HashMap::new(), enclosing: None }
    }

    pub fn new(enclosing_env: Box<Environment>) -> Self {
        Environment { values: HashMap::new(), enclosing: Some(enclosing_env) }
    }

    pub fn define(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        let local_value = self.values.get(key);
        if local_value.is_none() && self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().get(key);
        }
        local_value.cloned()
    }

    pub fn assign(&mut self, key: String, value: Value, location: Location) -> Result<Value, LoxError> {
        match self.values.entry(key.clone()) {
            Entry::Occupied(mut e) => {
                e.insert(value.clone());
                Ok(value)
            },
            _ => {
                match &mut self.enclosing {
                    Some(boxed_env) => (*boxed_env).assign(key, value, location),
                    None => Err(LoxError::RuntimeError(location, format!("Undefined variable: {}", key))),
                }
            }
        } 
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
        let mut env = Environment::new_global();
        match &value {
            Some(x) => env.define(name.to_string(), x.clone()),
            None => {}
        }
        assert_eq!(value, env.get(name));
    }

    #[test]
    fn test_get_variable_from_enclosing_env() {
        let mut global_env = Environment::new_global();
        global_env.define("foo".to_string(), Value::Number(42.0));
        let local_env = Environment::new(Box::new(global_env));
        assert_eq!(Some(Value::Number(42.0)), local_env.get("foo"))
    }

    #[test]
    fn test_get_nonexistant_variable_from_enclosing_env() {
        let global_env = Environment::new_global();
        let local_env = Environment::new(Box::new(global_env));
        assert_eq!(None, local_env.get("foo"))
    }

    #[test]
    fn test_variable_assignment() {
        let mut env = Environment::new_global();
        env.define("foo".to_string(), Value::Number(12.1));
        let result = env.assign("foo".to_string(), Value::Number(45.0), Location::Line("testfile.lox".to_string(), 85));
        assert_eq!(result, Ok(Value::Number(45.0)));
        assert_eq!(env.get("foo"), Some(Value::Number(45.0)));
    }

    #[test]
    fn test_enclosed_variable_assignment() {
        let mut global_env = Environment::new_global();
        global_env.define("foo".to_string(), Value::Number(42.0));
        let mut local_env = Environment::new(Box::new(global_env));
        let result = local_env.assign("foo".to_string(), Value::Number(45.0), Location::Line("testfile.lox".to_string(), 85));
        assert_eq!(result, Ok(Value::Number(45.0)));
        assert_eq!(local_env.get("foo"), Some(Value::Number(45.0)));
    }
}