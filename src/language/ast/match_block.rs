
use malachite::{Integer, Rational};
use super::stack::Stack;
use crate::language::eval::{Eval,EvalError,ChainMap,Values};
use crate::language::env::Env;

#[derive(Debug, Clone)]
pub enum Pattern {
    DontCare,
    Int(Integer),
    Float(Rational),
    Bool(bool),
    Variable(usize),
}


#[derive(Debug, Clone, Default)]
pub struct MatchElem {
    pattern: Vec<Pattern>,
    cond: Stack,
    body: Stack,
}
#[derive(Debug, Clone, Default)]
pub struct Match {
    elems: Vec<MatchElem>,
}




impl Eval<bool> for MatchElem {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<bool, EvalError> {
        vars.push();

        for pat in self.pattern.iter().rev() {
            let val = values.pop().unwrap();

            match (pat, val) {
                (Pattern::DontCare, _) => (),
                (Pattern::Int(x), Values::Int(y)) if *x == y => (),
                (Pattern::Float(x), Values::Float(y)) if *x == y => (),
                (Pattern::Bool(x), Values::Bool(y)) if *x == y => (),
                (Pattern::Variable(var), x) => vars.insert(*var, x.to_owned()),
                (_, _) => return Ok(false),
            }
        }

        if !self.cond.elems.is_empty() {
            let mut cond = vec![];

            match self.cond.eval(&mut cond, env, vars) {
                Ok(_) => {}
                Err(err) => {
                    return Err(EvalError::MatchCondFail(Box::new(err)));
                }
            }

            match cond.pop() {
                Some(Values::Bool(true)) => {}
                Some(Values::Bool(false)) => return Ok(false),
                Some(x) => {
                    return Err(EvalError::MatchCondExpectsBoolButGot(x.to_owned()));
                }
                None => {
                    return Err(EvalError::MatchCondUnderFlow);
                }
            };
        }

        match self.body.eval(values, env, vars) {
            Ok(_) => {}
            Err(err) => {
                return Err(EvalError::MatchArmFail(Box::new(err)));
            }
        }

        vars.pop();
        Ok(true)
    }
}




impl Eval<()> for Match {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<(), EvalError> {
        let max_len = self.elems.iter().map(|x| x.pattern.len()).max().unwrap();
        if max_len > values.len() {
            return Err(EvalError::MatchPatternUnderflow);
        }

        for arm in &self.elems {
            let mut temp_values = values.clone();
            match arm.eval(&mut temp_values, env, vars) {
                Ok(false) => {}
                Ok(true) => {
                    *values = temp_values;
                    return Ok(());
                }
                Err(err) => {
                    return Err(EvalError::MatchBodyFail(Box::new(err)));
                }
            }
        }

        return Err(EvalError::NoMatch);
    }
}


use crate::language::ast::Ast;
use std::str::FromStr;
use crate::language::parse::{Parse,Rule,ParseCtx};

impl Parse for Pattern{
    fn parse<'a>(pairs: pest::iterators::Pair<'a, Rule>, ctx: &mut ParseCtx) -> Self{
        match pairs.as_rule() {
            Rule::integer => Pattern::Int(Integer::from_str(pairs.as_str()).unwrap()),
            Rule::float => Pattern::Float(Rational::from_str(pairs.as_str()).unwrap()),
            Rule::bools => Pattern::Bool("true" == pairs.as_str()),
            Rule::varName => Pattern::Variable(ctx.insert_var(pairs.as_str())),
            Rule::dontCare => Pattern::DontCare,
            _ => unreachable!(),
        }

    }
}

impl Parse for MatchElem{
    fn parse<'a>(pairs: pest::iterators::Pair<'a, Rule>, ctx: &mut ParseCtx) -> Self{
        let mut inners = pairs.into_inner();
        let pattern: Vec<Pattern> = inners
            .next()
            .unwrap()
            .into_inner()
            .map(|x| Pattern::parse(x, ctx))
            .collect();
        let cond_or_action = inners.next().unwrap();
        if let Some(action) = inners.next() {
            let cond = Stack {
                elems: cond_or_action
                    .into_inner()
                    .map(|x| Ast::parse(x, ctx))
                    .collect(),
            };
            let body = Stack {
                elems: action.into_inner().map(|x| Ast::parse(x, ctx)).collect(),
            };
            MatchElem {
                pattern,
                cond,
                body,
            }
        } else {
            let condition = vec![] ;
            let body = Stack {
                elems: cond_or_action
                    .into_inner()
                    .map(|x| Ast::parse(x, ctx))
                    .collect(),
            };
            MatchElem {
                pattern,
                cond : Stack { elems: (condition.into_iter().collect()) },
                body,
            }
        }
        
    }
}


impl Parse for Match{
    fn parse<'a>(pairs: pest::iterators::Pair<'a, Rule>, ctx: &mut ParseCtx) -> Self{
        Match {
            elems: pairs
                .into_inner()
                .map(|x| MatchElem::parse(x, ctx))
                .collect(),
        }
    }
        
    }


use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Pattern {
    fn get_repr(&self, context: &ParseCtx) -> String {
        match self {
            Pattern::DontCare => "_".to_string(),
            Pattern::Int(i) => format!("{i}"),
            Pattern::Float(i) => format!("{i}"),
            Pattern::Bool(i) => format!("{i}"),
            Pattern::Variable(i) => context.lookup_var_name(*i),
        }
    }
}
impl Representation<(), ParseCtx> for MatchElem {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push_str("| ");
        self.pattern
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push_str("=> ");
        self.body
            .elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push_str(", ");
        result
    }
}
impl Representation<(), ParseCtx> for Match {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push_str("match ");
        self.elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result
    }
}
