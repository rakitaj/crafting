use crate::core::errors::LoxError;
use crate::environment::Environment;
use crate::parser::{Expr, Literal, Stmt};
use crate::tokens::TokenType;
use crate::value::Value;

struct InterpreterState {
    runtime_error: bool,
    environment: Environment
}

impl<'a> InterpreterState {
    fn runtime_error(&mut self, err: &LoxError) {
        self.runtime_error = true;
        println!("{}", err);
    }

    fn new() -> Self {
        InterpreterState { runtime_error: false, environment: Environment::new() }
    }
}

pub struct Interpreter {
    statements: Vec<Stmt>,
    state: InterpreterState,
    
}

impl Interpreter {
    pub fn new(statements: Vec<Stmt>) -> Self {
        let state = InterpreterState::new();
        Interpreter { 
            statements,
            state
        }
    }

    pub fn interpret(&mut self) -> Vec<LoxError> {
        let mut errors: Vec<LoxError> = Vec::new();
        for stmt in &self.statements {
            let result = self.evaluate(stmt);
            match result {
                Ok(x) => match x {
                    Some(y) => println!("{}", y),
                    None => println!("No value returned. No errors.")
                },
                Err(err) => {
                    self.state.runtime_error(&err);
                    errors.push(err);
                }
            }
        }
        errors
    }

    fn evaluate(&mut self, stmt: &Stmt) -> Result<Option<Value>, LoxError> {
        match stmt {
            Stmt::Expression(expr) => {
                let value = self.evaluate_expr(expr)?;
                Ok(Some(value))
            },
            Stmt::Print(expr) => self.evaluate_print(expr),
            Stmt::Var(identifier_token, initializer) => {
                if let TokenType::Identifier(name) = &identifier_token.token_type {
                    let value = self.evaluate_expr(initializer)?;
                    self.state.environment.define(name.to_string(), value);
                    Ok(None)
                } else {
                    Err(LoxError::RuntimeError(identifier_token.location.clone(), format!("Expected identifier token for var name. {}", identifier_token)))
                }
            }
        }
    }

    fn evaluate_print(&self, expr: &Expr) -> Result<Option<Value>, LoxError> {
        let value = self.evaluate_expr(expr)?;
        println!("{}", value);
        Ok(None)
    }

    fn evaluate_expr(&self, expr: &Expr) -> Result<Value, LoxError> {
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
            Expr::Grouping(grouping) => self.evaluate_expr(grouping),
            Expr::Unary(operator, unary) => {
                let right: Value = self.evaluate_expr(unary)?;
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
                let left = self.evaluate_expr(left_expr)?;
                let right = self.evaluate_expr(right_expr)?;
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
