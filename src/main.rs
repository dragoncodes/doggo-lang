mod ast;
mod build;
mod runner;

use crate::runner::run_ast;
use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("doggo.l");
lrpar_mod!("doggo.y");

fn build_ast(code: &str) -> Vec<ast::Node> {
    let lexer_def = doggo_l::lexerdef();
    let lexer = lexer_def.lexer(code);
    let (res, errs) = doggo_y::parse(&lexer);

    for e in errs {
        println!("{}", e.pp(&lexer, &doggo_y::token_epp));
    }

    match res {
        Some(Ok(r)) => r,
        _ => panic!("Unable to evaluate expression."),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let ast = build_ast(&args[1]);

        run_ast(&ast);
    } else {
        println!("Please provide at least one cli argument!")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn plus_operation() {
        let input = "2 + 2";

        assert_eq!("4", run_ast(&build_ast(&input)));
    }

    #[test]
    fn mult_operation() {
        let input = "2 * 2";

        assert_eq!("4", run_ast(&build_ast(&input)));
    }

    #[test]
    fn divide_operation() {
        let input = "2 / 2";

        assert_eq!("1", run_ast(&build_ast(&input)));
    }

    #[test]
    fn just_number() {
        let input = "10";

        assert_eq!("10", run_ast(&build_ast(&input)));
    }

    #[test]
    fn assign_operation() {
        let input = "let x = 25";

        assert_eq!("25", run_ast(&build_ast(&input)));
    }
}
