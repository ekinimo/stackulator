use super::env::Env;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum EvalError {
    UndefinedVariable(usize),
    UndefinedCall(usize),
    FuncCallFail(Box<(usize, EvalError)>),
    WhileCondUnderFlow,
    WhileCondExpectsBoolButGot(Values),
    WhileCondFail(Box<EvalError>),
    WhileBodyFail(Box<EvalError>),

    IfCondUnderFlow,
    IfCondExpectsBoolButGot(Values),
    IfBodyFail(Box<EvalError>),

    TakeUnderflow,
    TakeBodyFail(Box<EvalError>),

    NoMatch,
    MatchPatternUnderflow,
    MatchBodyFail(Box<EvalError>),
    MatchCondFail(Box<EvalError>),
    MatchArmFail(Box<EvalError>),
    MatchCondUnderFlow,
    MatchCondExpectsBoolButGot(Values),

    PrimitiveUnderflow,
    PrimitiveTypeErr,
    PrimitiveEvalErr,
}

use malachite::{Integer, Rational};

#[derive(Debug, Clone)]
pub enum Values {
    Float(Rational),
    Int(Integer),
    Bool(bool),
    Stack(super::ast::stack::Stack),
}

#[derive(Clone, Debug)]
pub struct ChainMap {
    data: Vec<HashMap<usize, Values>>,
}

impl Default for ChainMap {
    fn default() -> Self {
        ChainMap {
            data: vec![HashMap::default()],
        }
    }
}

impl ChainMap {
    pub fn push(&mut self) {
        self.data.push(HashMap::new());
    }
    pub fn pop(&mut self) {
        self.data.pop();
    }
    pub fn insert(&mut self, var: usize, value: Values) {
        let mut last = self
            .data
            .pop()
            .expect("this shouldnt happen, Chainmap supposed to be initialized");
        last.insert(var, value);
        self.data.push(last);
    }
    pub fn lookup(&mut self, var: &usize) -> Option<Values> {
        for map in self.data.iter().rev() {
            if map.contains_key(var) {
                return map.get(var).cloned();
            }
        }
        None
    }
}

use crate::language::parse::ParseCtx;

use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Values {
    fn get_repr(&self, context: &ParseCtx) -> String {
        match self {
            Values::Float(i) => format!("{i}"),
            Values::Int(i) => format!("{i}"),
            Values::Bool(i) => format!("{i}"),
            Values::Stack(s) => s.get_repr(context),
        }
    }
}
impl Representation<(), ParseCtx> for EvalError {
    fn get_repr(&self, context: &ParseCtx) -> String {
        match self {
            EvalError::UndefinedVariable(x) => {
                format!("UndefinedVariable {}", context.lookup_var_name(*x))
            }
            EvalError::UndefinedCall(x) => {
                format!("UndefinedCall {}", context.lookup_call_name(*x))
            }
            EvalError::FuncCallFail(x) => {
                let (x, y) = &**x;
                format!(
                    "FuncCallFail {} {} ",
                    context.lookup_call_name(*x),
                    y.get_repr(context)
                )
            }
            EvalError::WhileCondExpectsBoolButGot(x) => {
                format!("WhileCondExpectsBoolButGot {}", x.get_repr(context))
            }
            EvalError::WhileCondFail(x) => {
                format!("WhileCondFail {}", x.get_repr(context))
            }
            EvalError::WhileBodyFail(x) => {
                format!("WhileBodyFail {}", x.get_repr(context))
            }
            EvalError::IfCondExpectsBoolButGot(x) => {
                format!("IfCondExpectsBoolButGot {}", x.get_repr(context))
            }
            EvalError::IfBodyFail(x) => {
                format!("IfBodyFail {}", x.get_repr(context))
            }
            EvalError::TakeBodyFail(x) => {
                format!("TakeBodyFail {}", x.get_repr(context))
            }
            EvalError::MatchBodyFail(x) => {
                format!("MatchBodyFail {}", x.get_repr(context))
            }
            EvalError::MatchCondFail(x) => {
                format!("MatchCondFail {}", x.get_repr(context))
            }
            EvalError::MatchArmFail(x) => {
                format!("MatchArmFail {}", x.get_repr(context))
            }
            EvalError::MatchCondExpectsBoolButGot(x) => {
                format!("MatchCondExpectsBoolButGot {}", x.get_repr(context))
            }
            EvalError::MatchCondUnderFlow => "MatchCondUnderFlow".to_string(),
            EvalError::PrimitiveUnderflow => "PrimitiveUnderflow".to_string(),
            EvalError::PrimitiveTypeErr => "PrimitiveTypeErr".to_string(),
            EvalError::PrimitiveEvalErr => "PrimitiveEvalErr".to_string(),
            EvalError::IfCondUnderFlow => "IfCondUnderFlow".to_string(),
            EvalError::NoMatch => "NoMatch".to_string(),
            EvalError::MatchPatternUnderflow => "MatchPatternUnderflow".to_string(),
            EvalError::WhileCondUnderFlow => "WhileCondUnderFlow".to_string(),
            EvalError::TakeUnderflow => "TakeUnderflow".to_string(),
        }
    }
}

pub trait Eval<T> {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<T, EvalError>;
}
