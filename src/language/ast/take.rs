use super::stack::Stack;
use crate::language::eval::{Eval,EvalError,ChainMap,Values};
use crate::language::env::Env;

#[derive(Debug, Clone, Default)]
pub struct Take {
    vars: Vec<usize>,
    body: Stack,
}


impl Eval<()> for Take {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<(), EvalError> {
        if values.len() < self.vars.len() {
            return Err(EvalError::TakeUnderflow);
        }

        vars.push();
        for i in self.vars.iter().rev() {
            let val = values.pop().unwrap();
            vars.insert(*i, val);
        }
        match self.body.eval(values, env, vars) {
            Ok(_) => (),
            Err(err) => {
                vars.pop();
                return Err(EvalError::TakeBodyFail(Box::new(err)));
            }
        }
        vars.pop();
        return Ok(());
    }
}


use crate::language::ast::Ast;
use std::sync::Arc;

use crate::language::parse::{Parse,Rule,ParseCtx};

impl Parse for Take{
    fn parse<'a>(pairs: pest::iterators::Pair<'a, Rule>, ctx: &mut ParseCtx) -> Self{
        let mut inners = pairs.into_inner();
        let vars = inners.next().unwrap();
        let vars = match vars.as_rule() {
            Rule::takeVars => vars
                .into_inner()
                .map(|x| ctx.insert_var(x.as_str()))
                .collect::<Vec<_>>(),
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
        Take { vars, body }

    }
}

use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Take {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push('|');
        self.vars.iter().for_each(|x| {
            result.push_str(" ");
            result.push_str(&context.lookup_var_name(*x));
            result.push_str(" ");
        });

        result.push('|');
        result.push('{');
        self.body
            .elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push('}');
        result
    }
}
