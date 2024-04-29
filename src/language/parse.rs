use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ParseCtx {
    var_names: Vec<String>,
    fun_names: Vec<String>,
    var_idx: HashMap<String, usize>,
    fun_idx: HashMap<String, usize>,
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

    pub fn lookup_call_name(&self, i: usize) -> String {
        self.fun_names[i].to_string()
    }
    pub fn lookup_var_name(&self, i: usize) -> String {
        self.var_names[i].to_string()
    }
}

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct StackParser;

pub trait Parse{
    
    fn parse<'a>(pairs: pest::iterators::Pair<'a, Rule>, ctx: &mut ParseCtx) -> Self;
}
