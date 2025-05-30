use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use malachite::Rational;

use crate::language::ast::stack::Stack;

use super::{
    ast::Type,
    eval::{ChainMap, Eval, EvalError, Flow, Values},
    parse::ParseCtx,
};

#[derive(Clone)]
pub enum CallType {
    Stack(Stack),
    Fun(Rc<dyn for<'a> Fn(&'a mut Vec<Values>, &Env, &mut ChainMap) -> Result<(), EvalError>>),
}

impl CallType {
    pub fn eval(
        &self,
        vals: &mut Vec<Values>,
        env: &Env,
        map: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        match self {
            CallType::Stack(s) => s.eval(vals, env, map),
            CallType::Fun(f) => f(vals, env, map).map(|_| Ok(Flow::Ok))?,
        }
    }
}

#[derive(Clone)]
pub struct Env {
    pub data: HashMap<usize, Stack>,
    pub protocol_data: HashMap<usize, HashMap<Vec<Type>, (Vec<Type>, CallType)>>,
    pub protocol_arity: HashMap<usize, (usize, Option<usize>)>,
    pub typ_data: HashMap<(usize, Option<usize>), Vec<Type>>,
    pub type_variants: HashMap<usize, HashSet<usize>>,
}

impl Default for Env {
    fn default() -> Self {
        let mut ret = Self {
            data: Default::default(),
            protocol_data: Default::default(),
            typ_data: Default::default(),
            type_variants: Default::default(),
            protocol_arity: Default::default(),
        };

        let mut ctx = ParseCtx::default();
        {
            let fun = ctx.insert_fun("add");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a + b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Float(a + b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Integer, Type::Float],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Float(b)) => {
                                values.push(Values::Float(Rational::from(a) + b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Integer],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Int(b)) => {
                                values.push(Values::Float(a + Rational::from(b)))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("sub");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a - b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Float(a - b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Integer, Type::Float],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Float(b)) => {
                                values.push(Values::Float(Rational::from(a) - b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Integer],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Int(b)) => {
                                values.push(Values::Float(a - Rational::from(b)))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("mult");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a * b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Float(a * b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Integer, Type::Float],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Float(b)) => {
                                values.push(Values::Float(Rational::from(a) * b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Integer],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Int(b)) => {
                                values.push(Values::Float(a * Rational::from(b)))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("div");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a / b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Float(a / b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Integer, Type::Float],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Float(b)) => {
                                values.push(Values::Float(Rational::from(a) / b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Integer],
                (
                    vec![Type::Float],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Int(b)) => {
                                values.push(Values::Float(a / Rational::from(b)))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("eq");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a == b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Bool(a == b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::List, Type::List],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::List(a), Values::List(b)) => values.push(Values::Bool(a == b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set, Type::Set],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Set(a), Values::Set(b)) => values.push(Values::Bool(a == b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Map, Type::Map],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Set(a), Values::Set(b)) => values.push(Values::Bool(a == b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("neq");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a != b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Bool(a != b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::List, Type::List],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::List(a), Values::List(b)) => values.push(Values::Bool(a != b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set, Type::Set],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Set(a), Values::Set(b)) => values.push(Values::Bool(a != b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Map, Type::Map],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Set(a), Values::Set(b)) => values.push(Values::Bool(a == b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("geq");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a >= b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Bool(a >= b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }

        {
            let fun = ctx.insert_fun("leq");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a <= b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Bool(a <= b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("ge");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a > b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Bool(a > b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }

        {
            let fun = ctx.insert_fun("le");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a < b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Float, Type::Float],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Float(a), Values::Float(b)) => {
                                values.push(Values::Bool(a < b))
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }

        {
            let fun = ctx.insert_fun("and");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a & b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Bool, Type::Bool],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Bool(a), Values::Bool(b)) => values.push(Values::Bool(a & b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("xor");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a ^ b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Bool, Type::Bool],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Bool(a), Values::Bool(b)) => values.push(Values::Bool(a ^ b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("or");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a | b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Bool, Type::Bool],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        match (a, b) {
                            (Values::Bool(a), Values::Bool(b)) => values.push(Values::Bool(a | b)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("not");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Integer, Type::Integer],
                (
                    vec![Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let a = values.pop().unwrap();
                        match a {
                            Values::Int(a) => values.push(Values::Int(!a)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Bool, Type::Bool],
                (
                    vec![Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let a = values.pop().unwrap();
                        match a {
                            Values::Bool(a) => values.push(Values::Bool(!a)),
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (1, Some(1)));
        }
        {
            let fun = ctx.insert_fun("stack_size");
            let mut map = HashMap::new();
            map.insert(
                vec![],
                (
                    vec![Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let l = values.len();
                        values.push(Values::Int(l.into()));
                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (0, Some(1)));
        }
        {
            let fun = ctx.insert_fun("get");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::Integer],
                (
                    vec![Type::GenericTyp(usize::MAX)],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let idx = values.pop().unwrap();
                        let list = values.pop().unwrap();
                        match (list, idx) {
                            (Values::List(data), Values::Int(i)) => {
                                if i >= data.len() {
                                    return Err(EvalError::IndexOutOfBounds);
                                }
                                let idx =
                                    usize::try_from(&i).map_err(|_| EvalError::PrimitiveEvalErr)?;
                                let ret = data.get(idx).unwrap().clone();
                                let list = Values::List(data);
                                values.push(list);
                                values.push(ret);
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Map, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::GenericTyp(usize::MAX)],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let idx = values.pop().unwrap();
                        let list = values.pop().unwrap();
                        match (list, idx) {
                            (Values::Map(data), i) => {
                                let ret = data.get(&i).cloned();
                                let list = Values::Map(data);
                                values.push(list);
                                if ret.is_some() {
                                    values.push(ret.unwrap());
                                }
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(2)));
        }

        {
            let fun = ctx.insert_fun("set");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::Integer, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::GenericTyp(usize::MAX)],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let new_elem = values.pop().unwrap();
                        let idx = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, idx) {
                            (Values::List(mut data), Values::Int(i)) => {
                                if i >= data.len() {
                                    return Err(EvalError::IndexOutOfBounds);
                                }
                                let idx =
                                    usize::try_from(&i).map_err(|_| EvalError::PrimitiveEvalErr)?;
                                *data.get_mut(idx).unwrap() = new_elem;
                                let list = Values::List(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![
                    Type::Map,
                    Type::GenericTyp(usize::MAX),
                    Type::GenericTyp(usize::MAX),
                ],
                (
                    vec![Type::GenericTyp(usize::MAX)],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let new_elem = values.pop().unwrap();
                        let idx = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, idx) {
                            (Values::Map(mut data), i) => {
                                data.insert(i, new_elem);
                                let list = Values::Map(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (3, Some(1)));
        }
        {
            let fun = ctx.insert_fun("concat");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::List],
                (
                    vec![Type::List],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let to_append = values.pop().unwrap();
                        let orig = values.pop().unwrap();

                        match (orig, to_append) {
                            (Values::List(data1), Values::List(mut data2)) => {
                                //let mut data1 = data1.clone();
                                data1.to_owned().append(&mut data2);
                                let list = Values::List(data1);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set, Type::Set],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let to_append = values.pop().unwrap();
                        let orig = values.pop().unwrap();

                        match (orig, to_append) {
                            (Values::Set(data1), Values::Set(mut data2)) => {
                                //let mut data1 = data1.clone();
                                data1.to_owned().append(&mut data2);
                                let list = Values::Set(data1);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Map, Type::Map],
                (
                    vec![Type::Map],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let to_append = values.pop().unwrap();
                        let orig = values.pop().unwrap();

                        match (orig, to_append) {
                            (Values::Map(data1), Values::Map(mut data2)) => {
                                //let mut data1 = data1.clone();
                                data1.to_owned().append(&mut data2);
                                let list = Values::Map(data1);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("push_first");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::List],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let elem = values.pop().unwrap();
                        let list = values.pop().unwrap();
                        match (list, elem) {
                            (Values::List(mut data), value) => {
                                data.push_back(value);
                                let list = Values::List(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let elem = values.pop().unwrap();
                        let list = values.pop().unwrap();
                        match (list, elem) {
                            (Values::Set(mut data), value) => {
                                data.insert(value);
                                let list = Values::Set(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (0, Some(1)));
        }
        {
            let fun = ctx.insert_fun("push");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::List],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let elem = values.pop().unwrap();
                        let list = values.pop().unwrap();
                        match (list, elem) {
                            (Values::List(mut data), value) => {
                                data.push_front(value);
                                let list = Values::List(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let elem = values.pop().unwrap();
                        let list = values.pop().unwrap();
                        match (list, elem) {
                            (Values::Set(mut data), value) => {
                                data.insert(value);
                                let list = Values::Set(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("pop_first");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::GenericTyp(usize::MAX)],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let list = values.pop().unwrap();
                        match list {
                            Values::List(mut data) => {
                                if data.is_empty() {
                                    return Err(EvalError::IndexOutOfBounds);
                                }
                                let res = data.pop_front().unwrap();
                                let list = Values::List(data);
                                values.push(list);
                                values.push(res);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::GenericTyp(usize::MAX)],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let list = values.pop().unwrap();
                        match list {
                            Values::Set(mut data) => {
                                if data.is_empty() {
                                    return Err(EvalError::IndexOutOfBounds);
                                }
                                let res = data.pop_first().unwrap();
                                let list = Values::Set(data);
                                values.push(list);
                                values.push(res);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (1, Some(2)));
        }
        {
            let fun = ctx.insert_fun("pop");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::GenericTyp(usize::MAX)],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let list = values.pop().unwrap();
                        match list {
                            Values::List(mut data) => {
                                if data.is_empty() {
                                    return Err(EvalError::IndexOutOfBounds);
                                }
                                let res = data.pop_back().unwrap();
                                let list = Values::List(data);
                                values.push(list);
                                values.push(res);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::GenericTyp(usize::MAX)],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let list = values.pop().unwrap();
                        match list {
                            Values::Set(mut data) => {
                                if data.is_empty() {
                                    return Err(EvalError::IndexOutOfBounds);
                                }
                                let res = data.pop_last().unwrap();
                                let list = Values::Set(data);
                                values.push(list);
                                values.push(res);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );
            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (1, Some(2)));
        }
        {
            let fun = ctx.insert_fun("delete");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::Integer],
                (
                    vec![Type::List],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let idx = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, idx) {
                            (Values::List(mut data), Values::Int(i)) => {
                                if i >= data.len() {
                                    return Err(EvalError::IndexOutOfBounds);
                                }
                                let idx =
                                    usize::try_from(&i).map_err(|_| EvalError::PrimitiveEvalErr)?;
                                let _ = data.remove(idx);
                                let list = Values::List(data);
                                values.push(list);
                                //values.push(ret);
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let idx = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, idx) {
                            (Values::Set(mut data), i) => {
                                let _ = data.remove(&i);
                                let list = Values::Set(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Map, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::Map],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let idx = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, idx) {
                            (Values::Map(mut data), i) => {
                                let _ = data.remove(&i);
                                let list = Values::Map(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("insert");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::Integer, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::List],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let new_elem = values.pop().unwrap();

                        let idx = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, idx) {
                            (Values::List(mut data), Values::Int(i)) => {
                                if i >= data.len() {
                                    return Err(EvalError::IndexOutOfBounds);
                                }
                                let idx =
                                    usize::try_from(&i).map_err(|_| EvalError::PrimitiveEvalErr)?;
                                data.insert(idx, new_elem);
                                let list = Values::List(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![
                    Type::Map,
                    Type::GenericTyp(usize::MAX),
                    Type::GenericTyp(usize::MAX),
                ],
                (
                    vec![Type::Map],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let new_elem = values.pop().unwrap();

                        let idx = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, idx) {
                            (Values::Map(mut data), i) => {
                                data.insert(i, new_elem);
                                let list = Values::Map(data);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (3, Some(1)));
        }
        {
            let fun = ctx.insert_fun("len");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List],
                (
                    vec![Type::List, Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let list = values.pop().unwrap();
                        match list {
                            Values::List(data) => {
                                let len = data.len();
                                let list = Values::List(data);
                                values.push(list);
                                values.push(Values::Int(len.into()));
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set],
                (
                    vec![Type::Set, Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let list = values.pop().unwrap();
                        match list {
                            Values::Set(data) => {
                                let len = data.len();
                                let list = Values::Set(data);
                                values.push(list);
                                values.push(Values::Int(len.into()));
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Map],
                (
                    vec![Type::Map, Type::Integer],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let list = values.pop().unwrap();
                        match list {
                            Values::Map(data) => {
                                let len = data.len();
                                let list = Values::Map(data);
                                values.push(list);
                                values.push(Values::Int(len.into()));
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (1, Some(2)));
        }
        {
            let fun = ctx.insert_fun("contains");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::List, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::List, Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let elem = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, elem) {
                            (Values::List(data), elem) => {
                                let ret = data.contains(&elem);
                                let list = Values::List(data);
                                values.push(list);
                                values.push(Values::Bool(ret));
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Set, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::Set, Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let elem = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, elem) {
                            (Values::Set(data), elem) => {
                                let ret = data.contains(&elem);
                                let list = Values::Set(data);
                                values.push(list);
                                values.push(Values::Bool(ret));
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );
            map.insert(
                vec![Type::Map, Type::GenericTyp(usize::MAX)],
                (
                    vec![Type::Map, Type::Bool],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let elem = values.pop().unwrap();
                        let list = values.pop().unwrap();

                        match (list, elem) {
                            (Values::Map(data), elem) => {
                                let ret = data.contains_key(&elem);
                                let list = Values::Map(data);
                                values.push(list);
                                values.push(Values::Bool(ret));
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(2)));
        }

        {
            let fun = ctx.insert_fun("intersect");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Set, Type::Set],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let to_append = values.pop().unwrap();
                        let orig = values.pop().unwrap();
                        match (orig, to_append) {
                            (Values::Set(data1), Values::Set(data2)) => {
                                let ret = data1.intersection(&data2).cloned().collect();
                                let list = Values::Set(ret);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }
        {
            let fun = ctx.insert_fun("union");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Set, Type::Set],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let to_append = values.pop().unwrap();
                        let orig = values.pop().unwrap();
                        match (orig, to_append) {
                            (Values::Set(data1), Values::Set(data2)) => {
                                let ret = data1.union(&data2).cloned().collect();
                                let list = Values::Set(ret);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }

        {
            let fun = ctx.insert_fun("difference");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Set, Type::Set],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let to_append = values.pop().unwrap();
                        let orig = values.pop().unwrap();
                        match (orig, to_append) {
                            (Values::Set(data1), Values::Set(data2)) => {
                                let ret = data1.difference(&data2).cloned().collect();
                                let list = Values::Set(ret);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }

        {
            let fun = ctx.insert_fun("symmetric_difference");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Set, Type::Set],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let to_append = values.pop().unwrap();
                        let orig = values.pop().unwrap();
                        match (orig, to_append) {
                            (Values::Set(data1), Values::Set(data2)) => {
                                let ret = data1.symmetric_difference(&data2).cloned().collect();
                                let list = Values::Set(ret);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }

        {
            let fun = ctx.insert_fun("subset");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Set, Type::Set],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let to_append = values.pop().unwrap();
                        let orig = values.pop().unwrap();
                        match (orig, to_append) {
                            (Values::Set(data1), Values::Set(data2)) => {
                                let ret = data1.is_subset(&data2);
                                let list = Values::Bool(ret);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }

        {
            let fun = ctx.insert_fun("superset");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Set, Type::Set],
                (
                    vec![Type::Set],
                    CallType::Fun(Rc::new(|values, _env, _chain_map| {
                        let to_append = values.pop().unwrap();
                        let orig = values.pop().unwrap();
                        match (orig, to_append) {
                            (Values::Set(data1), Values::Set(data2)) => {
                                let ret = data1.is_superset(&data2);
                                let list = Values::Bool(ret);
                                values.push(list);
                            }
                            _ => unreachable!(),
                        }

                        Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (2, Some(1)));
        }

        {
            let fun = ctx.insert_fun("apply");
            let mut map = HashMap::new();
            map.insert(
                vec![Type::Stack],
                (
                    vec![],
                    CallType::Fun(Rc::new(|values, env, chain_map| {
                        if let Values::Stack(stack) = values.pop().unwrap() {
                            match stack.to_owned().eval(values, env, chain_map) {
                                Ok(Flow::Ret | Flow::Break) => Ok(()),
                                _ret @ Ok(_) => Ok(()),
                                Err(_) => Err(EvalError::PrimitiveEvalErr),
                            }
                        } else {
                            unreachable!()
                        }
                        //Ok(())
                    })),
                ),
            );

            ret.protocol_data.insert(fun, map);
            ret.protocol_arity.insert(fun, (1, None));
        }
        ret
    }
}
