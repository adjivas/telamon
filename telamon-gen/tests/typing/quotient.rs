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
    fn print() {
        println!("{:?}",
               parser::parse_ast(Lexer::new(b"
        set BasicBlocks:
            item_type = \"ir::BasicBlock\"
            id_type = \"ir::BBId\"
            item_getter = \"$fun.block($id)\"
            id_getter = \"$item.bb_id()\"
            iterator = \"$fun.blocks()\"
            var_prefix = \"bb\"
            new_objs = \"$objs.basic_blocks\"
        end

        set Instructions subsetof BasicBlocks:
            item_type = \"ir::Instruction\"
            id_type = \"ir::InstId\"
            item_getter = \"$fun.inst($id)\"
            id_getter = \"$item.id()\"
            iterator = \"$fun.insts()\"
            var_prefix = \"inst\"
            from_superset = \"$item.as_inst()\"
            new_objs = \"$objs.instructions\"
        end

        set Dimensions subsetof BasicBlocks:
            disjoint: Instructions
            item_type = \"ir::Dimension\"
            id_type = \"ir::DimId\"
            item_getter = \"$fun.dim($id)\"
            id_getter = \"$item.id()\"
            iterator = \"$fun.dims()\"
            var_prefix = \"dim\"
            from_superset = \"$item.as_dim()\"
            new_objs = \"$objs.dimensions\"
        end

        quotient IterationDims($inst in Instructions) of $dim in Dimensions:
            is_iteration_dim = order($dim, $inst) is OUTER / order is MERGED
            item_type = \"ir::Dimension\"
            id_type = \"ir::DimId\"
            item_getter = \"$fun.dim($id)\"
            id_getter = \"$item.id()\"
            iterator = \"$inst.iteration_dims().iter().map(|&d| $fun.dim(d))\"
            var_prefix = \"iter_dim\"
            new_objs = \"$objs.iteration_dims\"
            from_superset = \"$inst.iteration_dims().get(&$item.id()).map(|_| $item)\"
            reverse forall $dim in Dimensions = \"$dim.iterated().map(|id| $fun.inst(id))\"
            add_to_set = \"::search_space::add_iteration_dim($fun, $inst, $item)\"
        end

        define enum order($lhs in BasicBlocks, $rhs in BasicBlocks):
            antisymmetric:
            BEFORE -> AFTER
            INNER -> OUTER
            value BEFORE:
            value AFTER:
            value INNER:
            value OUTER:
            value MERGED:
            alias NESTED = INNER | OUTER:
            alias ORDERED = BEFORE | AFTER:
        end

        require forall $dim in Dimensions:
            forall $init in Instructions:
                forall $reduce in Instructions:
                    \"!$reduce.is_reduction_common_dim($init.id(), $dim.id())\"
                        || is_iteration_dim($reduce, $dim) is FALSE
                        || order($dim, $init) is OUTER
            ".to_vec())).unwrap().type_check().err()
        );
    }
}
