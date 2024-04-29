use crate::language::ast::stack::Stack;
use crate::language::eval::{Eval,EvalError,ChainMap,Values};
use crate::language::env::Env;

#[derive(Debug, Clone, Default)]
pub struct IfTrue {
    elems: Stack,
}


impl Eval<()> for IfTrue {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<(), EvalError> {
        match values.pop() {
            Some(Values::Bool(true)) => match self.elems.eval(values, env, vars) {
                x @ Ok(_) => x,
                Err(err) => return Err(EvalError::IfBodyFail(Box::new(err))),
            },
            Some(Values::Bool(false)) => Ok(()),
            Some(x) => {
                return Err(EvalError::IfCondExpectsBoolButGot(x.to_owned()));
            }
            None => {
                return Err(EvalError::IfCondUnderFlow);
            }
        }
    }
}

use crate::language::ast::Ast;
use std::sync::Arc;

use crate::language::parse::{Parse,Rule,ParseCtx};
impl Parse for IfTrue{
    fn parse<'a>(pairs: pest::iterators::Pair<'a, Rule>, ctx: &mut ParseCtx) -> Self{
        let mut inners = pairs.into_inner();
        let vars = inners.next().unwrap();
        let elems = match vars.as_rule() {
            Rule::block => Stack {
                elems: vars
                    .into_inner()
                    .map(|x| Ast::parse(x, ctx))
                    .collect::<Arc<_>>(),
            },
            _ => unreachable!(),
        };
        IfTrue { elems }
    }
}


use crate::language::repr::Representation;

impl Representation<(), ParseCtx> for IfTrue {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push_str("?{");
        self.elems
            .elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push('}');
        result
    }
}
