pub use super::utils::RcStr;

pub use super::telamon_gen::lexer::{Lexer, Spanned, Position, LexerPosition};
pub use super::telamon_gen::parser;
pub use super::telamon_gen::ir;
pub use super::telamon_gen::ast::*;

/// Undefined
#[cfg(test)]
mod undefined {
    pub use super::*;

    #[test]
    fn set() {
        assert_eq!(parser::parse_ast(Lexer::new(
            b"set Instruction:
                  item_type = \"ir::inst::Obj\"
                  id_type = \"ir::inst::Id\"
                  item_getter = \"ir::inst::get($fun, $id)\"
                  id_getter = \"ir::inst::Obj::id($item)\"
                  iterator = \"ir::inst::iter($fun)\"
                  var_prefix = \"inst\"
                  new_objs = \"$objs.inst\"
              end

              define enum foo($inst in Instruction, $op in Operand($inst)):
                  value A:
                  value B:
              end

              define enum bar($inst in Instruction):
                  value C:
                  value D:
              end

              require forall $inst in Instruction:
                  forall $op in Operand($inst):
                       bar($inst) is C || foo($inst, $op) is A
            ".to_vec())).unwrap().type_check().err(),
            Some(TypeError::Undefined {
                object_name: Spanned {
                    beg: Position::new_optional(LexerPosition::new(10, 26), None),
                    end: Position::new_optional(LexerPosition::new(10, 29), None),
                    data: String::from("Operand"),
                }
            })
        );
    }

    #[test]
    fn enumeration() {
        assert_eq!(parser::parse_ast(Lexer::new(
            b"set Instruction:
                  item_type = \"ir::inst::Obj\"
                  id_type = \"ir::inst::Id\"
                  item_getter = \"ir::inst::get($fun, $id)\"
                  id_getter = \"ir::inst::Obj::id($item)\"
                  iterator = \"ir::inst::iter($fun)\"
                  var_prefix = \"inst\"
                  new_objs = \"$objs.inst\"
              end

              set Operand($inst in Instruction):
                  item_type = \"ir::operand::Obj\"
                  id_type = \"ir::operand::Id\"
                  item_getter = \"ir::operand::get($fun, $inst, $id)\"
                  id_getter = \"$item.id()\"
                  iterator = \"ir::operand::iter($fun, ir::inst::Obj::id($inst))\"
                  var_prefix = \"op\"
                  new_objs = \"$objs.operand\"
              end

              define enum bar($inst in Instruction):
                  value C:
                  value D:
              end

              require forall $inst in Instruction:
                  forall $op in Operand($inst):
                       bar($inst) is C || foo($inst, $op) is A
            ".to_vec())).unwrap().type_check().err(),
            Some(TypeError::Undefined {
                object_name: Spanned {
                    beg: Position::new_optional(LexerPosition::new(27, 42), None),
                    end: Position::new_optional(LexerPosition::new(27, 45), None),
                    data: String::from("foo"),
                }
            })
        );
    }
}
