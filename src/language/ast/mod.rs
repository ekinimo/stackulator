use malachite::{Integer, Rational};

pub mod iftrue;
pub mod list;
pub mod map;
pub mod match_block;
pub mod primitives;
pub mod set;
pub mod stack;
pub mod take;
pub mod while_block;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Type {
    Bool,
    Integer,
    Float,
    Stack,
    List,
    Set,
    Map,

    CustomType(usize),
    GenericTyp(usize),
}

impl Parse for Type {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        dbg!(pairs.as_str());
        match pairs.as_rule() {
            Rule::typName => Type::CustomType(ctx.insert_type(pairs.as_str())),
            Rule::genericName => Type::GenericTyp(ctx.insert_type(pairs.as_str())),
            Rule::primTyps => match pairs.as_str() {
                "Bool" => Type::Bool,
                "Int" => Type::Integer,
                "Rat" => Type::Float,
                "Stack" => Type::Stack,
                "List" => Type::List,
                "Set" => Type::Set,
                "Map" => Type::Map,
                _ => unreachable!(),
            },
            x => {
                dbg!(x);
                unreachable!();
            }
        }
    }
}

impl Type {
    pub fn match_values(&self, val: &Values, generics: &mut HashMap<usize, Values>) -> bool {
        match (self, val) {
            (Type::Bool, Values::Bool(_))
            | (Type::Integer, Values::Int(_))
            | (Type::Float, Values::Float(_))
            | (Type::Stack, Values::Stack(_))
            | (Type::List, Values::List(_))
            | (Type::Set, Values::Set(_))
            | (Type::Map, Values::Map(_)) => true,
            (Type::CustomType(name1), Values::Custom { name, .. }) => name1 == name,
            (Type::GenericTyp(name), other) => match generics.entry(*name) {
                std::collections::hash_map::Entry::Occupied(x) => {
                    x.get().get_real_type() == other.get_real_type()
                }
                std::collections::hash_map::Entry::Vacant(pos) => {
                    pos.insert(other.clone());
                    true
                }
            },
            x => {
                dbg!(x);
                false
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Ast {
    Return,
    Break,

    PrimitiveCall(primitives::Primitives),
    Call(usize),
    TypeCall(
        usize,         //name,
        Option<usize>, // possible tag),
        Stack,
    ),

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
use crate::language::ast::list::List;
use crate::language::ast::match_block::Match;
use crate::language::ast::primitives::Primitives;
use crate::language::ast::stack::Stack;
use crate::language::ast::take::Take;
use crate::language::ast::while_block::While;
use crate::language::parse::{Parse, ParseCtx, Rule};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;

impl Parse for Ast {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        match pairs.as_rule() {
            Rule::integer => Ast::Int(Integer::from_str(pairs.as_str()).unwrap()),
            Rule::float => Ast::Float(Rational::from_str(pairs.as_str()).unwrap()),
            Rule::bools => Ast::Bool("true" == pairs.as_str()),
            Rule::primitives => Ast::PrimitiveCall(Primitives::parse(pairs, ctx)),

            Rule::funName => Ast::Call(ctx.insert_fun(pairs.as_str())),
            Rule::varName => Ast::Var(ctx.insert_var(pairs.as_str())),
            Rule::take => Ast::Take(Take::parse(pairs, ctx)),
            Rule::whileLoop => Ast::While(While::parse(pairs, ctx)),
            Rule::ifTrue => Ast::IfTrue(IfTrue::parse(pairs, ctx)),
            Rule::stack => Ast::Stack(Stack::parse(pairs, ctx)),
            Rule::matchBlock => Ast::Match(Match::parse(pairs, ctx)),
            Rule::ret => Ast::Return,
            Rule::brek => Ast::Break,
            Rule::list => Ast::List(List::parse(pairs, ctx)),
            Rule::set => Ast::Set(Set::parse(pairs, ctx)),
            Rule::map => Ast::Map(Map::parse(pairs, ctx)),
            Rule::variantInst => {
                let mut def = pairs.into_inner();
                let mut names = def.next().unwrap().into_inner();
                let type_name = ctx.insert_type(names.next().unwrap().as_str());
                let tag_name = ctx.insert_tag(names.next().unwrap().as_str());
                let elements = Stack {
                    elems: def.map(|x| Ast::parse(x, ctx)).collect::<Arc<_>>(),
                };
                Ast::TypeCall(type_name, Some(tag_name), elements)
            }
            Rule::typeInst => {
                let mut def = pairs.into_inner();
                let type_name = ctx.insert_type(def.next().unwrap().as_str());
                let elements = Stack {
                    elems: def.map(|x| Ast::parse(x, ctx)).collect::<Arc<_>>(),
                };
                dbg!(&elements);
                Ast::TypeCall(type_name, None, elements)
            }

            x => {dbg!(x); unreachable!() },
        }
    }
}

use crate::language::repr::Representation;

use self::map::Map;
use self::set::Set;

use super::eval::{Eval, EvalError, Flow, Values};

impl Representation<(), ParseCtx> for Type {
    fn get_repr(&self, context: &ParseCtx) -> String {
        match self {
            Type::Bool => "Bool".to_string(),
            Type::Integer => "Int".to_string(),
            Type::Float => "Rat".to_string(),
            Type::Stack => "Stack".to_string(),
            Type::List => "List".to_string(),
            Type::Set => "Set".to_string(),
            Type::Map => "Map".to_string(),
            Type::CustomType(t) => context.lookup_type_name(*t),
            Type::GenericTyp(t) => context.lookup_type_name(*t),
        }
    }
}

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
            Ast::TypeCall(name, Some(variant), data) => {
                let mut ret = data.get_repr(context);
                ret.pop();
                format!("{name}::{variant}( {} )", &ret[1..])
            }
            Ast::TypeCall(name, None, data) => {
                let mut ret = data.get_repr(context);
                ret.pop();

                format!("{name}( {} )", &ret[1..])
            }
        }
    }
}

impl Eval<Flow> for Ast {
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
                {
                    let name = env.data.get(fun_name);

                    match &name {
                        Some(func) => {
                            return func
                                .eval(values, env, vars)
                                .map_err(|err| EvalError::FuncCallFail(Box::new((*fun_name, err))))
                        }

                        None => (),
                    };
                }
                let arity = env.protocol_arity.get(fun_name);

                match arity {
                    Some(arity) => {
                        if values.len() < arity.0 {
                            dbg!(values.len());
                            dbg!(arity);

                            return Err(EvalError::Underflow);
                        }
                        let mut temp = vec![];

                        for i in 0..(arity.0) {
                            temp.push(values[values.len()-i-1].to_owned());
                        }
                        temp.reverse();
                        let arms = env.protocol_data.get(fun_name).unwrap();
                        for tmp in arms {
                            let (types, act) = tmp;
                            if types.iter().zip(temp.clone().into_iter()).all(|(a, b)| match (a, b) {
                                (Type::Bool, Values::Bool(_)) => true,
                                (Type::Integer, Values::Int(_)) => true,
                                (Type::Float, Values::Float(_)) => true,
                                (Type::Stack, Values::Stack(_)) => true,
                                (Type::List, Values::List(_)) => true,
                                (Type::Set, Values::Set(_)) => true,
                                (Type::Map, Values::Map(_)) => true,
                                (Type::CustomType(name1),Values::Custom { name, .. }) if name1 == &name => true,
                                (Type::GenericTyp(_), _) => true,
                                (a,b) => {dbg!((a,b));false},
                            }){
                                let (_ , act) = act;
                                return act.eval(values,env,vars)
                            }
                        }
                        dbg!("UNDERFLOOOOOOOW");
                        Err(EvalError::Underflow)
                    }

                    None => Err(EvalError::UndefinedCall(*fun_name)),
                }
            }
            Ast::PrimitiveCall(p) => p.eval(values, env, vars),
            Ast::Var(var) => match vars.lookup(var) {
                Some(val) => {
                    values.push(val);
                    Ok(Flow::Ok)
                }
                None => Err(EvalError::UndefinedVariable(*var)),
            },
            Ast::Float(f) => {
                values.push(Values::Float(f.clone()));
                Ok(Flow::Ok)
            }
            Ast::Int(i) => {
                values.push(Values::Int(i.clone()));
                Ok(Flow::Ok)
            }
            Ast::Bool(b) => {
                values.push(Values::Bool(*b));
                Ok(Flow::Ok)
            }
            Ast::Stack(s) => {
                let mut free_vars = HashSet::new();
                s.get_free_vars(&mut free_vars);
                let s = s.clone().replace_vars(&mut free_vars, vars);
                values.push(Values::Stack(s));
                Ok(Flow::Ok)
            }
            Ast::Return => Ok(Flow::Ret),
            Ast::Break => Ok(Flow::Break),

            Ast::TypeCall(typ_name, tag, constructor) => {
                let mut temp = vec![];
                dbg!(&constructor);
                constructor.eval(&mut temp, env, vars)?;
                match env.typ_data.get(&(*typ_name, *tag)) {
                    Some(types) if types.len() == temp.len() => {
                        let ret;
                        {
                            let mut generics = HashMap::new();
                            ret = types
                                .iter()
                                .zip(temp.iter())
                                .all(|(ty, val)| ty.match_values(val, &mut generics));
                        }
                        if ret {
                            if temp.is_empty() {
                                values.push(Values::Custom {
                                    name: *typ_name,
                                    tag: *tag,
                                    values: None,
                                });
                            } else {
                                values.push(Values::Custom {
                                    name: *typ_name,
                                    tag: *tag,
                                    values: Some(temp.into_iter().collect()),
                                });
                            }
                            Ok(Flow::Ok)
                        } else {
                            dbg!(&types);
                            dbg!(&temp);
                            Err(EvalError::TypeDoesntExist(*typ_name))
                        }
                    }
                    Some(types) => {
                        dbg!(&types);
                        dbg!(&temp);
                        Err(EvalError::TypeConstructorLenMismatch(
                            *typ_name,
                            temp.len(),
                            types.len(),
                        ))
                    }
                    None => {
                        dbg!(&temp);
                        dbg!(&(*typ_name, *tag));
                        Err(EvalError::TypeDoesntExist(*typ_name))
                    }
                }
            }
        }
    }

    fn get_free_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        match self {
            Ast::While(w) => w.get_free_vars(vars),
            Ast::Take(w) => w.get_free_vars(vars),
            Ast::Stack(w) => w.get_free_vars(vars),
            Ast::IfTrue(w) => w.get_free_vars(vars),
            Ast::Match(w) => w.get_free_vars(vars),
            Ast::Call(_) => (),
            Ast::Var(w) => {
                vars.insert(*w);
            }
            Ast::PrimitiveCall(w) => w.get_free_vars(vars),
            Ast::Float(_) => (),
            Ast::Int(_) => (),
            Ast::Bool(_) => (),
            Ast::List(w) => w.get_free_vars(vars),
            Ast::Set(w) => w.get_free_vars(vars),
            Ast::Map(w) => w.get_free_vars(vars),
            Ast::Return => (),
            Ast::Break => (),
            Ast::TypeCall(_, _, w) => w.get_free_vars(vars),
        }
    }

    fn get_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        match self {
            Ast::TypeCall(_, _, w) => w.get_vars(vars),
            Ast::While(w) => w.get_vars(vars),
            Ast::Take(w) => w.get_vars(vars),
            Ast::Stack(w) => w.get_vars(vars),
            Ast::IfTrue(w) => w.get_vars(vars),
            Ast::Match(w) => w.get_vars(vars),
            Ast::Call(_) => (),
            Ast::Var(w) => {
                vars.insert(*w);
            }
            Ast::PrimitiveCall(w) => w.get_vars(vars),
            Ast::Float(_) => (),
            Ast::Int(_) => (),
            Ast::Bool(_) => (),
            Ast::List(w) => w.get_vars(vars),
            Ast::Set(w) => w.get_vars(vars),
            Ast::Map(w) => w.get_vars(vars),
            Ast::Return => (),
            Ast::Break => (),
        }
    }

    fn replace_vars(
        self,
        free_vars: &std::collections::HashSet<usize>,
        vars: &super::eval::ChainMap,
    ) -> Self {
        match self {
            Ast::TypeCall(name, variant, x) => {
                Ast::TypeCall(name, variant, x.replace_vars(free_vars, vars))
            }
            Ast::While(x) => Ast::While(x.replace_vars(free_vars, vars)),
            Ast::Take(x) => Ast::Take(x.replace_vars(free_vars, vars)),
            Ast::Stack(x) => Ast::Stack(x.replace_vars(free_vars, vars)),
            Ast::IfTrue(x) => Ast::IfTrue(x.replace_vars(free_vars, vars)),
            Ast::Match(x) => Ast::Match(x.replace_vars(free_vars, vars)),
            Ast::Var(i) if free_vars.contains(&i) => {
                match vars.lookup(&i) {
                    Some(val) => val.into(),
                    None => {
                        //TODO we should raise an error here
                        unreachable!("this shouldnt happen i gusss / unknown variable ")
                    }
                }
            }
            Ast::PrimitiveCall(x) => Ast::PrimitiveCall(x.replace_vars(free_vars, vars)),
            Ast::List(x) => Ast::List(x.replace_vars(free_vars, vars)),
            Ast::Set(x) => Ast::Set(x.replace_vars(free_vars, vars)),
            Ast::Map(x) => Ast::Map(x.replace_vars(free_vars, vars)),
            _ => self,
        }
    }
}

impl From<Values> for Ast {
    fn from(value: Values) -> Self {
        match value {
            Values::Float(i) => Ast::Float(i),
            Values::Int(i) => Ast::Int(i),
            Values::Bool(i) => Ast::Bool(i),
            Values::Stack(i) => Ast::Stack(i),
            Values::List(i) => Ast::List(
                Stack {
                    elems: i.into_iter().map(|x| x.into()).collect(),
                }
                .into(),
            ),
            Values::Set(i) => Ast::List(
                Stack {
                    elems: i.into_iter().map(|x| x.into()).collect(),
                }
                .into(),
            ),
            Values::Map(i) => Ast::List(
                Stack {
                    elems: i
                        .into_iter()
                        .map(|(x, y)| {
                            let d = [x, y].into_iter().collect();
                            Values::List(d).into()
                        })
                        .collect(),
                }
                .into(),
            ),
            Values::Custom {
                name,
                tag,
                values: Some(val),
            } => Ast::TypeCall(
                name,
                tag,
                Stack {
                    elems: val.into_iter().map(|x| x.into()).collect(),
                },
            ),
            Values::Custom {
                name,
                tag,
                values: None,
            } => Ast::TypeCall(
                name,
                tag,
                Stack {
                    elems: vec![].into_iter().collect(),
                },
            ),
        }
    }
}
