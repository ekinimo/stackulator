

use crate::language::env::Env;
use crate::language::eval::{ChainMap, Eval, EvalError, Flow, Values};

use malachite::Rational;
use malachite::num::arithmetic::traits::*;
#[derive(Debug, Clone,PartialEq,PartialOrd,Ord,Eq)]
pub enum Primitives {
    Add,
    Sub,
    Mult,
    Div,


    Eq,
    Ge,
    Le,
    Geq,
    Leq,
    And,
    Or,
    Not,

    Eval,
    StackSize,

    Get,
    Set,
    Concat,
    PushFirst,
    Push,
    Pop,
    PopFirst,
    Delete,
    Insert,
    Len,
    Contains,

    Intersection,
    Union,
    Difference,
    SymmetricDifference,
    Subset,
    Superset,

    IntToFloat,
    FloatToInt,
    SetToList,
    ListToSet,
}

impl Eval<Flow> for Primitives {
    fn eval(
        &self,
        _values: &mut Vec<Values>,
        _env: &Env,
        _vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
                Ok(Flow::Ok)
    }

    fn get_free_vars(&self,_vars:&mut std::collections::HashSet<usize>) {
        
    }

    fn get_vars(&self,_vars:&mut std::collections::HashSet<usize>) {
        
    }

    fn replace_vars(self,_free_vars:& std::collections::HashSet<usize>,_vars:&ChainMap)->Self {
       self
    }

}

use crate::language::parse::{ParseCtx, Parse};
use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Primitives {
    fn get_repr(&self, _context: &ParseCtx) -> String {
        match self {
            Primitives::Add                 => "add"            ,
            Primitives::Sub                 => "sub"            ,
            Primitives::Mult                => "mul"            ,
            Primitives::Div                 => "div"            ,
            Primitives::Eq                  => "eq"             ,
            Primitives::Ge                  => "ge"             ,
            Primitives::Le                  => "le"             ,
            Primitives::Geq                 => "geq"            ,
            Primitives::Leq                 => "leq"            ,
            Primitives::And                 => "and"            ,
            Primitives::Or                  => "or"             ,
            Primitives::Not                 => "not"            ,
            Primitives::Eval                => "apply"          ,
            Primitives::IntToFloat          => "i2f"            ,
            Primitives::FloatToInt          => "f2i"            ,
            Primitives::StackSize           => "stack_size"     ,
            Primitives::Get                 => "get"            ,
            Primitives::Set                 => "set"            ,
            Primitives::Concat              => "concat"         ,
            Primitives::Push                => "push_first"     ,
            Primitives::PushFirst           => "push"           ,
            Primitives::PopFirst            => "pop"            ,
            Primitives::Pop                 => "pop_first"      ,
            Primitives::Delete              => "delete"         ,
            Primitives::Insert              => "insert"         ,
            Primitives::Len                 => "len"            ,
            Primitives::Contains            => "contains"       ,
            Primitives::Intersection        => "intersect"      ,
            Primitives::Union               => "union"          ,
            Primitives::Difference          => "difference"     ,
            Primitives::SymmetricDifference => "symmetric_diff" ,
            Primitives::Subset              => "is_subset"      ,
            Primitives::Superset            => "is_superset"    ,
            Primitives::SetToList           => "s2l"            ,
            Primitives::ListToSet           => "l2s"            ,
        }
        .to_string()
    }
}

impl Parse for Primitives{
    fn parse(pairs: pest::iterators::Pair<'_, crate::language::parse::Rule>, _ctx: &mut ParseCtx) -> Self {
                        let ret = match pairs.as_str() {

                       "add"                  =>Primitives::Add                 ,
                       "sub"                  =>Primitives::Sub                 ,
                       "mul"                  =>Primitives::Mult                ,
                       "div"                  =>Primitives::Div                 ,
                       "eq"                   =>Primitives::Eq                  ,
                       "ge"                   =>Primitives::Ge                  ,
                       "le"                   =>Primitives::Le                  ,
                       "geq"                  =>Primitives::Geq                 ,
                       "leq"                  =>Primitives::Leq                 ,
                       "and"                  =>Primitives::And                 ,
                       "or"                   =>Primitives::Or                  ,
                       "not"                  =>Primitives::Not                 ,
                       "apply"                =>Primitives::Eval                ,
                       "i2f"                  =>Primitives::IntToFloat          ,
                       "f2i"                  =>Primitives::FloatToInt          ,
                       "stack_size"           =>Primitives::StackSize           ,
                       "get"                  =>Primitives::Get                 ,
                       "set"                  =>Primitives::Set                 ,
                       "concat"               =>Primitives::Concat              ,
                       "push_first"           =>Primitives::Push                ,
                       "push"                 =>Primitives::PushFirst           ,
                       "pop"                  =>Primitives::PopFirst            ,
                       "pop_first"            =>Primitives::Pop                 ,
                       "delete"               =>Primitives::Delete              ,
                       "insert"               =>Primitives::Insert              ,
                       "len"                  =>Primitives::Len                 ,
                       "contains"             =>Primitives::Contains            ,
                       "intersect"            =>Primitives::Intersection        ,
                       "union"                =>Primitives::Union               ,
                       "difference"           =>Primitives::Difference          ,
                       "symmetric_diff"       =>Primitives::SymmetricDifference ,
                       "is_subset"            =>Primitives::Subset              ,
                       "is_superset"          =>Primitives::Superset            ,
                       "s2l"                  =>Primitives::SetToList           ,
                       "l2s"                  =>Primitives::ListToSet           ,




                            _ => unreachable!(),
                };
                ret

    }
}
