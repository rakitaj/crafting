use crate::core::errors::LoxError;
use crate::parser::{Expr, Literal};
use crate::tokens::TokenType;
use crate::value::Value;

pub struct Interpreter {
    root_expr: Expr,
    runtime_error: bool
}

impl Interpreter {
    pub fn new(root_expr: Expr) -> Self {
        Interpreter { 
            root_expr, 
            runtime_error: false 
        }
    }

    pub fn interpret(mut self) {
        let result = self.evaluate(&self.root_expr);
        match result {
            Ok(x) => println!("{}", x),
            Err(err) => self.runtime_error(&err)
        }
    }

    fn runtime_error(&mut self, err: &LoxError) {
        self.runtime_error = true;
        println!("{}", err);
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, LoxError> {
        match expr {
            Expr::Literal(_, literal) => {
                match literal {
                    Literal::Nil => Ok(Value::Nil),
                    Literal::False => Ok(Value::Boolean(false)),
                    Literal::True => Ok(Value::Boolean(true)),
                    Literal::Number(number) => Ok(Value::Number(*number)),
                    Literal::String(string) => Ok(Value::String(string.to_string())),
                }
            },
            Expr::Grouping(grouping) => self.evaluate(grouping),
            Expr::Unary(operator, unary) => {
                let right: Value = self.evaluate(unary)?;
                match operator.token_type {
                    TokenType::Minus => {
                        match right {
                            Value::Number(number) => Ok(Value::Number(-number)),
                            _ =>  {
                                let error_msg = format!("Expected Value::Number and got {:?}", right);
                                Err(LoxError::RuntimeError(operator.location.clone(), error_msg))
                            }
                        }
                    },
                    TokenType::Bang => {
                        // Negate, aka flip, the result of is_truthy because this is the bang unary operator.
                        match right.is_truthy() {
                            true => Ok(Value::Boolean(false)),
                            false => Ok(Value::Boolean(true))
                        }
                    },
                    _ => Err(LoxError::RuntimeError(operator.location.clone(), "Visiting unary expr and wasn't a unary operator.".to_string()))
                }
            },
            Expr::Binary(left_expr, operator, right_expr) => {
                let left = self.evaluate(left_expr)?;
                let right = self.evaluate(right_expr)?;
                match (left, right) {
                    (Value::Number(left_num), Value::Number(right_num)) => {
                        match operator.token_type {
                            TokenType::Minus => Ok(Value::Number(left_num - right_num)),
                            TokenType::Slash => Ok(Value::Number(left_num / right_num)),
                            TokenType::Star => Ok(Value::Number(left_num * right_num)),
                            TokenType::Plus => Ok(Value::Number(left_num + right_num)),
                            TokenType::Greater => Ok(Value::Boolean(left_num > right_num)),
                            TokenType::GreaterEqual => Ok(Value::Boolean(left_num >= right_num)),
                            TokenType::Less => Ok(Value::Boolean(left_num < right_num)),
                            TokenType::LessEqual => Ok(Value::Boolean(left_num <= right_num)),
                            _ => Err(LoxError::RuntimeError(operator.location.clone(), "Should be unreachable".to_string()))
                        }
                    },
                    (Value::String(left_string), Value::String(right_string)) => {
                        match operator.token_type {
                            TokenType::Plus =>  {
                                let concat_string = format!("{}{}", left_string, right_string);
                                Ok(Value::String(concat_string))
                            },
                            _ => Err(LoxError::RuntimeError(operator.location.clone(), "Should be unreachable".to_string()))
                        }
                    }
                    (left, right) => {
                        match operator.token_type {
                            TokenType::EqualEqual => Ok(Value::Boolean(left == right)),
                            TokenType::BangEqual => Ok(Value::Boolean(!(left == right))),
                            _ => {
                                let msg = format!("Expected two numbers and got left: {:?} -- right: {:?}", left, right);
                                Err(LoxError::RuntimeError(operator.location.clone(), msg))
                            }
                        }
                    }
                }
            },
            _ => Err(LoxError::Critical("Shouldn't get here".to_string()))
        }
    }
}
