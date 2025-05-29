use std::{collections::BTreeSet, sync::Arc};

use crate::language::{
    env::Env,
    eval::{ChainMap, Eval, EvalError, Flow, Values},
    parse::{Parse, ParseCtx, Rule},
};

use super::{stack::Stack, Ast};

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Set {
    elements: Stack,
}

impl From<Stack> for Set {
    fn from(value: Stack) -> Self {
        Self { elements: value }
    }
}

impl Parse for Set {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        Self {
            elements: Stack {
                elems: pairs
                    .into_inner()
                    .map(|x| Ast::parse(x, ctx))
                    .collect::<Arc<_>>(),
            },
        }
    }
}

use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Set {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push_str("Set(");
        self.elements
            .elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push(')');
        result
    }
}

impl Eval<Flow> for Set {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        let mut result = vec![];
        let _ = self.elements.eval(&mut result, env, vars)?;
        let ret = Values::Set(result.into_iter().collect::<BTreeSet<_>>());
        values.push(ret);
        Ok(Flow::Ok)
    }

    fn get_free_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        self.elements.get_free_vars(vars)
    }

    fn get_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        self.elements.get_vars(vars)
    }

    fn replace_vars(self, free_vars: &std::collections::HashSet<usize>, vars: &ChainMap) -> Self {
        let elements = self.elements;
        let elems = elements.replace_vars(free_vars, vars);
        Self { elements: elems }
    }
}
