use super::Ast;
use crate::language::env::Env;
use crate::language::eval::{ChainMap, Eval, EvalError, Flow, Values};
use crate::language::parse::{Parse, ParseCtx, Rule};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Stack {
    pub elems: Arc<[Ast]>,
}

impl Default for Stack {
    fn default() -> Self {
        let elems = vec![].into();
        Self { elems }
    }
}

impl Eval<Flow> for Stack {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        for elem in self.elems.iter() {
            let ret = elem.eval(values, env, vars);
            match ret {
                Ok(Flow::Ok | Flow::Cont) => (),
                ret @ Ok(Flow::Break | Flow::Ret) => return ret,
                err @ Err(_) => return err,
            }
        }
        Ok(Flow::Ok)
    }

    fn get_free_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        self.elems.iter().for_each(|x| x.get_free_vars(vars));
    }

    fn get_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        self.elems.iter().for_each(|x| x.get_vars(vars));
    }

    fn replace_vars(self, free_vars: &std::collections::HashSet<usize>, vars: &ChainMap) -> Self {
        let Stack { mut elems } = self;
        elems = elems
            .iter()
            .map(|x| x.clone().replace_vars(free_vars, vars))
            .collect();
        Stack { elems }
    }
}

impl Parse for Stack {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        Self {
            elems: pairs
                .into_inner()
                .map(|x| Ast::parse(x, ctx))
                .collect::<Arc<_>>(),
        }
    }
}

use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Stack {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push('[');
        self.elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push(']');
        result
    }
}
