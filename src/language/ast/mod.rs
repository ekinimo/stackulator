use malachite::{Integer, Rational};

pub mod iftrue;
pub mod match_block;
pub mod primitives;
pub mod stack;
pub mod take;
pub mod while_block;
pub mod list;
pub mod set;
pub mod map;

#[derive(Debug, Clone,PartialEq,PartialOrd,Ord,Eq)]
pub enum Ast {
    Return,
    Break,

    PrimitiveCall(primitives::Primitives),
    Call(usize),

    While(while_block::While),
    Take(take::Take),
    IfTrue(iftrue::IfTrue),
    Match(match_block::Match),

    Bool(bool),
    Int(Integer),
    Float(Rational),

    Stack(stack::Stack),
    List(list::List),
    Set(set::Set),
    Map(map::Map),
    Var(usize),
    
}

use crate::language::ast::iftrue::IfTrue;
use crate::language::ast::match_block::Match;
use crate::language::ast::primitives::Primitives;
use crate::language::ast::stack::Stack;
use crate::language::ast::take::Take;
use crate::language::ast::while_block::While;
use crate::language::ast::list::List;
use crate::language::parse::{Parse, ParseCtx, Rule};
use std::collections::HashSet;
use std::str::FromStr;

impl Parse for Ast {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        match pairs.as_rule() {
            Rule::integer => Ast::Int(Integer::from_str(pairs.as_str()).unwrap()),
            Rule::float => Ast::Float(Rational::from_str(pairs.as_str()).unwrap()),
            Rule::bools => Ast::Bool("true" == pairs.as_str()),
            Rule::primitives => 
                Ast::PrimitiveCall( Primitives::parse(pairs, ctx)),
        
            Rule::funName => Ast::Call(ctx.insert_fun(pairs.as_str())),
            Rule::varName => Ast::Var(ctx.insert_var(pairs.as_str())),
            Rule::take => Ast::Take(Take::parse(pairs, ctx)),
            Rule::whileLoop => Ast::While(While::parse(pairs, ctx)),
            Rule::ifTrue => Ast::IfTrue(IfTrue::parse(pairs, ctx)),
            Rule::stack => Ast::Stack(Stack::parse(pairs, ctx)),
            Rule::matchBlock => Ast::Match(Match::parse(pairs, ctx)),
            Rule::ret        => Ast::Return,
            Rule::brek       => Ast::Break,
            Rule::list       => Ast::List(List::parse(pairs, ctx)),
            Rule::set       => Ast::Set(Set::parse(pairs, ctx)),
            Rule::map       => Ast::Map(Map::parse(pairs, ctx)),
            _ => unreachable!(),
        }
    }
}

use crate::language::repr::Representation;

use self::map::Map;
use self::set::Set;

use super::eval::{Eval, Flow, EvalError, Values};


impl Representation<(), ParseCtx> for Ast {
    fn get_repr(&self, context: &ParseCtx) -> String {
        match self {
            Ast::List(l) => l.get_repr(context),
            Ast::Set(l) => l.get_repr(context),
            Ast::Map(l) => l.get_repr(context),

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

impl Eval<Flow> for Ast{
    fn eval(
        &self,
        values: &mut Vec<super::eval::Values>,
        env: &super::env::Env,
        vars: &mut super::eval::ChainMap,
    ) -> Result<Flow, super::eval::EvalError> {
        match self {
                Ast::While(block) => block.eval(values, env, vars),
                Ast::IfTrue(block) => block.eval(values, env, vars),
                Ast::Take(take) => take.eval(values, env, vars),
                Ast::Match(arms) => arms.eval(values, env, vars),
                Ast::List(l) => l.eval(values, env, vars),
                Ast::Set(s) => s.eval(values, env, vars),
                Ast::Map(s) => s.eval(values, env, vars),

                Ast::Call(fun_name) => {
                    let name = env.data.get(fun_name);
                    match name {
                        Some(func) => {
                            func.eval(values, env, vars).map_err(|err| EvalError::FuncCallFail(Box::new((*fun_name,err))))
                        }

                        None => Err(EvalError::UndefinedCall(*fun_name)),
                    }
                }
                Ast::PrimitiveCall(p) => p.eval(values, env, vars),
                Ast::Var(var) => match vars.lookup(var) {
                    Some(val) => {values.push(val); Ok(Flow::Ok)},
                    None => Err(EvalError::UndefinedVariable(*var)),
                },
                Ast::Float(f) => {values.push(Values::Float(f.clone())); Ok(Flow::Ok) },
                Ast::Int(i) => {values.push(Values::Int(i.clone())); Ok(Flow::Ok)},
                Ast::Bool(b) => {values.push(Values::Bool(*b));Ok(Flow::Ok)},
                Ast::Stack(s) => {
                    let mut free_vars = HashSet::new();
                    s.get_free_vars(&mut free_vars);
                    let s = s.clone().replace_vars(&mut free_vars, vars);
                    values.push(Values::Stack(s));
                    Ok(Flow::Ok)
                },
                Ast::Return => {Ok(Flow::Ret)},
                Ast::Break => Ok(Flow::Break),
                
                
            }
    }

    fn get_free_vars(&self,vars:&mut std::collections::HashSet<usize>) {
        match self {
            Ast::While(w) => w.get_free_vars(vars),
            Ast::Take(w) => w.get_free_vars(vars),
            Ast::Stack(w) => w.get_free_vars(vars),
            Ast::IfTrue(w) => w.get_free_vars(vars),
            Ast::Match(w) => w.get_free_vars(vars),
            Ast::Call(_) => (),
            Ast::Var(w) => {vars.insert(*w);},
            Ast::PrimitiveCall(w) => w.get_free_vars(vars),
            Ast::Float(_) => (),
            Ast::Int(_) => (),
            Ast::Bool(_) =>(),
            Ast::List(w) => w.get_free_vars(vars),
            Ast::Set(w) => w.get_free_vars(vars),
            Ast::Map(w) => w.get_free_vars(vars),
            Ast::Return => (),
            Ast::Break => (),
        }
    }

    fn get_vars(&self,vars:&mut std::collections::HashSet<usize>) {
        match self {
            Ast::While(w) => w.get_vars(vars),
            Ast::Take(w) => w.get_vars(vars),
            Ast::Stack(w) => w.get_vars(vars),
            Ast::IfTrue(w) => w.get_vars(vars),
            Ast::Match(w) => w.get_vars(vars),
            Ast::Call(_) => (),
            Ast::Var(w) => {vars.insert(*w);},
            Ast::PrimitiveCall(w) => w.get_vars(vars),
            Ast::Float(_) => (),
            Ast::Int(_) => (),
            Ast::Bool(_) =>(),
            Ast::List(w) => w.get_vars(vars),
            Ast::Set(w) => w.get_vars(vars),
            Ast::Map(w) => w.get_vars(vars),
            Ast::Return => (),
            Ast::Break => (),
        }
    }

    fn replace_vars(self,free_vars:& std::collections::HashSet<usize>,vars:&super::eval::ChainMap)->Self {
        match self {
            Ast::While(x) => Ast::While(x.replace_vars(free_vars,vars)),
            Ast::Take(x) => Ast::Take(x.replace_vars(free_vars,vars)),
            Ast::Stack(x) => Ast::Stack(x.replace_vars(free_vars,vars)),
            Ast::IfTrue(x) => Ast::IfTrue(x.replace_vars(free_vars,vars)),
            Ast::Match(x) => Ast::Match(x.replace_vars(free_vars,vars)),
            Ast::Var(i) if free_vars.contains(&i) => {
                match vars.lookup(&i) {
                    Some(val) => {
                        val.into()
                    },
                    None => {
                        //TODO we should raise an error here
                        unreachable!("this shouldnt happen i gusss / unknown variable ")
                    },
                }
            },
            Ast::PrimitiveCall(x) => Ast::PrimitiveCall(x.replace_vars(free_vars,vars)),
            Ast::List(x) => Ast::List(x.replace_vars(free_vars,vars)),
            Ast::Set(x) => Ast::Set(x.replace_vars(free_vars,vars)),
            Ast::Map(x) => Ast::Map(x.replace_vars(free_vars,vars)),
            _ => self
    }

    
    }
}

impl From<Values> for Ast{
    fn from(value: Values) -> Self {
        match value{
            Values::Float(i) => Ast::Float(i),
            Values::Int(i) => Ast::Int(i),
            Values::Bool(i) => Ast::Bool(i),
            Values::Stack(i) => Ast::Stack(i),
            Values::List(i) => {
                Ast::List( 
                                Stack{
                                   elems:  i.into_iter().map(|x| x.into()).collect(),
                                }.into()
                )
            },
            Values::Set(i) => {
                Ast::List( 
                    Stack{
                        elems:  i.into_iter().map(|x| x.into()).collect(),
                    }.into()
                )
            },
            Values::Map(i) => Ast::List( 
                Stack{
                    elems:  i.into_iter().map(|(x,y)|{
                        let d = [x,y].into_iter().collect();
                        Values::List(d).into()
                    }).collect() ,
                }.into()
            ),
        }
    }
}
