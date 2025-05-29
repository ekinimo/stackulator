use std::collections::{HashMap, HashSet};

use pest::error::Error;

use super::ast::*;
use super::env::{CallType, Env};
use super::parse::{Parse, ParseCtx, Rule, StackParser};
use crate::language::ast::stack::Stack;
use crate::language::eval::*;
use crate::language::repr::Representation;
use pest::Parser;

#[derive(Default, Clone)]
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
                Rule::protocol_def => {
                    let mut def = pair.into_inner();
                    let fun_name = self.parse_ctx.insert_fun(def.next().unwrap().as_str());
                    let typs: Vec<Type> = def
                        .next()
                        .unwrap()
                        .into_inner()
                        .map(|x| Type::parse(x, &mut self.parse_ctx))
                        .collect();
                    let expr = Stack {
                        elems: def.map(|x| Ast::parse(x, &mut self.parse_ctx)).collect(),
                    };
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        self.env.protocol_arity.entry(fun_name)
                    {
                        e.insert((typs.len(), None));
                        let mut map = HashMap::new();
                        map.insert(typs, (vec![], CallType::Stack(expr)));
                        self.env.protocol_data.insert(fun_name, map);
                    } else {
                        if self.env.protocol_arity[&fun_name].0 != typs.len() {
                            todo!("Error handling")
                        }
                        match self.env.protocol_data.get_mut(&fun_name) {
                            Some(r) => {
                                r.insert(typs, (vec![], CallType::Stack(expr)));
                            }
                            None => {
                                unreachable!()
                            }
                        }
                    }
                }
                Rule::structDef => {
                    let mut def = pair.into_inner();
                    let struct_name = self.parse_ctx.insert_type(def.next().unwrap().as_str());
                    let types: Vec<_> = def.map(|x| Type::parse(x, &mut self.parse_ctx)).collect();
                    self.env.typ_data.insert((struct_name, None), types);
                }
                Rule::enumDef => {
                    let mut def = pair.into_inner();
                    let enum_name = self.parse_ctx.insert_type(def.next().unwrap().as_str());

                    def.for_each(|variant| {
                        let mut def = variant.into_inner();
                        let variant_name = self.parse_ctx.insert_tag(def.next().unwrap().as_str());
                        let types: Vec<_> =
                            def.map(|x| Type::parse(x, &mut self.parse_ctx)).collect();
                        self.env
                            .typ_data
                            .insert((enum_name, Some(variant_name)), types);
                        match self.env.type_variants.entry(enum_name) {
                            std::collections::hash_map::Entry::Occupied(mut vec) => {
                                vec.get_mut().insert(variant_name);
                            }
                            std::collections::hash_map::Entry::Vacant(vacant) => {
                                let mut set = HashSet::new();
                                set.insert(variant_name);
                                vacant.insert(set);
                            }
                        }
                    });
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
            Rule::protocol_def => {
                let mut def = pair.into_inner();
                let fun_name = self.parse_ctx.insert_fun(def.next().unwrap().as_str());
                let typs: Vec<Type> = def
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|x| Type::parse(x, &mut self.parse_ctx))
                    .collect();
                dbg!(&typs);
                dbg!(&def);
                let expr = Stack {
                    elems: def.map(|x| Ast::parse(x, &mut self.parse_ctx)).collect(),
                };
                if let std::collections::hash_map::Entry::Vacant(e) =
                    self.env.protocol_arity.entry(fun_name)
                {
                    e.insert((typs.len(), None));
                    let mut map = HashMap::new();
                    map.insert(typs, (vec![], CallType::Stack(expr)));
                    self.env.protocol_data.insert(fun_name, map);
                } else {
                    if self.env.protocol_arity[&fun_name].0 != typs.len() {
                        todo!("Error handling")
                    }
                    match self.env.protocol_data.get_mut(&fun_name) {
                        Some(r) => {
                            r.insert(typs, (vec![], CallType::Stack(expr)));
                        }
                        None => {
                            unreachable!()
                        }
                    }
                }
            }

            Rule::def => {
                let mut def = pair.into_inner();
                let fun_name = self.parse_ctx.insert_fun(def.next().unwrap().as_str());
                let expr = Stack {
                    elems: def.map(|x| Ast::parse(x, &mut self.parse_ctx)).collect(),
                };
                self.env.data.insert(fun_name, expr);
            }
            Rule::structDef => {
                let mut def = pair.into_inner();
                let struct_name = self.parse_ctx.insert_type(def.next().unwrap().as_str());
                dbg!(&def);
                let types: Vec<_> = def
                    .map(|x| Type::parse(dbg!(x), &mut self.parse_ctx))
                    .collect();
                dbg!(&types);
                self.env.typ_data.insert((struct_name, None), types);
            }
            Rule::enumDef => {
                let mut def = pair.into_inner();
                let enum_name = self.parse_ctx.insert_type(def.next().unwrap().as_str());

                def.for_each(|variant| {
                    let mut def = variant.into_inner();
                    let variant_name = self.parse_ctx.insert_tag(def.next().unwrap().as_str());
                    let types: Vec<_> = def.map(|x| Type::parse(x, &mut self.parse_ctx)).collect();
                    self.env
                        .typ_data
                        .insert((enum_name, Some(variant_name)), types);
                    match self.env.type_variants.entry(enum_name) {
                        std::collections::hash_map::Entry::Occupied(mut vec) => {
                            vec.get_mut().insert(variant_name);
                        }
                        std::collections::hash_map::Entry::Vacant(vacant) => {
                            let mut set = HashSet::new();
                            set.insert(variant_name);
                            vacant.insert(set);
                        }
                    }
                });
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

    pub fn get_structs(&self) -> Vec<(String, Vec<String>)> {
        let mut ret = vec![];

        for ((type_name, maybe_variant), y) in self.env.typ_data.iter() {
            let name = self.parse_ctx.lookup_type_name(*type_name);
            match maybe_variant {
                Some(_variant) => {}
                None => {
                    let def = y.iter().map(|x| x.get_repr(&self.parse_ctx)).collect();
                    ret.push((name, def));
                }
            }
        }
        ret
    }

    pub fn get_enums(&self) -> Vec<(String, Vec<(String, Vec<String>)>)> {
        let mut map: HashMap<String, Vec<(String, Vec<String>)>> = HashMap::new();
        for ((type_name, maybe_variant), y) in self.env.typ_data.iter() {
            let name = self.parse_ctx.lookup_type_name(*type_name);
            let def = y.iter().map(|x| x.get_repr(&self.parse_ctx)).collect();

            match maybe_variant {
                Some(variant_name) => {
                    let variant_name = self.parse_ctx.lookup_tag_name(*variant_name);
                    let value = (variant_name, def);
                    match map.entry(name) {
                        std::collections::hash_map::Entry::Occupied(mut occ) => {
                            occ.get_mut().push(value);
                        }
                        std::collections::hash_map::Entry::Vacant(vac) => {
                            vac.insert(vec![value]);
                        }
                    }
                }
                None => (),
            }
        }
        map.into_iter().collect()
    }
}
