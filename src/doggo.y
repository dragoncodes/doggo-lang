%start Expressions
%avoid_insert "INTEGER"
%%

Expressions -> Result<Vec<Node>, ()>:
  Expressions Expression { flattenr($1, $2) }
  | { Ok(vec![]) }
  ;

Expression -> Result<Node, ()>:
    Assignment { $1 }
  | Node { $1 }
  ;

Assignment -> Result<Node, ()>:
    'LET' 'ID' 'EQ' Expression {
        Ok(Node::Assign {
            id: $lexer.span_str($2.unwrap().span()).to_string(), 
            expr: Box::new($4?) 
        })
    }
  ;


Node -> Result<Node, ()>:
      Node 'ADD' Term {
        Ok(Node::Add{ 
          left: Box::new($1?), 
          right: Box::new($3?) 
        })
      }
    | Term { $1 }
    ;

Term -> Result<Node, ()>:
      Term 'MUL' Factor {
        Ok(Node::Multiply {  
          left: Box::new($1?), 
          right: Box::new($3?) 
        })
      }
    | Term 'DIV' Factor {
      Ok(Node::Divide {  
        left: Box::new($1?), 
        right: Box::new($3?) 
      })
    }
    | Factor { $1 }
    ;

Factor -> Result<Node, ()>:
      'LPAR' Node 'RPAR' { $2 }
    | 'INTEGER' { 
        match $1.map_err(|err| format!("Parsing Error: {}", err)) {
            Ok(s) => {
              let s = $lexer.span_str(s.span());
              match s.parse::<i32>() {
                  Ok(n_val) => Ok(Node::Number{ value: n_val }),
                  Err(_) => Err(())
              }
            }
            Err(_) => Err(())
        }
      }
    ;
%%
use crate::ast::Node;

/// Flatten `rhs` into `lhs`.
fn flattenr<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>) -> Result<Vec<T>, ()> {
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}
