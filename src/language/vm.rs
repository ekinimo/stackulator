use pest::error::Error;

use crate::language::eval::{*};
use crate::language::ast::stack::Stack;
use super::parse::{Parse,ParseCtx, StackParser, Rule};
use super::env::Env;
use super::ast::{*};
use pest::Parser;
use crate::language::repr::Representation;

#[derive(Clone, Debug, Default)]
pub struct VM {
    pub stack: Vec<Values>,
    pub var_map: ChainMap,
    pub env: Env,
    pub exprs: Vec<Stack>,
    pub parse_ctx: ParseCtx,
}


impl VM {
    pub fn parse_full_program<'a>(&mut self, source: &'a str) -> Result<(), Error<Rule>> {
        let pairs = StackParser::parse(Rule::defsAndExprs, source)?;
        for pair in pairs {
            match pair.as_rule() {
                Rule::expr => self.exprs.push(Stack {
                    elems: pair
                        .into_inner()
                        .map(|x| Ast::parse(x, &mut self.parse_ctx))
                        .collect(),
                }),
                Rule::def => {
                    let mut def = pair.into_inner();
                    let fun_name = self.parse_ctx.insert_fun(def.next().unwrap().as_str());
                    let expr = Stack {
                        elems: def.map(|x| Ast::parse(x, &mut self.parse_ctx)).collect(),
                    };
                    self.env.data.insert(fun_name, expr);
                }
                Rule::EOI => (),
                x => {
                    dbg!(x);
                    unreachable!();
                }
            }
        }
        Ok(())
    }

    pub fn parse_snippet<'a>(&mut self, source: &'a str) -> Result<(), Error<Rule>> {
        let pair = StackParser::parse(Rule::justExprOrDef, source)?.next().unwrap();
            match pair.as_rule() {
                Rule::expr => self.exprs.push(Stack {
                    elems: pair
                        .into_inner()
                        .map(|x| Ast::parse(x, &mut self.parse_ctx))
                        .collect(),
                }),
                Rule::def => {
                    let mut def = pair.into_inner();
                    let fun_name = self.parse_ctx.insert_fun(def.next().unwrap().as_str());
                    let expr = Stack {
                        elems: def.map(|x| Ast::parse(x, &mut self.parse_ctx)).collect(),
                    };
                    self.env.data.insert(fun_name, expr);
                }
                Rule::EOI => (),
                x => {
                    dbg!(x);
                    unreachable!();
                }
            }
        Ok(())
    }

    pub fn eval(&mut self) -> Result<(), EvalError> {
        for expr in &self.exprs {
            match expr.eval(&mut self.stack, &self.env, &mut self.var_map) {
                Ok(_) => {}
                Err(err) => {
                    self.exprs.clear();
                    return Err(err);
                }
            };
        }
        self.exprs.clear();
        Ok(())
    }

    pub fn get_definitons(&self) -> Vec<(String, Vec<String>)> {
        let mut ret = vec![];
        for (x, y) in self.env.data.iter() {
            ret.push((
                self.parse_ctx.lookup_call_name(*x),
                y.elems
                    .iter()
                    .map(|x| x.get_repr(&self.parse_ctx))
                    .collect(),
            ))
        }
        ret
    }
}
