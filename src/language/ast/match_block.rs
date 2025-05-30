use super::stack::Stack;
use crate::language::env::Env;
use crate::language::eval::{ChainMap, Eval, EvalError, Flow, Values};

use malachite::{Integer, Rational};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum ListPattern {
    All(Option<usize>),
    FullList(Vec<Pattern>),
    StartEnd(Vec<Pattern>, Option<usize>, Vec<Pattern>),
    //Inside(usize,Vec<Pattern>,usize),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum SetPattern {
    All(Option<usize>),
    FullSet(Vec<Pattern>),
    Front(Vec<Pattern>, Option<usize>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum TypePattern {
    All(Option<usize>),
    FullList(Vec<Pattern>),
    StartEnd(Vec<Pattern>, Option<usize>, Vec<Pattern>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Pattern {
    Bool(bool),
    TypeBool(Option<usize>),
    Int(Integer),
    TypeInt(Option<usize>),
    Float(Rational),
    TypeFloat(Option<usize>),

    TypeStack(Option<usize>),

    TypeList(ListPattern),
    TypeSet(SetPattern),
    TypeMap(Option<usize>),
    NamedStruct(usize, TypePattern),
    NamedVariant(usize, usize, TypePattern),

    Variable(usize),
    DontCare,
}

impl Pattern {
    pub fn get_defined_vars(&self) -> Vec<usize> {
        use ListPattern::*;
        use Pattern::*;
        match self {
            TypeBool(Some(i)) => vec![*i],
            TypeInt(Some(i)) => vec![*i],
            TypeFloat(Some(i)) => vec![*i],
            TypeStack(Some(i)) => vec![*i],
            NamedStruct(_, TypePattern::All(Some(i))) => vec![*i],
            NamedStruct(_, TypePattern::FullList(v)) => v
                .iter()
                .flat_map(|v| v.get_defined_vars().into_iter())
                .collect(),
            NamedStruct(_, TypePattern::StartEnd(start, middle, end)) => start
                .iter()
                .chain(end)
                .flat_map(|v| v.get_defined_vars().into_iter())
                .chain(middle.iter().cloned())
                .collect(),
            NamedVariant(_, _, TypePattern::All(Some(i))) => vec![*i],
            NamedVariant(_, _, TypePattern::FullList(v)) => v
                .iter()
                .flat_map(|v| v.get_defined_vars().into_iter())
                .collect(),
            NamedVariant(_, _, TypePattern::StartEnd(start, middle, end)) => start
                .iter()
                .chain(end)
                .flat_map(|v| v.get_defined_vars().into_iter())
                .chain(middle.iter().cloned())
                .collect(),

            TypeList(All(Some(i))) => vec![*i],
            TypeList(FullList(v)) => v
                .iter()
                .flat_map(|v| v.get_defined_vars().into_iter())
                .collect(),
            TypeList(StartEnd(start, middle, end)) => start
                .iter()
                .chain(end)
                .flat_map(|v| v.get_defined_vars().into_iter())
                .chain(middle.iter().cloned())
                .collect(),

            TypeSet(SetPattern::All(Some(i))) => vec![*i],
            TypeSet(SetPattern::FullSet(v)) => v
                .iter()
                .flat_map(|v| v.get_defined_vars().into_iter())
                .collect(),
            TypeSet(SetPattern::Front(start, name)) => start
                .iter()
                .flat_map(|v| v.get_defined_vars().into_iter())
                .chain(name.iter().cloned())
                .collect(),

            TypeMap(_) => todo!(),
            Variable(i) => vec![*i],
            _ => vec![],
        }
    }
    pub fn pattern_match(&self, val: Values, vars: &mut ChainMap) -> Flow {
        dbg!((self, &val));
        match (self, val) {
            (Pattern::DontCare, _) => Flow::Ok,
            (Pattern::Int(x), Values::Int(y)) if *x == y => Flow::Ok,
            (Pattern::Float(x), Values::Float(y)) if *x == y => Flow::Ok,
            (Pattern::Bool(x), Values::Bool(y)) if *x == y => Flow::Ok,
            (Pattern::Variable(var), x) => {
                vars.insert(*var, x.to_owned());
                Flow::Ok
            }
            (Pattern::TypeInt(Some(var)), x @ Values::Int(_)) => {
                vars.insert(*var, x.to_owned());
                Flow::Ok
            }
            (Pattern::TypeInt(None), Values::Int(_)) => Flow::Ok,
            (Pattern::TypeFloat(Some(var)), x @ Values::Float(_)) => {
                vars.insert(*var, x.to_owned());
                Flow::Ok
            }
            (Pattern::TypeFloat(None), Values::Float(_)) => Flow::Ok,
            (Pattern::TypeBool(Some(var)), x @ Values::Bool(_)) => {
                vars.insert(*var, x.to_owned());
                Flow::Ok
            }
            (Pattern::TypeBool(None), Values::Bool(_)) => Flow::Ok,
            (Pattern::TypeStack(Some(var)), x @ Values::Stack(_)) => {
                vars.insert(*var, x.to_owned());
                Flow::Ok
            }
            (Pattern::TypeStack(None), Values::Stack(_)) => Flow::Ok,

            (Pattern::TypeList(ListPattern::All(None)), Values::List(_)) => Flow::Ok,
            (Pattern::TypeList(ListPattern::All(Some(var))), x @ Values::List(_)) => {
                vars.insert(*var, x.to_owned());
                Flow::Ok
            }

            (
                Pattern::NamedStruct(name, TypePattern::All(None)),
                Values::Custom {
                    name: name2,
                    tag: None,
                    values: _,
                },
            ) if *name == name2 => Flow::Ok,
            (
                Pattern::NamedStruct(name, TypePattern::All(Some(var))),
                Values::Custom {
                    name: name2,
                    tag: None,
                    values: Some(x),
                },
            ) if *name == name2 => {
                vars.insert(*var, Values::List(x.to_owned()));
                Flow::Ok
            }

            (
                Pattern::NamedVariant(name, tag, TypePattern::All(None)),
                Values::Custom {
                    name: name2,
                    tag: Some(tag1),
                    values: _,
                },
            ) if *name == name2 && *tag == tag1 => Flow::Ok,
            (
                Pattern::NamedVariant(name, tag, TypePattern::All(Some(var))),
                Values::Custom {
                    name: name2,
                    tag: Some(tag2),
                    values: Some(x),
                },
            ) if *name == name2 && *tag == tag2 => {
                vars.insert(*var, Values::List(x.to_owned()));
                Flow::Ok
            }

            (Pattern::TypeList(ListPattern::FullList(pats)), Values::List(mut x)) => {
                for pat in pats {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                Flow::Ok
            }
            (Pattern::TypeList(ListPattern::StartEnd(start, None, end)), Values::List(mut x)) => {
                for pat in start {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                for pat in end {
                    match x.pop_back().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                Flow::Ok
            }
            (
                Pattern::TypeList(ListPattern::StartEnd(start, Some(var), end)),
                Values::List(mut x),
            ) => {
                for pat in start {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }

                for pat in end.iter().rev() {
                    match x.pop_back().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }

                dbg!(&x);
                vars.insert(*var, Values::List(x.to_owned()));
                Flow::Ok
            }

            (Pattern::TypeSet(SetPattern::All(None)), Values::Set(_)) => Flow::Ok,
            (Pattern::TypeSet(SetPattern::All(Some(var))), x @ Values::Set(_)) => {
                vars.insert(*var, x.to_owned());
                Flow::Ok
            }
            (Pattern::TypeSet(SetPattern::FullSet(pats)), Values::Set(x)) => {
                /*for pat in pats {
                    match x.pop_first().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }*/
                let mut pats = pats.clone();
                let mut x = x.clone();
                let mut rest = BTreeSet::new();
                pats.sort();
                for pat in pats {
                    loop {
                        match x.pop_first() {
                            Some(x) => {
                                if matches!(pat.pattern_match(x.clone(), vars), Flow::Cont) {
                                    rest.insert(x);
                                } else {
                                    break;
                                }
                            }
                            None => {
                                return Flow::Cont;
                            }
                        }
                        x = rest.clone();
                    }
                }
                if rest.is_empty() {
                    Flow::Ok
                } else {
                    Flow::Cont
                }
            }
            (Pattern::TypeSet(SetPattern::Front(pats, Some(rest_name))), Values::Set(x)) => {
                /*for pat in start {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                for pat in end {
                    match x.pop_back().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                 */
                let mut pats = pats.clone();
                let mut x = x.clone();

                let mut rest = BTreeSet::new();
                pats.sort();
                for pat in pats {
                    loop {
                        match x.pop_first() {
                            Some(x) => {
                                if matches!(pat.pattern_match(x.clone(), vars), Flow::Cont) {
                                    rest.insert(x);
                                } else {
                                    break;
                                }
                            }
                            None => {
                                return Flow::Cont;
                            }
                        }
                        x = rest.clone();
                    }
                }
                vars.insert(*rest_name, Values::Set(rest));

                Flow::Ok
            }
            (Pattern::TypeSet(SetPattern::Front(pats, None)), Values::Set(x)) => {
                /*for pat in start {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                for pat in end {
                    match x.pop_back().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                vars.insert(*var, Values::Set(x.to_owned()));
                */
                let mut pats = pats.clone();
                let mut x = x.clone();

                let mut rest = BTreeSet::new();
                pats.sort();
                for pat in pats {
                    loop {
                        match x.pop_first() {
                            Some(x) => {
                                if matches!(pat.pattern_match(x.clone(), vars), Flow::Cont) {
                                    rest.insert(x);
                                } else {
                                    break;
                                }
                            }
                            None => {
                                return Flow::Cont;
                            }
                        }
                        x = rest.clone();
                    }
                }
                //vars.insert(*rest_name, Values::Set(rest));

                Flow::Ok
            }

            (
                Pattern::NamedStruct(pat_name, TypePattern::FullList(pats)),
                Values::Custom {
                    name,
                    tag: None,
                    values: Some(mut x),
                },
            ) if pat_name == &name => {
                for pat in pats.iter() {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                Flow::Ok
            }
            (
                Pattern::NamedStruct(pat_name, TypePattern::StartEnd(start, None, end)),
                Values::Custom {
                    name,
                    tag: None,
                    values: Some(mut x),
                },
            ) if pat_name == &name => {
                for pat in start {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                for pat in end {
                    match x.pop_back().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                Flow::Ok
            }
            (
                Pattern::NamedStruct(pat_name, TypePattern::StartEnd(start, Some(var), end)),
                Values::Custom {
                    name,
                    tag: None,
                    values: Some(mut x),
                },
            ) if pat_name == &name => {
                for pat in start {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }

                for pat in end.iter().rev() {
                    match x.pop_back().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }

                dbg!(&x);
                vars.insert(*var, Values::List(x.to_owned()));
                Flow::Ok
            }

            (
                Pattern::NamedVariant(pat_name, var_name, TypePattern::FullList(pats)),
                Values::Custom {
                    name,
                    tag: Some(var),
                    values: Some(mut x),
                },
            ) if pat_name == &name && var_name == &var => {
                for pat in pats.iter() {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                Flow::Ok
            }
            (
                Pattern::NamedVariant(pat_name, var_name, TypePattern::StartEnd(start, None, end)),
                Values::Custom {
                    name,
                    tag: Some(var),
                    values: Some(mut x),
                },
            ) if pat_name == &name && var_name == &var => {
                for pat in start {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                for pat in end {
                    match x.pop_back().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }
                Flow::Ok
            }
            (
                Pattern::NamedVariant(
                    pat_name,
                    var_name,
                    TypePattern::StartEnd(start, Some(var), end),
                ),
                Values::Custom {
                    name,
                    tag: Some(variant),
                    values: Some(mut x),
                },
            ) if pat_name == &name && var_name == &variant => {
                for pat in start {
                    match x.pop_front().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }

                for pat in end.iter().rev() {
                    match x.pop_back().map(|x| pat.pattern_match(x, vars)) {
                        None => return Flow::Cont,
                        _ => (),
                    }
                }

                dbg!(&x);
                vars.insert(*var, Values::List(x.to_owned()));
                Flow::Ok
            }

            (_, _) => Flow::Cont,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct MatchElem {
    pattern: Vec<Pattern>,
    cond: Stack,
    body: Stack,
}
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct Match {
    elems: Vec<MatchElem>,
}

impl Eval<Flow> for MatchElem {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        vars.push();

        for pat in self.pattern.iter().rev() {
            let val = values.pop().unwrap();
            match pat.pattern_match(val, vars) {
                Flow::Cont => return Ok(Flow::Cont),
                _ => {}
            }
        }

        if !self.cond.elems.is_empty() {
            let mut cond = vec![];

            match self.cond.eval(&mut cond, env, vars) {
                Ok(_) => {}
                Err(err) => {
                    return Err(EvalError::MatchCondFail(Box::new(err)));
                }
            }

            match cond.pop() {
                Some(Values::Bool(true)) => {}
                Some(Values::Bool(false)) => return Ok(Flow::Cont),
                Some(x) => {
                    return Err(EvalError::MatchCondExpectsBoolButGot(x.to_owned()));
                }
                None => {
                    return Err(EvalError::MatchCondUnderFlow);
                }
            };
        }

        let ret = self
            .body
            .eval(values, env, vars)
            .map_err(|err| EvalError::MatchArmFail(Box::new(err)));

        vars.pop();
        ret
    }

    fn get_free_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        self.get_vars(vars);

        let defined_vars: HashSet<usize> = self
            .pattern
            .iter()
            .flat_map(|x| x.get_defined_vars().into_iter())
            .collect();
        *vars = vars.difference(&defined_vars).cloned().collect();
    }

    fn get_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        self.cond.get_vars(vars);
        self.body.get_vars(vars);
    }

    fn replace_vars(self, free_vars: &std::collections::HashSet<usize>, vars: &ChainMap) -> Self {
        let MatchElem {
            pattern,
            mut cond,
            mut body,
        } = self;
        cond = cond.replace_vars(free_vars, vars);
        body = body.replace_vars(free_vars, vars);
        MatchElem {
            pattern,
            cond,
            body,
        }
    }
}

impl Eval<Flow> for Match {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        let max_len = self.elems.iter().map(|x| x.pattern.len()).max().unwrap();
        if max_len > values.len() {
            return Err(EvalError::MatchPatternUnderflow);
        }

        for arm in &self.elems {
            let mut temp_values = values.clone();
            match arm.eval(&mut temp_values, env, vars) {
                Ok(Flow::Cont) => {}
                Ok(ret @ Flow::Break | ret @ Flow::Ret | ret @ Flow::Ok) => {
                    *values = temp_values;
                    return Ok(ret);
                }
                Err(err) => {
                    return Err(EvalError::MatchBodyFail(Box::new(err)));
                }
            }
        }

        Err(EvalError::NoMatch)
    }

    fn get_free_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        let ret: HashSet<usize> = HashSet::new();
        *vars = self
            .elems
            .iter()
            .map(|x| {
                let mut vars = vars.clone();
                x.get_free_vars(&mut vars);
                vars
            })
            .fold(ret, |x, y| x.union(&y).cloned().collect());
    }

    fn get_vars(&self, vars: &mut std::collections::HashSet<usize>) {
        let ret: HashSet<usize> = HashSet::new();
        *vars = self
            .elems
            .iter()
            .map(|x| {
                let mut vars = vars.clone();
                x.get_free_vars(&mut vars);
                vars
            })
            .fold(ret, |x, y| x.union(&y).cloned().collect());
    }

    fn replace_vars(self, free_vars: &std::collections::HashSet<usize>, vars: &ChainMap) -> Self {
        let elements = self.elems;
        let elems = elements
            .into_iter()
            .map(|x| x.replace_vars(free_vars, vars))
            .collect();
        Self { elems }
    }
}

use crate::language::ast::Ast;
use crate::language::parse::{Parse, ParseCtx, Rule};
use std::collections::{BTreeSet, HashSet};
use std::str::FromStr;

impl Parse for Pattern {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        match pairs.as_rule() {
            Rule::integer => Pattern::Int(Integer::from_str(pairs.as_str()).unwrap()),
            Rule::float => Pattern::Float(Rational::from_str(pairs.as_str()).unwrap()),
            Rule::bools => Pattern::Bool("true" == pairs.as_str()),
            Rule::varName => Pattern::Variable(ctx.insert_var(pairs.as_str())),
            Rule::dontCare => Pattern::DontCare,
            Rule::intPattern => Pattern::TypeInt(Some(ctx.insert_var(pairs.as_str()))),
            Rule::intDontCarePattern => Pattern::TypeInt(None),
            Rule::ratPattern => Pattern::TypeInt(Some(ctx.insert_var(pairs.as_str()))),
            Rule::ratDontCarePattern => Pattern::TypeFloat(None),
            Rule::boolPattern => Pattern::TypeBool(Some(ctx.insert_var(pairs.as_str()))),
            Rule::boolDontCarePattern => Pattern::TypeBool(None),
            Rule::stackPattern => Pattern::TypeStack(Some(ctx.insert_var(pairs.as_str()))),
            Rule::stackDontCarePattern => Pattern::TypeStack(None),
            Rule::listAllpattern => {
                Pattern::TypeList(ListPattern::All(Some(ctx.insert_var(pairs.as_str()))))
            }
            Rule::listAllDontCarepattern => Pattern::TypeList(ListPattern::All(None)),
            Rule::setAllDontCarepattern => Pattern::TypeSet(SetPattern::All(None)),

            Rule::structAllDontCarepattern => {
                let mut pairs = pairs.into_inner();
                let name = ctx.insert_type(pairs.next().unwrap().as_str());

                Pattern::NamedStruct(name, TypePattern::All(None))
            }
            Rule::enumAllDontCarepattern => {
                let mut pairs = pairs.into_inner();
                let mut pairs = pairs.next().unwrap().into_inner();
                let name = ctx.insert_type(dbg!(pairs.next().unwrap().as_str()));

                let tag = ctx.insert_tag(dbg!(pairs.next().unwrap().as_str()));

                Pattern::NamedVariant(name, tag, TypePattern::All(None))
            }

            Rule::structAllpattern => {
                let mut pairs = pairs.into_inner();
                let name = ctx.insert_type(pairs.next().unwrap().as_str());
                let var = ctx.insert_var(pairs.next().unwrap().as_str());

                Pattern::NamedStruct(name, TypePattern::All(Some(var)))
            }
            Rule::enumAllpattern => {
                let mut pair = pairs.into_inner();
                let mut pairs = pair.next().unwrap().into_inner();
                let name = ctx.insert_type(dbg!(pairs.next().unwrap().as_str()));

                let tag = ctx.insert_tag(dbg!(pairs.next().unwrap().as_str()));
                let var = ctx.insert_var(pairs.next().unwrap().as_str());

                Pattern::NamedVariant(name, tag, TypePattern::All(Some(var)))
            }

            Rule::structFullPattern => {
                let mut pairs = pairs.into_inner();
                let name = ctx.insert_type(pairs.next().unwrap().as_str());
                Pattern::NamedStruct(
                    name,
                    TypePattern::FullList(pairs.map(|x| Self::parse(x, ctx)).collect()),
                )
            }

            Rule::enumFullPattern => {
                let mut pairs = pairs.into_inner();
                let mut names = pairs.next().unwrap().into_inner();
                let name = ctx.insert_type(dbg!(names.next().unwrap().as_str()));

                let tag = ctx.insert_tag(dbg!(names.next().unwrap().as_str()));

                Pattern::NamedVariant(
                    name,
                    tag,
                    TypePattern::FullList(pairs.map(|x| Self::parse(x, ctx)).collect()),
                )
            }

            Rule::structStartEnd => {
                let mut inner = pairs.into_inner();
                let struct_name = ctx.insert_type(inner.next().unwrap().as_str());
                let mut start = vec![];
                let mut end = vec![];
                let mut name = None;
                let mut has_seen = false;
                for val in inner {
                    match val.as_rule() {
                        Rule::manyvar => {
                            name = Some(val);
                            has_seen = true;
                        }

                        _ => {
                            if !has_seen {
                                start.push(Self::parse(val, ctx));
                            } else {
                                end.push(Self::parse(val, ctx));
                            }
                        }
                    }
                }

                Pattern::NamedStruct(
                    struct_name,
                    TypePattern::StartEnd(
                        start,
                        Some(ctx.insert_var(&name.unwrap().as_str()[1..])),
                        end,
                    ),
                )
            }

            Rule::enumStartEnd => {
                let mut inner = pairs.into_inner();
                let mut names = inner.next().unwrap().into_inner();
                let enum_name = ctx.insert_type(dbg!(names.next().unwrap().as_str()));

                let tag = ctx.insert_tag(dbg!(names.next().unwrap().as_str()));

                let mut start = vec![];
                let mut end = vec![];
                let mut name = None;
                let mut has_seen = false;
                for val in inner {
                    match val.as_rule() {
                        Rule::manyvar => {
                            name = Some(val);
                            has_seen = true;
                        }

                        _ => {
                            if !has_seen {
                                start.push(Self::parse(val, ctx));
                            } else {
                                end.push(Self::parse(val, ctx));
                            }
                        }
                    }
                }

                Pattern::NamedVariant(
                    enum_name,
                    tag,
                    TypePattern::StartEnd(
                        start,
                        Some(ctx.insert_var(&name.unwrap().as_str()[1..])),
                        end,
                    ),
                )
            }

            Rule::structStartEndDontCare => {
                let mut inner = pairs.into_inner();
                let struct_name = ctx.insert_type(inner.next().unwrap().as_str());
                let mut start = vec![];
                let mut end = vec![];
                let mut has_seen = false;
                for val in inner {
                    match val.as_rule() {
                        Rule::many => {
                            has_seen = true;
                        }

                        _ => {
                            if !has_seen {
                                start.push(Self::parse(val, ctx));
                            } else {
                                end.push(Self::parse(val, ctx));
                            }
                        }
                    }
                }

                Pattern::NamedStruct(struct_name, TypePattern::StartEnd(start, None, end))
            }

            Rule::enumStartEndDontCare => {
                let mut inner = pairs.into_inner();
                let mut names = inner.next().unwrap().into_inner();
                let enum_name = ctx.insert_type(dbg!(names.next().unwrap().as_str()));

                let tag = ctx.insert_tag(dbg!(names.next().unwrap().as_str()));

                let mut start = vec![];
                let mut end = vec![];
                let mut has_seen = false;
                for val in inner {
                    match val.as_rule() {
                        Rule::many => {
                            has_seen = true;
                        }

                        _ => {
                            if !has_seen {
                                start.push(Self::parse(val, ctx));
                            } else {
                                end.push(Self::parse(val, ctx));
                            }
                        }
                    }
                }

                Pattern::NamedVariant(enum_name, tag, TypePattern::StartEnd(start, None, end))
            }

            Rule::listFullPattern => Pattern::TypeList(ListPattern::FullList(
                pairs.into_inner().map(|x| Self::parse(x, ctx)).collect(),
            )),

            Rule::setFullPattern => Pattern::TypeSet(SetPattern::FullSet(
                pairs.into_inner().map(|x| Self::parse(x, ctx)).collect(),
            )),
            Rule::listStart => {
                let inner = pairs.into_inner();
                let mut start = vec![];
                let mut name = None;

                for val in inner {
                    match val.as_rule() {
                        Rule::manyvar => {
                            name = Some(val);
                            break;
                        }

                        _ => start.push(Self::parse(val, ctx)),
                    }
                }
                Pattern::TypeList(ListPattern::StartEnd(
                    start,
                    Some(ctx.insert_var(&name.unwrap().as_str()[1..])),
                    vec![],
                ))
            }
            Rule::listStartDontCare => {
                let inner = pairs.into_inner();
                let mut start = vec![];
                for val in inner {
                    match val.as_rule() {
                        Rule::many => {
                            //name=Some(val);
                            break;
                        }

                        _ => start.push(Self::parse(val, ctx)),
                    }
                }

                Pattern::TypeList(ListPattern::StartEnd(start, None, vec![]))
            }
            Rule::listEnd => {
                let inner = pairs.into_inner();
                let mut start = vec![];
                let mut name = None;

                for val in inner {
                    match val.as_rule() {
                        Rule::manyvar => {
                            name = Some(val);
                        }

                        _ => start.push(Self::parse(val, ctx)),
                    }
                }

                Pattern::TypeList(ListPattern::StartEnd(
                    vec![],
                    Some(ctx.insert_var(&name.unwrap().as_str()[1..])),
                    start,
                ))
            }
            Rule::listEndDontCare => {
                let inner = pairs.into_inner();
                let mut start = vec![];
                for val in inner {
                    match val.as_rule() {
                        Rule::many => {
                            //name=Some(val);
                        }

                        _ => start.push(Self::parse(val, ctx)),
                    }
                }

                Pattern::TypeList(ListPattern::StartEnd(vec![], None, start))
            }

            Rule::listStartEnd => {
                let inner = pairs.into_inner();
                let mut start = vec![];
                let mut end = vec![];
                let mut name = None;
                let mut has_seen = false;
                for val in inner {
                    match val.as_rule() {
                        Rule::manyvar => {
                            name = Some(val);
                            has_seen = true;
                        }

                        _ => {
                            if !has_seen {
                                start.push(Self::parse(val, ctx));
                            } else {
                                end.push(Self::parse(val, ctx));
                            }
                        }
                    }
                }

                Pattern::TypeList(ListPattern::StartEnd(
                    start,
                    Some(ctx.insert_var(&name.unwrap().as_str()[1..])),
                    end,
                ))
            }

            Rule::setFront => {
                let inner = pairs.into_inner();
                let mut start = vec![];

                let mut name = None;
                for val in inner {
                    match val.as_rule() {
                        Rule::manyvar => {
                            name = Some(val);
                        }
                        _ => {
                            start.push(Self::parse(val, ctx));
                        }
                    }
                }

                Pattern::TypeSet(SetPattern::Front(
                    start,
                    Some(ctx.insert_var(&name.unwrap().as_str()[1..])),
                ))
            }
            Rule::listStartEndDontCare => {
                let inner = pairs.into_inner();
                let mut start = vec![];
                let mut end = vec![];
                let mut has_seen = false;
                for val in inner {
                    match val.as_rule() {
                        Rule::many => {
                            has_seen = true;
                        }

                        _ => {
                            if !has_seen {
                                start.push(Self::parse(val, ctx));
                            } else {
                                end.push(Self::parse(val, ctx));
                            }
                        }
                    }
                }

                Pattern::TypeList(ListPattern::StartEnd(start, None, end))
            }
            Rule::setFrontDontCare => {
                let inner = pairs.into_inner();
                let mut start = vec![];

                for val in inner {
                    match val.as_rule() {
                        Rule::manyvar => {}
                        _ => {
                            start.push(Self::parse(val, ctx));
                        }
                    }
                }

                Pattern::TypeSet(SetPattern::Front(start, None))
            }

            x => {
                dbg!(x);
                unreachable!()
            }
        }
    }
}

impl Parse for MatchElem {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        let mut inners = pairs.into_inner();
        let pattern: Vec<Pattern> = inners
            .next()
            .unwrap()
            .into_inner()
            .map(|x| Pattern::parse(x, ctx))
            .collect();
        let cond_or_action = inners.next().unwrap();
        if let Some(action) = inners.next() {
            let cond = Stack {
                elems: cond_or_action
                    .into_inner()
                    .map(|x| Ast::parse(x, ctx))
                    .collect(),
            };
            let body = Stack {
                elems: action.into_inner().map(|x| Ast::parse(x, ctx)).collect(),
            };
            MatchElem {
                pattern,
                cond,
                body,
            }
        } else {
            let condition = vec![];
            let body = Stack {
                elems: cond_or_action
                    .into_inner()
                    .map(|x| Ast::parse(x, ctx))
                    .collect(),
            };
            MatchElem {
                pattern,
                cond: Stack {
                    elems: (condition.into_iter().collect()),
                },
                body,
            }
        }
    }
}

impl Parse for Match {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self {
        Match {
            elems: pairs
                .into_inner()
                .map(|x| MatchElem::parse(x, ctx))
                .collect(),
        }
    }
}

use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Pattern {
    fn get_repr(&self, context: &ParseCtx) -> String {
        match self {
            Pattern::DontCare => "_".to_string(),
            Pattern::Int(i) => format!("{i}"),
            Pattern::Float(i) => format!("{i}"),
            Pattern::Bool(i) => format!("{i}"),
            Pattern::Variable(i) => context.lookup_var_name(*i),
            Pattern::TypeBool(None) => "todo!()".to_string(),
            Pattern::TypeBool(Some(_i)) => "todo!()".to_string(),
            Pattern::TypeInt(None) => "todo!()".to_string(),
            Pattern::TypeInt(Some(_i)) => "todo!()".to_string(),
            Pattern::TypeFloat(None) => "todo!()".to_string(),
            Pattern::TypeFloat(Some(_i)) => "todo!()".to_string(),
            Pattern::TypeStack(None) => "todo!()".to_string(),
            Pattern::TypeStack(Some(_i)) => "todo!()".to_string(),
            Pattern::TypeList(_) => "todo!()".to_string(),
            Pattern::TypeSet(_) => "todo!()".to_string(),
            Pattern::TypeMap(_) => "todo!()".to_string(),
            Pattern::NamedStruct(t, p) => {
                format!("{},({})", context.lookup_type_name(*t), p.get_repr(context))
            }
            Pattern::NamedVariant(_, _, _) => "todo!()".to_string(),
        }
    }
}

impl Representation<(), ParseCtx> for TypePattern {
    fn get_repr(&self, context: &ParseCtx) -> String {
        "todo!()".to_string()
    }
}

impl Representation<(), ParseCtx> for MatchElem {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push_str("| ");
        self.pattern
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push_str("=> ");
        self.body
            .elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result.push_str(", ");
        result
    }
}
impl Representation<(), ParseCtx> for Match {
    fn get_repr(&self, context: &ParseCtx) -> String {
        let mut result = String::new();
        result.push_str("match ");
        self.elems
            .iter()
            .for_each(|x| result.push_str(&format!(" {} ", x.get_repr(context))));
        result
    }
}
