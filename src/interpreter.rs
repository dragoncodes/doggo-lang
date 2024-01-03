use std::collections::HashMap;

use crate::ast;
use itertools::*;

pub fn run_ast(ast: &Vec<ast::Node>) -> Vec<String> {
    let mut stack_allocated_variables: HashMap<String, i32> = HashMap::new();

    ast.iter()
        .map(|node| match node {
            ast::Node::Add { left, right } => {
                let left = resolve_node(left, &stack_allocated_variables);
                let right = resolve_node(right, &stack_allocated_variables);

                println!("Left = {}, Right = {}", left, right);

                format!("{}", left + right)
            }

            ast::Node::Multiply { left, right } => {
                let left = resolve_node(left, &stack_allocated_variables);
                let right = resolve_node(right, &stack_allocated_variables);

                format!("{}", left * right)
            }
            ast::Node::Divide { left, right } => {
                let left = resolve_node(left, &stack_allocated_variables);
                let right = resolve_node(right, &stack_allocated_variables);

                if right == 0 {
                    panic!("Cannot divide by zero!");
                }

                format!("{}", left / right)
            }

            ast::Node::Number { value } => format!("{}", value),

            ast::Node::Assign { id, expr } => {
                let expr = resolve_node(expr, &stack_allocated_variables);

                stack_allocated_variables.insert(id.clone(), expr);

                format!("{}", expr)
            }

            ast::Node::Reference { id } => {
                let value = stack_allocated_variables
                    .get(id)
                    .expect(format!("Reference {} not recognised", id).as_str());

                format!("{value}")
            }
        })
        .collect_vec()
}

fn resolve_node(node: &ast::Node, stack_allocated_variables: &HashMap<String, i32>) -> i32 {
    match node {
        ast::Node::Add { left, right } => {
            resolve_node(left, stack_allocated_variables)
                + resolve_node(right, stack_allocated_variables)
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
