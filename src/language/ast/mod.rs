use malachite::{Integer, Rational};

pub mod iftrue;
pub mod match_block;
pub mod primitives;
pub mod stack;
pub mod take;
pub mod while_block;

#[derive(Debug, Clone)]
pub enum Ast {
    While(while_block::While),
    Take(take::Take),
    Stack(stack::Stack),
    IfTrue(iftrue::IfTrue),
    Match(match_block::Match),
    Call(usize),
    Var(usize),
    PrimitiveCall(primitives::Primitives),
    Float(Rational),
    Int(Integer),
    Bool(bool),
    Return,
    Break,
}

use crate::language::ast::iftrue::IfTrue;
use crate::language::ast::match_block::Match;
use crate::language::ast::primitives::Primitives;
use crate::language::ast::stack::Stack;
use crate::language::ast::take::Take;
use crate::language::ast::while_block::While;
use crate::language::parse::{Parse, ParseCtx, Rule};
use std::str::FromStr;

impl Parse for Ast {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        match pairs.as_rule() {
            Rule::integer => Ast::Int(Integer::from_str(pairs.as_str()).unwrap()),
            Rule::float => Ast::Float(Rational::from_str(pairs.as_str()).unwrap()),
            Rule::bools => Ast::Bool("true" == pairs.as_str()),
            Rule::primitives => {
                let ret = match pairs.as_str() {
                    "add" => Primitives::Add,
                    "sub" => Primitives::Sub,
                    "mult" => Primitives::Mult,
                    "div" => Primitives::Div,
                    "and" => Primitives::And,
                    "or" => Primitives::Or,
                    "eq" => Primitives::Eq,
                    "geq" => Primitives::Geq,
                    "leq" => Primitives::Leq,
                    "ge" => Primitives::Ge,
                    "le" => Primitives::Le,
                    "not" => Primitives::Not,
                    "i2f" => Primitives::IntToFloat,
                    "f2i" => Primitives::FloatToInt,
                    "apply" => Primitives::Eval,
                    _ => unreachable!(),
                };
                Ast::PrimitiveCall(ret)
            }
            Rule::funName => Ast::Call(ctx.insert_fun(pairs.as_str())),
            Rule::varName => Ast::Var(ctx.insert_var(pairs.as_str())),
            Rule::take => Ast::Take(Take::parse(pairs, ctx)),
            Rule::whileLoop => Ast::While(While::parse(pairs, ctx)),
            Rule::ifTrue => Ast::IfTrue(IfTrue::parse(pairs, ctx)),
            Rule::stack => Ast::Stack(Stack::parse(pairs, ctx)),
            Rule::matchBlock => Ast::Match(Match::parse(pairs, ctx)),
            Rule::ret        => Ast::Return,
            Rule::brek       => Ast::Break,
            _ => unreachable!(),
        }
    }
}

use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Ast {
    fn get_repr(&self, context: &ParseCtx) -> String {
        match self {
            Ast::While(w) => w.get_repr(context),
            Ast::Take(s) => s.get_repr(context),
            Ast::Stack(s) => s.get_repr(context),
            Ast::IfTrue(s) => s.get_repr(context),
            Ast::Match(s) => s.get_repr(context),
            Ast::Call(x) => context.lookup_call_name(*x),
            Ast::Var(x) => context.lookup_var_name(*x),
            Ast::PrimitiveCall(p) => p.get_repr(context),
            Ast::Float(i) => format!("{i}"),
            Ast::Int(i) => format!("{i}"),
            Ast::Bool(true) => "false".to_string(),
            Ast::Bool(false) => "true".to_string(),
            Ast::Break => "break".to_string(),
            Ast::Return => "return".to_string(),
        }
    }
}
