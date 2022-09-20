#[derive(Debug)]
pub enum Value {
    Nil,
    True,
    False,
    Number(f32),
    String(String),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::False | Value::Nil => false,
            _ => true
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Value::Nil, false)]
    #[case(Value::False, false)]
    #[case(Value::True, true)]
    #[case(Value::String("".to_string()), true)]
    fn test_is_truthy(#[case] value: Value, #[case] expected: bool) {
        assert_eq!(value.is_truthy(), expected);
    }
}