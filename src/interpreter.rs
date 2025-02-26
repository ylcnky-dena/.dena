use crate::environment::Environment;
use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable reference to environment"),
                    )?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable ref to env"),
                    )?;
                    println!("\"{}\"", value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable ref to env"),
                    )?;

                    Rc::get_mut(&mut self.environment)
                        .expect("Could not get mutable ref to env")
                        .define(name.lexeme, value);
                }
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());
                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(new_environment);
                    let block_result = self.interpret(statements);
                    self.environment = old_environment;

                    block_result?;
                }
                Stmt::IfStmt {
                    predicate,
                    then,
                    els,
                } => {
                    let truth_value = predicate.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable ref to env"),
                    )?;
                    if truth_value.is_truthy() == LiteralValue::True {
                        self.interpret(vec![*then])?;
                    } else if let Some(els_stmt) = els {
                        self.interpret(vec![*els_stmt])?;
                    }
                }
            };
        }

        Ok(())
    }
}

