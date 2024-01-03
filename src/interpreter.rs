use std::collections::HashMap;

use crate::ast;
use itertools::*;

pub struct Interpreter {
    stack_allocated_variables: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            stack_allocated_variables: HashMap::new(),
        }
    }

    pub fn run_node(self: &mut Self, node: &ast::Node) -> String {
        match node {
            ast::Node::Add { left, right } => {
                let left = resolve_node(left, &self.stack_allocated_variables);
                let right = resolve_node(right, &self.stack_allocated_variables);

                format!("{}", left + right)
            }

            ast::Node::Substract { left, right } => {
                let left = resolve_node(left, &self.stack_allocated_variables);

                let right = resolve_node(right, &self.stack_allocated_variables);

                format!("{}", left - right)
            }

            ast::Node::Multiply { left, right } => {
                let left = resolve_node(left, &self.stack_allocated_variables);
                let right = resolve_node(right, &self.stack_allocated_variables);

                format!("{}", left * right)
            }
            ast::Node::Divide { left, right } => {
                let left = resolve_node(left, &self.stack_allocated_variables);
                let right = resolve_node(right, &self.stack_allocated_variables);

                if right == 0 {
                    panic!("Cannot divide by zero!");
                }

                format!("{}", left / right)
            }

            ast::Node::Number { value } => format!("{}", value),

            ast::Node::Assign { id, expr } => {
                let expr = resolve_node(expr, &self.stack_allocated_variables);

                self.stack_allocated_variables.insert(id.clone(), expr);

                format!("{}", expr)
            }

            ast::Node::Reference { id } => {
                let value = self
                    .stack_allocated_variables
                    .get(id)
                    .expect(format!("Reference {} not recognised", id).as_str());

                format!("{value}")
            }
        }
    }

    pub fn run_nodes(self: &mut Self, nodes: &Vec<ast::Node>) -> Vec<String> {
        nodes.iter().map(|node| self.run_node(node)).collect_vec()
    }
}

fn resolve_node(node: &ast::Node, stack_allocated_variables: &HashMap<String, i32>) -> i32 {
    match node {
        ast::Node::Add { left, right } => {
            resolve_node(left, stack_allocated_variables)
                + resolve_node(right, stack_allocated_variables)
        }
        ast::Node::Substract { left, right } => {
            resolve_node(left, stack_allocated_variables)
                - resolve_node(right, stack_allocated_variables)
        }
        ast::Node::Multiply { left, right } => {
            resolve_node(left, stack_allocated_variables)
                * resolve_node(right, stack_allocated_variables)
        }
        ast::Node::Divide { left, right } => {
            resolve_node(left, stack_allocated_variables)
                / resolve_node(right, stack_allocated_variables)
        }
        ast::Node::Assign { id, expr } => resolve_node(expr, stack_allocated_variables),
        ast::Node::Reference { id } => {
            let val = stack_allocated_variables
                .get(id)
                .expect(format!("Unrecognised variable {id}").as_str());

            *val
        }
        ast::Node::Number { value } => *value,
    }
}
