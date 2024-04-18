use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f32),
    String(String),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(false) | Value::Nil => false,
            _ => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(x) => match x {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            Value::String(x) => write!(f, "{}", x),
            Value::Number(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Value::Nil, false)]
    #[case(Value::Boolean(false), false)]
    #[case(Value::Boolean(true), true)]
    #[case(Value::String("".to_string()), true)]
    fn test_is_truthy(#[case] value: Value, #[case] expected: bool) {
        assert_eq!(value.is_truthy(), expected);
    }
}
