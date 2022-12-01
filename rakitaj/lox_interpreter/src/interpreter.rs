use crate::core::errors::LoxError;
use crate::environment::Environment;
use crate::parser::{Expr, Literal, Stmt};
use crate::tokens::TokenType;
use crate::value::Value;

pub struct Interpreter {
    statements: Vec<Stmt>,
}

impl Interpreter {
    pub fn new(statements: Vec<Stmt>) -> Self {
        
        Interpreter { 
            statements
        }
    }

    pub fn interpret(&self) -> Vec<LoxError> {
        let mut errors: Vec<LoxError> = Vec::new();
        let mut environment = Environment::new();
        for stmt in &self.statements {
            let result = self.evaluate(stmt, &mut environment);
            match result {
                Ok(x) => if let Some(y) = x { println!("{}", y) }
                Err(err) => {
                    println!("Error: {}", err);
                    errors.push(err);
                }
            }
        }
        errors
    }

    fn evaluate(&self, stmt: &Stmt, environment: &mut Environment) -> Result<Option<Value>, LoxError> {
        match stmt {
            Stmt::Expression(expr) => {
                let value = self.evaluate_expr(expr, environment)?;
                Ok(Some(value))
            },
            Stmt::Print(expr) => self.evaluate_print(expr, environment),
            Stmt::Var(identifier_token, initializer) => {
                if let TokenType::Identifier(name) = &identifier_token.token_type {
                    let value = self.evaluate_expr(initializer, environment)?;
                    environment.define(name.to_string(), value);
                    Ok(None)
                } else {
                    Err(LoxError::RuntimeError(identifier_token.location.clone(), format!("Expected identifier token for var name. {}", identifier_token)))
                }
            },
            Stmt::Block(statements) => {
                environment.new_child_scope();
                self.execute_block(statements, environment)?;
                environment.destroy_child_scope();
                Ok(None)
            },
            Stmt::If(condition, left_stmt, right_stmt) => {
                match self.evaluate_expr(condition, environment)?.is_truthy() {
                    true => self.evaluate(left_stmt, environment),
                    false => {
                        match right_stmt {
                            Some(x) => self.evaluate(x, environment),
                            None => Ok(None)
                        }
                    }
                }
            }
        }
    }

    fn execute_block(&self, statements: &Vec<Stmt>, environment: &mut Environment) -> Result<(), LoxError> {
        for stmt in statements {
            self.evaluate(stmt, environment)?;
        }
        Ok(())
    }

    fn evaluate_print(&self, expr: &Expr, environment: &mut Environment) -> Result<Option<Value>, LoxError> {
        let value = self.evaluate_expr(expr, environment)?;
        println!("{}", value);
        Ok(None)
    }

    fn evaluate_expr(&self, expr: &Expr, environment: &mut Environment) -> Result<Value, LoxError> {
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
            Expr::Grouping(grouping) => self.evaluate_expr(grouping, environment),
            Expr::Unary(operator, unary) => {
                let right: Value = self.evaluate_expr(unary, environment)?;
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
                let left = self.evaluate_expr(left_expr, environment)?;
                let right = self.evaluate_expr(right_expr, environment)?;
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
            Expr::Variable(token) => {
                match &token.token_type {
                    TokenType::Identifier(variable_name) => {
                        match environment.get(variable_name) {
                            Some(val) => Ok(val),
                            None => Err(LoxError::RuntimeError(token.location.clone(), format!("Undefined variable: {}", variable_name)))
                        }
                    },
                    _ => Err(LoxError::RuntimeError(token.location.clone(), format!("Expected a variable expression. Got {}", token)))
                }
            },
            Expr::Assign(token, expr) => {
                let value = self.evaluate_expr(expr, environment)?;
                match &token.token_type {
                    TokenType::Identifier(name) => {
                        environment.assign(name, value.clone(), token.location.clone())?;
                        Ok(value)
                    },
                    _ => Err(LoxError::RuntimeError(token.location.clone(), "Assignment didn't work".to_string()))
                }
            }
            _ => Err(LoxError::Critical("Happening in the interpreter.".to_string()))
        }
    }    
}
