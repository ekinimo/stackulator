use super::Ast;
use crate::language::env::Env;
use crate::language::eval::{ChainMap, Eval, EvalError, Values};
use crate::language::parse::{Parse, ParseCtx, Rule};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Stack {
    pub elems: Arc<[Ast]>,
}

impl Default for Stack {
    fn default() -> Self {
        let elems = vec![].into();
        Self { elems }
    }
}

impl Eval<()> for Stack {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<(), EvalError> {
        for elem in self.elems.iter() {
            match elem {
                Ast::While(block) => block.eval(values, env, vars)?,
                Ast::IfTrue(block) => block.eval(values, env, vars)?,
                Ast::Take(take) => take.eval(values, env, vars)?,
                Ast::Match(arms) => arms.eval(values, env, vars)?,

                Ast::Call(fun_name) => {
                    let name = env.data.get(fun_name);
                    match name {
                        Some(func) => {
                            if let Err(err) = func.eval(values, env, vars) {
                                return Err(EvalError::FuncCallFail(Box::new((*fun_name, err))));
                            }
                        }

                        None => return Err(EvalError::UndefinedCall(*fun_name)),
                    }
                }
                Ast::PrimitiveCall(p) => p.eval(values, env, vars)?,
                Ast::Var(var) => match vars.lookup(var) {
                    Some(val) => values.push(val),
                    None => return Err(EvalError::UndefinedVariable(*var)),
                },
                Ast::Float(f) => values.push(Values::Float(f.clone())),
                Ast::Int(i) => values.push(Values::Int(i.clone())),
                Ast::Bool(b) => values.push(Values::Bool(*b)),
                Ast::Stack(s) => values.push(Values::Stack(s.clone())),
            }
        }
        Ok(())
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
