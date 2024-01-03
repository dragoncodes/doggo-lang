mod ast;
mod build;
mod interpreter;

use crate::interpreter::run_ast;
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

        if let Some(last_expr) = run_ast(&ast).iter().last() {
            println!("{last_expr}")
        }
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

        assert_eq!(vec!["4"], run_ast(&build_ast(&input)));
    }

    #[test]
    fn plus_operation_multiline() {
        let input = r"
2 + 2            
10 + 10
-6 + 10
";

        assert_eq!(vec!["4", "20", "4"], run_ast(&build_ast(&input)));
    }

    #[test]
    fn mult_operation() {
        let input = "2 * 2";

        assert_eq!(vec!["4"], run_ast(&build_ast(&input)));
    }

    #[test]
    fn divide_operation() {
        let input = "2 / 2";

        assert_eq!(vec!["1"], run_ast(&build_ast(&input)));
    }

    #[test]
    fn just_number() {
        let input = "10";

        assert_eq!(vec!["10"], run_ast(&build_ast(&input)));
    }

    #[test]
    fn assign_operation() {
        let input = "let x = 25";

        assert_eq!(vec!["25"], run_ast(&build_ast(&input)));
    }

    #[test]
    fn reference_var() {
        let input = r"let x = 25
         x
        ";

        dbg!("{:?}", build_ast(&input));

        assert_eq!(vec!["25", "25"], run_ast(&build_ast(&input)));
    }

    #[test]
    fn reference_var_with_usage() {
        let input = r"
let x = 25
x + 5";

        dbg!("{:?}", build_ast(&input));

        assert_eq!(vec!["25", "30"], run_ast(&build_ast(&input)));
    }
}
