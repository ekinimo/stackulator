use pest::error::Error;

use super::ast::*;
use super::env::Env;
use super::parse::{Parse, ParseCtx, Rule, StackParser};
use crate::language::ast::stack::Stack;
use crate::language::eval::*;
use crate::language::repr::Representation;
use pest::Parser;

#[derive(Clone, Debug, Default)]
pub struct VM {
    pub stack: Vec<Values>,
    pub var_map: ChainMap,
    pub env: Env,
    pub exprs: Vec<Stack>,
    pub parse_ctx: ParseCtx,
}

impl VM {
    pub fn parse_full_program(&mut self, source: &str) -> Result<(), Box<Error<Rule>>> {
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

    pub fn parse_snippet(&mut self, source: &str) -> Result<(), Box<Error<Rule>>> {
        let pair = StackParser::parse(Rule::justExprOrDef, source)?
            .next()
            .unwrap();
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
