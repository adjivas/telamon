pub use super::utils::RcStr;

pub use super::telamon_gen::lexer::{Lexer, Spanned, Position};
pub use super::telamon_gen::parser;
pub use super::telamon_gen::ir;
pub use super::telamon_gen::ast::*;

#[test]
fn redefinition() {
    assert_eq!(parser::parse_ast(Lexer::from(
        b"set Foo:
          end
          
          set Foo:
          end".to_vec())).unwrap().type_check().err(),
        Some(TypeError::Redefinition(Spanned {
            beg: Position { line: 0, column: 4},
            end: Position { line: 0, column: 7},
            data: Hint::Set,
        }, Spanned {
            beg: Position { line: 3, column: 14},
            end: Position { line: 3, column: 17},
            data:  String::from("Foo"),
        }))
    );
    assert_eq!(parser::parse_ast(Lexer::from(
        b"define enum foo():
          end
          
          define enum foo():
          end".to_vec())).unwrap().type_check().err(),
        Some(TypeError::Redefinition(Spanned {
            beg: Position { line: 0, column: 12},
            end: Position { line: 0, column: 15},
            data: Hint::Enum,
        }, Spanned {
            beg: Position { line: 3, column: 22},
            end: Position { line: 3, column: 25},
            data: String::from("foo"),
        }))
    );
    assert_eq!(parser::parse_ast(Lexer::from(
        b"define integer foo($myarg in Arg): \"mycode\"
          end
          define integer foo($myarg in Arg): \"mycode\"
          end".to_vec())).unwrap().type_check().err(),
        Some(TypeError::Redefinition(Spanned {
            beg: Position { line: 0, column: 15},
            end: Position { line: 0, column: 18},
            data: Hint::Integer,
        }, Spanned {
            beg: Position { line: 2, column: 25},
            end: Position { line: 2, column: 28},
            data:  String::from("foo"),
        }))
    );
    assert!(parser::parse_ast(Lexer::from(
        b"set Foo:
            item_type = \"ir::inst::Obj\"
            id_type = \"ir::inst::Id\"
            item_getter = \"ir::inst::get($fun, $id)\"
            id_getter = \"ir::inst::Obj::id($item)\"
            iterator = \"ir::inst::iter($fun)\"
            var_prefix = \"inst\"
            new_objs = \"$objs.inst\"
          end
          
          define enum foo():
          end".to_vec())).unwrap().type_check().is_ok()
    );
}