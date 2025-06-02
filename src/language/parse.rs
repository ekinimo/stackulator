use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ParseCtx {
    var_names: Vec<String>,
    fun_names: Vec<String>,
    type_names: Vec<String>,
    tag_names: Vec<String>,
    field_names: Vec<String>,
    var_idx: HashMap<String, usize>,
    fun_idx: HashMap<String, usize>,
    type_idx: HashMap<String, usize>,
    tag_idx: HashMap<String, usize>,
    field_idx: HashMap<String, usize>,
}

impl Default for ParseCtx {
    fn default() -> Self {
        let mut ret = Self {
            var_names: Default::default(),
            fun_names: Default::default(),
            type_names: Default::default(),
            tag_names: Default::default(),
            field_names: Default::default(),
            var_idx: Default::default(),
            fun_idx: Default::default(),
            type_idx: Default::default(),
            tag_idx: Default::default(),
            field_idx: Default::default(),
        };
        ret.insert_fun("add");
        ret.insert_fun("sub");
        ret.insert_fun("mul");
        ret.insert_fun("div");

        ret.insert_fun("eq");
        ret.insert_fun("neq");
        ret.insert_fun("geq");
        ret.insert_fun("leq");
        ret.insert_fun("ge");
        ret.insert_fun("le");

        ret.insert_fun("and");
        ret.insert_fun("or");
        ret.insert_fun("xor");
        ret.insert_fun("not");

        ret.insert_fun("stack_size");

        ret.insert_fun("get");
        ret.insert_fun("set");
        ret.insert_fun("concat");
        ret.insert_fun("push_first");
        ret.insert_fun("push");
        ret.insert_fun("pop_first");
        ret.insert_fun("pop");
        ret.insert_fun("delete");
        ret.insert_fun("insert");
        ret.insert_fun("len");
        ret.insert_fun("contains");

        ret.insert_fun("intersect");
        ret.insert_fun("union");
        ret.insert_fun("difference");
        ret.insert_fun("symmetric_difference");
        ret.insert_fun("subset");
        ret.insert_fun("superset");

        ret.insert_fun("map");
        ret.insert_fun("apply");
        ret
    }
}

impl ParseCtx {
    pub fn insert_var(&mut self, var: impl Into<String> + Clone) -> usize {
        match self.var_idx.entry(var.clone().into()) {
            std::collections::hash_map::Entry::Occupied(occ) => *occ.get(),
            std::collections::hash_map::Entry::Vacant(vac) => {
                let len = self.var_names.len();
                self.var_names.push(var.into());
                vac.insert(len);
                len
            }
        }
    }

    pub fn insert_fun(&mut self, fun: impl Into<String> + Clone) -> usize {
        match self.fun_idx.entry(fun.clone().into()) {
            std::collections::hash_map::Entry::Occupied(occ) => *occ.get(),
            std::collections::hash_map::Entry::Vacant(vac) => {
                let len = self.fun_names.len();
                self.fun_names.push(fun.into());
                vac.insert(len);
                len
            }
        }
    }

    pub fn insert_type(&mut self, fun: impl Into<String> + Clone) -> usize {
        match self.type_idx.entry(fun.clone().into()) {
            std::collections::hash_map::Entry::Occupied(occ) => *occ.get(),
            std::collections::hash_map::Entry::Vacant(vac) => {
                let len = self.type_names.len();
                self.type_names.push(fun.into());
                vac.insert(len);
                len
            }
        }
    }
    pub fn insert_tag(&mut self, fun: impl Into<String> + Clone) -> usize {
        match self.tag_idx.entry(fun.clone().into()) {
            std::collections::hash_map::Entry::Occupied(occ) => *occ.get(),
            std::collections::hash_map::Entry::Vacant(vac) => {
                let len = self.tag_names.len();
                self.tag_names.push(fun.into());
                vac.insert(len);
                len
            }
        }
    }

    pub fn insert_field(&mut self, fun: impl Into<String> + Clone) -> usize {
        match self.field_idx.entry(fun.clone().into()) {
            std::collections::hash_map::Entry::Occupied(occ) => *occ.get(),
            std::collections::hash_map::Entry::Vacant(vac) => {
                let len = self.field_names.len();
                self.field_names.push(fun.into());
                vac.insert(len);
                len
            }
        }
    }

    pub fn lookup_call_name(&self, i: usize) -> String {
        self.fun_names[i].to_string()
    }

    pub fn lookup_call_name_maybe(&self, i: usize) -> Option<String> {
        self.fun_names.get(i).map(|x| x.to_string())
    }

    pub fn lookup_tag_name(&self, i: usize) -> String {
        self.tag_names[i].to_string()
    }
    pub fn lookup_type_name(&self, i: usize) -> String {
        self.type_names[i].to_string()
    }

    pub fn lookup_var_name(&self, i: usize) -> String {
        self.var_names[i].to_string()
    }

    pub fn lookup_field_name(&self, i: usize) -> String {
        self.field_names[i].to_string()
    }
}

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct StackParser;

pub trait Parse {
    fn parse(pairs: pest::iterators::Pair<'_, Rule>, ctx: &mut ParseCtx) -> Self;
}
