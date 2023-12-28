use crate::ast;
use itertools::*;

pub fn run_ast(ast: &Vec<ast::Node>) -> String {
    let mut assignments = Vec::new();

    ast.iter()
        .map(|node| match node {
            ast::Node::Add { left, right } => {
                let left = resolve_node(left);
                let right = resolve_node(right);

                format!("{}", left + right)
            }

            ast::Node::Multiply { left, right } => {
                let left = resolve_node(left);
                let right = resolve_node(right);

                format!("{}", left * right)
            }
            ast::Node::Divide { left, right } => {
                let left = resolve_node(left);
                let right = resolve_node(right);

                if right == 0 {
                    panic!("Cannot divide by zero!");
                }

                format!("{}", left / right)
            }

            ast::Node::Number { value } => format!("{}", value),

            ast::Node::Assign { id, expr } => {
                let expr = resolve_node(expr);

                assignments.push((id.clone(), expr));

                format!("{}", expr)
            }
        })
        .join("\n")
}

fn resolve_node(node: &ast::Node) -> i32 {
    match node {
        ast::Node::Add { left, right } => resolve_node(left) + resolve_node(right),
        ast::Node::Multiply { left, right } => resolve_node(left) * resolve_node(right),
        ast::Node::Divide { left, right } => resolve_node(left) / resolve_node(right),
        ast::Node::Assign { id, expr } => resolve_node(expr),
        ast::Node::Number { value } => *value,
    }
}
