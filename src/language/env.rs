use std::collections::HashMap;

use crate::language::ast::stack::Stack;

#[derive(Clone, Default, Debug)]
pub struct Env {
    pub data: HashMap<usize, Stack>,
}
