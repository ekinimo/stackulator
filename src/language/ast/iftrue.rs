use crate::language::ast::stack::Stack;
use crate::language::env::Env;
use crate::language::eval::{ChainMap, Eval, EvalError, Values,Flow};

#[derive(Debug, Clone, Default,PartialEq,PartialOrd,Ord,Eq)]
pub struct IfTrue {
    elems: Stack,
}

impl From<Stack> for IfTrue{
    fn from(value: Stack) -> Self {
        Self{
            elems:value
        }
    }
}


impl Eval<Flow> for IfTrue {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        match values.pop() {
            Some(Values::Bool(true)) => match self.elems.eval(values, env, vars) {
                x @ Ok(_) => x,
                Err(err) => Err(EvalError::IfBodyFail(Box::new(err))),
            },
            Some(Values::Bool(false)) => Ok(Flow::Ok),
            Some(x) => Err(EvalError::IfCondExpectsBoolButGot(x.to_owned())),
            None => Err(EvalError::IfCondUnderFlow),
        }
    }

    fn get_free_vars(&self,vars:&mut std::collections::HashSet<usize>) {
        self.elems.get_free_vars(vars)
    }

    fn get_vars(&self,vars:&mut std::collections::HashSet<usize>) {
        self.elems.get_vars(vars)
    }

    fn replace_vars( self,free_vars:& std::collections::HashSet<usize>,vars:&ChainMap)->Self {
        let IfTrue{mut elems} = self;
        elems = elems.replace_vars(free_vars, vars);
        IfTrue{elems}
    }

    
}

use crate::language::ast::Ast;
use std::sync::Arc;

use crate::language::parse::{Parse, ParseCtx, Rule};
impl Parse for IfTrue {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
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
