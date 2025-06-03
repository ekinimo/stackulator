use super::stack::Stack;
use crate::language::env::Env;
use crate::language::eval::{ChainMap, Eval, EvalError, Flow, Values};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct Take {
    vars: Vec<usize>,
    body: Stack,
}

impl Eval<Flow> for Take {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        if values.len() < self.vars.len() {
            return Err(EvalError::TakeUnderflow);
        }

        vars.push();
        for i in self.vars.iter().rev() {
            let val = values.pop().unwrap();
            vars.insert(*i, val);
        }
        let ret = match self.body.eval(values, env, vars) {
            res @ Ok(_) => res,
            Err(err) => {
                vars.pop();
                return Err(EvalError::TakeBodyFail(Box::new(err)));
            }
        };
        vars.pop();
        ret
    }

    fn get_free_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        self.body.get_vars(vars);
        let my_vars: HashSet<usize> = self.vars.iter().cloned().collect();
        *vars = vars.difference(&my_vars).cloned().collect();
    }

    fn get_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        self.body.get_vars(vars)
    }

    fn replace_vars(
        self,
        free_vars: &std::collections::HashSet<usize>,
        map_vars: &ChainMap,
    ) -> Self {
        let Take { vars, mut body } = self;
        body = body.replace_vars(free_vars, map_vars);
        Take { vars, body }
    }
}

use crate::language::ast::Ast;
use std::collections::HashSet;
use std::sync::Arc;

use crate::language::parse::{Parse, ParseCtx, Rule};

impl Parse for Take {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        let mut inners = pairs.into_inner();
        let vars = inners.next().unwrap();
        ctx.push_scope();
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
        ctx.pop_scope();
        Take { vars, body }
    }
}

use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Take {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push('|');
        self.vars.iter().for_each(|x| {
            result.push(' ');
            result.push_str(&context.lookup_var_name(*x));
            result.push(' ');
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
