use super::stack::Stack;
use crate::language::env::Env;
use crate::language::eval::{ChainMap, Eval, EvalError, Values,Flow};
#[derive(Debug, Clone, Default,PartialEq,PartialOrd,Ord,Eq)]
pub struct While {
    cond: Stack,
    body: Stack,
}

impl Eval<Flow> for While {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        loop {
            let res = self.cond.eval(values, env, vars);
            match res {
                
                Ok(_) => match values.pop() {
                    Some(Values::Bool(true)) => {}
                    Some(Values::Bool(false)) => break,
                    Some(x) => {
                        return Err(EvalError::WhileCondExpectsBoolButGot(x.to_owned()));
                    }
                    None => {
                        return Err(EvalError::WhileCondUnderFlow);
                    }
                },
                Err(err) => return Err(EvalError::WhileCondFail(Box::new(err))),
            }
            let res = self.body.eval(values, env, vars);
            match res {
                Ok(Flow::Break) => { break;}
                Ok(Flow::Ret)   => {return Ok(Flow::Ret)}
                Ok(_) => (),
                Err(err) => return Err(EvalError::WhileBodyFail(Box::new(err))),
            }
        }
        Ok(Flow::Ok)
    }

    fn replace_vars(self,free_vars:& std::collections::HashSet<usize>,vars:&ChainMap)->Self {
        let While { mut cond, mut body } = self;
        cond = cond.replace_vars(free_vars, vars);
        body = body.replace_vars(free_vars, vars);
        While {  cond,  body }
    }
    fn get_free_vars(&self,vars:&mut std::collections::HashSet<usize>) {
        self.cond.get_free_vars(vars);
        self.body.get_free_vars(vars)
    }

    fn get_vars(&self,vars:&mut std::collections::HashSet<usize>) {
        self.cond.get_vars(vars);
        self.body.get_vars(vars)
    }
}

use crate::language::ast::Ast;
use std::sync::Arc;

use crate::language::parse::{Parse, ParseCtx, Rule};

impl Parse for While {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        let mut inners = pairs.into_inner();
        let vars = inners.next().unwrap();
        let cond = match vars.as_rule() {
            Rule::whileCond => Stack {
                elems: vars
                    .into_inner()
                    .map(|x| Ast::parse(x, ctx))
                    .collect::<Arc<_>>(),
            },
            _ => unreachable!(),
        };

        let blocks = inners.next().unwrap();
        let body = match blocks.as_rule() {
            Rule::block => Stack {
                elems: blocks
                    .into_inner()
                    .map(|x| Ast::parse(x, ctx))
                    .collect::<Arc<_>>(),
            },
            _ => unreachable!(),
        };
        While { cond, body }
    }
}

use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for While {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push_str("while ");
        self.cond
            .elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push('{');
        self.body
            .elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push('}');
        result
    }
}
