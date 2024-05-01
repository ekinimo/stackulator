use std::{sync::Arc, collections::{ BTreeMap}};

use crate::language::{parse::{Parse, Rule, ParseCtx}, eval::{Eval, Flow, Values, ChainMap, EvalError}, env::Env};

use super::{stack::Stack, Ast};


#[derive(Clone,Debug,PartialEq,PartialOrd,Ord,Eq)]
pub struct Map{
    elements: Stack
}

impl From<Stack> for Map{
    fn from(value: Stack) -> Self {
        Self{
            elements : value
        }
    }
}

impl Parse for Map {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        Self {
            elements: Stack{elems: pairs
                .into_inner()
                .map(|x| Ast::parse(x, ctx))
                            .collect::<Arc<_>>()},

        }
    }
}


use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Map {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push_str("Set(");
        self.elements.elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push(')');
        result
    }
}

impl Eval<Flow> for Map {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        let mut result = vec![];
        let _ = self.elements.eval(&mut result, env, vars)?;
        
        for res in &result{
            match res{
                Values::List(l) if l.len() == 2 => (),
                 _ => return Err(EvalError::MapExprMustHaveListOfLen2)
            }
        }
        let ret = Values::Map(
            result.into_iter().map(|x|{
        match x {
            Values::List(mut l) => {
                let key = l.pop_front().unwrap();
                let value = l.pop_front().unwrap();
                (key,value)
            }
            _ => unreachable!()
        }}).collect::<BTreeMap<Values,Values>>()
);

        values.push(ret);
        Ok(Flow::Ok)
        
    }

    fn get_free_vars(&self,vars:&mut std::collections::HashSet<usize>) {
        self.elements.get_free_vars(vars);
    }

    fn get_vars(&self,vars:&mut std::collections::HashSet<usize>) {
        self.elements.get_vars(vars);

    }

    fn replace_vars(self,free_vars:& std::collections::HashSet<usize>,vars:&ChainMap)->Self {
           let elements = self.elements;
            let elems = elements.replace_vars(free_vars, vars);
            Self{elements : elems}
        
    }
    
    
}
