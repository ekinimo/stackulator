use super::ast::primitives::Primitives;
use super::ast::Type;
use super::env::Env;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

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

    PrimitiveUnderflow(Primitives),
    PrimitiveTypeErr(Primitives, String),
    PrimitiveEvalErr,
    MapExprMustHaveListOfLen2,

    TypeDoesntExist(usize),
    TypeConstructorLenMismatch(usize, usize, usize),

    IndexOutOfBounds,
    Underflow,
}

use malachite::{Integer, Rational};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Values {
    Bool(bool),
    Int(Integer),
    Float(Rational),

    Stack(super::ast::stack::Stack),
    List(VecDeque<Values>),
    Set(BTreeSet<Values>),
    Map(BTreeMap<Values, Values>),

    Custom {
        name: usize,
        tag: Option<usize>,
        values: Option<VecDeque<Values>>,
    },
}

impl Values {
    pub fn get_type(&self) -> &str {
        match self {
            Values::Float(_) => "float",
            Values::Int(_) => "int",
            Values::Bool(_) => "bool",
            Values::Stack(_) => "stack",
            Values::List(_) => "list",
            Values::Set(_) => "set",
            Values::Map(_) => "map",
            Values::Custom {
                name: _,
                tag: _,
                values: _,
            } => "",
        }
    }

    pub fn get_real_type(&self) -> Type {
        match self {
            Values::Bool(_) => Type::Bool,
            Values::Int(_) => Type::Integer,
            Values::Float(_) => Type::Float,
            Values::Stack(_) => Type::Stack,
            Values::List(_) => Type::List,
            Values::Set(_) => Type::Set,
            Values::Map(_) => Type::Map,
            Values::Custom { name, .. } => Type::CustomType(*name),
        }
    }
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
    pub fn lookup(&self, var: &usize) -> Option<Values> {
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
            Values::List(l) => {
                let mut ret = String::new();
                ret.push_str("List(");
                let len = l.len();
                l.iter().enumerate().for_each(|(idx, x)| {
                    ret.push_str(&x.get_repr(context));
                    if idx != len - 1 {
                        ret.push_str(", ");
                    }
                });
                ret.push(')');
                ret
            }
            Values::Set(l) => {
                let mut ret = String::new();
                let len = l.len();
                ret.push_str("Set(");
                l.iter().enumerate().for_each(|(idx, x)| {
                    ret.push_str(&x.get_repr(context));
                    if idx != len - 1 {
                        ret.push_str(", ");
                    }
                });
                ret.push(')');
                ret
            }
            Values::Map(map) => {
                let mut ret = String::new();
                ret.push_str("Map(");
                let len = map.len();
                map.iter().enumerate().for_each(|(idx, (x, y))| {
                    ret.push_str("[ ");
                    ret.push_str(&x.get_repr(context));
                    ret.push_str(", ");
                    ret.push_str(&y.get_repr(context));
                    ret.push_str("] ");
                    if idx != len - 1 {
                        ret.push_str(", ");
                    }
                });
                ret.push(')');
                ret
            }
            Values::Custom { name, tag, values } => {
                let mut ret = String::new();
                ret.push_str(context.lookup_type_name(*name).as_str());
                tag.map(|tag| {
                    ret.push_str("::");
                    ret.push_str(context.lookup_tag_name(tag).as_str())
                });
                ret.push('(');
                values.as_ref().map(|x| {
                    let len = x.len();
                    x.iter().enumerate().for_each(|(idx, value)| {
                        ret.push_str(&value.get_repr(context));
                        if idx != len - 1 {
                            ret.push(',');
                        }
                    })
                });

                ret.push(')');
                ret
            }
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
            EvalError::PrimitiveUnderflow(p) => {
                format!("PrimitiveUnderflow {}", p.get_repr(context))
            }
            EvalError::PrimitiveTypeErr(p, s) => {
                format!("PrimitiveTypeErr {} {}", p.get_repr(context), s)
            }
            EvalError::PrimitiveEvalErr => "PrimitiveEvalErr ".to_string(),
            EvalError::IfCondUnderFlow => "IfCondUnderFlow".to_string(),
            EvalError::NoMatch => "NoMatch".to_string(),
            EvalError::MatchPatternUnderflow => "MatchPatternUnderflow".to_string(),
            EvalError::WhileCondUnderFlow => "WhileCondUnderFlow".to_string(),
            EvalError::TakeUnderflow => "TakeUnderflow".to_string(),
            EvalError::MapExprMustHaveListOfLen2 => "MapExprMustHaveListOfLen2".to_string(),
            EvalError::TypeDoesntExist(x) => {
                format!("TypeDoesntExist {}", context.lookup_type_name(*x))
            }
            EvalError::TypeConstructorLenMismatch(name, expects, got) => {
                format!(
                    "TypeConstructorLenMismatch {} expects {expects} many arguments but got {got}.",
                    context.lookup_type_name(*name)
                )
            }
            EvalError::IndexOutOfBounds => todo!(),
            EvalError::Underflow => "Underflow".to_string(),
        }
    }
}

pub enum Flow {
    Break,
    Ok,
    Ret,
    Cont,
}

pub trait Eval<T> {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<T, EvalError>;

    fn get_free_vars(&self, vars: &mut HashSet<usize>);
    fn get_vars(&self, vars: &mut std::collections::HashSet<usize>);
    fn replace_vars(self, free_vars: &std::collections::HashSet<usize>, vars: &ChainMap) -> Self;
}
