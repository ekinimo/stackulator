

use crate::language::env::Env;
use crate::language::eval::{ChainMap, Eval, EvalError, Flow, Values};

use malachite::num::arithmetic::traits::*;
#[derive(Debug, Clone,PartialEq,PartialOrd,Ord,Eq)]
pub enum Primitives {
    Add,
    Sub,
    Mult,
    Div,


    Eq,
    Ge,
    Le,
    Geq,
    Leq,
    And,
    Or,
    Not,

    Eval,
    StackSize,

    Get,
    Set,
    Concat,
    PushFirst,
    Push,
    Pop,
    PopFirst,
    Delete,
    Insert,
    Len,
    Contains,

    Intersection,
    Union,
    Difference,
    SymmetricDifference,
    Subset,
    Superset,

    IntToFloat,
    FloatToInt,
    SetToList,
    ListToSet,
}

impl Eval<Flow> for Primitives {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<Flow, EvalError> {
        match self {
            Primitives::Add => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Float(a + b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a + b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::Sub => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Float(a - b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a - b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::Mult => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Float(a * b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a * b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::Div => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Float(a / b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a / b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }

            Primitives::Eq => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a == b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a == b)),
                    (Values::Bool(a), Values::Bool(b)) => values.push(Values::Bool(a == b)),
                    (Values::List(a), Values::List(b)) => values.push(Values::Bool(a == b)),
                    (Values::Set(a), Values::Set(b)) => values.push(Values::Bool(a == b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::Ge => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a > b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a > b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::Le => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a < b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a < b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::Geq => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a >= b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a >= b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::Leq => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a <= b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a <= b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }

            Primitives::And => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Bool(a), Values::Bool(b)) => values.push(Values::Bool(a && b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a & b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::Or => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Bool(a), Values::Bool(b)) => values.push(Values::Bool(a || b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a | b)),
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            }

            Primitives::Not => {
                if values.is_empty() {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let a = values.pop().unwrap();
                match a {
                    Values::Bool(a) => values.push(Values::Bool(!a)),
                    Values::Int(a) => values.push(Values::Int(!a)),
                    ty => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {}",ty.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::IntToFloat => {
                if values.is_empty() {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let a = values.pop().unwrap();
                match a {
                    Values::Int(a) => values.push(Values::Float(a.into())),
                    ty => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {}",ty.get_type()))),
                }
                Ok(Flow::Ok)
            }

            Primitives::FloatToInt => {
                if values.is_empty() {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let a = values.pop().unwrap();
                match a {
                    Values::Float(a) => values.push(Values::Int(a.ceiling())),
                    ty => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {}",ty.get_type()))),
                }
                Ok(Flow::Ok)
            }
            Primitives::Eval => {
                if values.is_empty() {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                if let Values::Stack(stack) = values.pop().unwrap() {
                    match stack.to_owned().eval(values, env, vars) {
                        ret @ Ok(_) => {
                            ret
                        }
                        Err(_) => {
                            Err(EvalError::PrimitiveEvalErr)
                        }
                    }
                } else {
                    Err(EvalError::PrimitiveTypeErr(self.clone(),"not stack".to_string()))
                }
            }
            Primitives::StackSize => {
                let l = values.len();
                values.push(Values::Int(l.into()));
                Ok(Flow::Ok)
            }
            Primitives::Get => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let idx = values.pop().unwrap();
                let list = values.pop().unwrap();
                match (list, idx) {
                    (Values::List(data), Values::Int(i)) => {
                        if i >= data.len() {
                            return Err(EvalError::PrimitiveEvalErr);
                        }
                        let idx = usize::try_from(&i).map_err(|_| EvalError::PrimitiveEvalErr)?;
                        let ret = data.get(idx).unwrap().clone();
                        let list = Values::List(data);
                        values.push(list);
                        values.push(ret);
                    }
                    (Values::Map(data), i) => {

                        let ret = data.get(&i).cloned();
                        let list = Values::Map(data);
                        values.push(list);
                        if ret.is_some(){
                        values.push(ret.unwrap());
                        }
                    },
                    (ty1,ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }

                Ok(Flow::Ok)
            }
            Primitives::Set => {
                if values.len() < 3 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let new_elem = values.pop().unwrap();
                let idx = values.pop().unwrap();
                let list = values.pop().unwrap();
                
                
                match (list, idx) {
                    (Values::List(mut data), Values::Int(i)) => {
                        if i >= data.len() {
                            return Err(EvalError::PrimitiveEvalErr);
                        }
                        let idx = usize::try_from(&i).map_err(|_| EvalError::PrimitiveEvalErr)?;
                        *data.get_mut(idx).unwrap() = new_elem;
                        let list = Values::List(data);
                        values.push(list);
                    }
                    (Values::Map(mut data), i) => {
                        data.insert(i, new_elem);
                        
                        let list = Values::Map(data);
                        values.push(list);
                    }
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                    
                }

                Ok(Flow::Ok)
            }
            Primitives::Concat => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let to_append = values.pop().unwrap();
                let orig  = values.pop().unwrap();
                match (orig, to_append) {
                    (Values::List(data1), Values::List(mut data2)) => {
                        //let mut data1 = data1.clone();
                        data1.to_owned().append(&mut data2);
                        let list = Values::List(data1);
                        values.push(list);
                    }
                    (Values::Set(data1), Values::Set(mut data2)) => {
                        //let mut data1 = data1.clone();
                        data1.to_owned().append(&mut data2);
                        let list = Values::Set(data1);
                        values.push(list);
                    }
                    (Values::Map(data1), Values::Map(mut data2)) => {
                        //let mut data1 = data1.clone();
                        data1.to_owned().append(&mut data2);
                        let list = Values::Map(data1);
                        values.push(list);
                    }
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }

                Ok(Flow::Ok)

            },
            Primitives::PushFirst => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let elem = values.pop().unwrap();
                let list = values.pop().unwrap();
                let last = if values.len() < 3 {values.pop()}else{None};
                match (last,list, elem) {
                    (_,Values::List(mut data), value) => {
                        data.push_back(value);
                        let list = Values::List(data);
                        values.push(list);
                    },

                    (_,Values::Set(mut data), value) => {
                        data.insert(value);
                        let list = Values::Set(data);
                        values.push(list);
                    },
                    (Some(Values::Map(mut data)), key,value) => {
                        if values.len() < 2 {
                            return Err(EvalError::PrimitiveUnderflow(self.clone()));
                        }
                        
                        data.insert(key,value);
                        let list = Values::Map(data);
                        values.push(list);
                    }
                    (ty0,ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {ty0:?} {} {} ",ty1.get_type(),ty2.get_type()))),
                }
                Ok(Flow::Ok)
            },
            Primitives::Push => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let elem = values.pop().unwrap();
                let list = values.pop().unwrap();
                let last = if values.len() < 3 {values.pop()}else{None};
                match (last,list, elem) {
                    (_,Values::List(mut data), value) => {
                        data.push_front(value);
                        let list = Values::List(data);
                        values.push(list);
                    },
                    (_,Values::Set(mut data), value) => {
                        data.insert(value);
                        let list = Values::Set(data);
                        values.push(list);
                    },
                    (Some(Values::Map(mut data)), key,value) => {
                        if values.len() < 2 {
                            return Err(EvalError::PrimitiveUnderflow(self.clone()));
                        }
                        
                        data.insert(key,value);
                        let list = Values::Map(data);
                        values.push(list);
                    },
                    (ty0,ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {ty0:?} {} {} ",ty1.get_type(),ty2.get_type()))),
                    
                }
                Ok(Flow::Ok)
            },
            Primitives::Len => {
                if values.is_empty() {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let list = values.pop().unwrap();
                match list {
                    Values::List(data) => {
                        let len = data.len();
                        let list = Values::List(data);
                        values.push(list);
                        values.push(Values::Int(len.into()));
                    },
                    Values::Set(data) => {
                        let len = data.len();
                        let list = Values::Set(data);
                        values.push(list);
                        values.push(Values::Int(len.into()));
                    },
                    Values::Map(data) => {
                        let len = data.len();
                        let list = Values::Map(data);
                        values.push(list);
                        values.push(Values::Int(len.into()));
                    },

                    ty => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {}",ty.get_type()))),
                }
                Ok(Flow::Ok)
                    
            },
            Primitives::Pop => {
                if values.is_empty() {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let list = values.pop().unwrap();
                match list {
                    Values::List(mut data) => {
                        if data.is_empty() {
                            return Err(EvalError::PrimitiveEvalErr)
                        }
                        let res = data.pop_front().unwrap();
                        let list = Values::List(data);
                        values.push(list);
                        values.push(res);
                    },
                    Values::Set(mut data) => {
                        if data.is_empty() {
                            return Err(EvalError::PrimitiveEvalErr)
                        }
                        let res = data.pop_last().unwrap();
                        let list = Values::Set(data);
                        values.push(list);
                        values.push(res);
                    },
                    Values::Map(mut data) => {
                        if data.is_empty() {
                            return Err(EvalError::PrimitiveEvalErr)
                        }
                        let (k,v) = data.pop_last().unwrap();
                        let ret = vec![k,v].into_iter().collect();
                        let map = Values::Map(data);
                        let list = Values::List(ret);
                        values.push(map);
                        values.push(list);
                    },
                    ty => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {}",ty.get_type()))),
                }
                Ok(Flow::Ok)
                    
            },
            Primitives::PopFirst => {
                if values.is_empty() {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let list = values.pop().unwrap();
                match list {
                    Values::List(mut data) => {
                        if data.is_empty() {
                            return Err(EvalError::PrimitiveEvalErr)
                        }
                        let res = data.pop_back().unwrap();
                        let list = Values::List(data);
                        values.push(list);
                        values.push(res);
                    },
                    Values::Set(mut data) => {
                        if data.is_empty() {
                            return Err(EvalError::PrimitiveEvalErr)
                        }
                        let res = data.pop_first().unwrap();
                        let list = Values::Set(data);
                        values.push(list);
                        values.push(res);
                    },
                    Values::Map(mut data) => {
                        if data.is_empty() {
                            return Err(EvalError::PrimitiveEvalErr)
                        }
                        let (k,v) = data.pop_first().unwrap();
                        let ret = vec![k,v].into_iter().collect();
                        let map = Values::Map(data);
                        let list = Values::List(ret);
                        values.push(map);
                        values.push(list);
                    },

                    ty => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {}",ty.get_type()))),
                }
                Ok(Flow::Ok)
                    
            },
            Primitives::Delete => {
                
                    if values.len() < 2 {
                        return Err(EvalError::PrimitiveUnderflow(self.clone()));
                    }
                    let idx = values.pop().unwrap();
                let list = values.pop().unwrap();

                match (list, idx) {
                        (Values::List(mut data), Values::Int(i)) => {
                            if i >= data.len() {
                                return Err(EvalError::PrimitiveEvalErr);
                            }
                            let idx = usize::try_from(&i).map_err(|_| EvalError::PrimitiveEvalErr)?;
                            let _ = data.remove(idx);
                            let list = Values::List(data);
                            values.push(list);
                            //values.push(ret);
                        }
                    (Values::Set(mut data), i) => {
                        
                        
                        let _ = data.remove(&i);
                        let list = Values::Set(data);
                        values.push(list);
                        
                    }
                    (Values::Map(mut data), i) => {
                        
                        
                        let _ = data.remove(&i);
                        let list = Values::Map(data);
                        values.push(list);
                        
                    }
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                    }

                    Ok(Flow::Ok)
                
            },
            Primitives::Contains => {
                
                    if values.len() < 2 {
                        return Err(EvalError::PrimitiveUnderflow(self.clone()));
                    }
                    let elem = values.pop().unwrap();
                let list = values.pop().unwrap();

                match (list, elem) {
                        (Values::List(data), elem) => {
                            let ret = data.contains(&elem);
                            let list = Values::List(data);
                            values.push(list);
                            values.push(Values::Bool(ret));
                        }
                    (Values::Set(data), elem) => {
                        let ret = data.contains(&elem);
                        let list = Values::Set(data);
                        values.push(list);
                        values.push(Values::Bool(ret));
                    }
                    (Values::Map(data), elem) => {
                        
                        let ret = data.contains_key(&elem);
                        let list = Values::Map(data);
                        values.push(list);
                        values.push(Values::Bool(ret));
                    }
                    
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                    }

                    Ok(Flow::Ok)
                
            },

            Primitives::Insert => {
                {
                    if values.len() < 3 {
                        return Err(EvalError::PrimitiveUnderflow(self.clone()));
                    }
                    let new_elem = values.pop().unwrap();

                    let idx = values.pop().unwrap();
                    let list = values.pop().unwrap();

                    match (list, idx) {
                        (Values::List(mut data), Values::Int(i)) => {
                            if i >= data.len() {
                                return Err(EvalError::PrimitiveEvalErr);
                            }
                            let idx = usize::try_from(&i).map_err(|_| EvalError::PrimitiveEvalErr)?;
                            data.insert(idx, new_elem);
                            let list = Values::List(data);
                            values.push(list);
                        }
                        (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                    }

                    Ok(Flow::Ok)
                }
            },
            Primitives::Union => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let to_append = values.pop().unwrap();
                let orig  = values.pop().unwrap();
                match (orig, to_append) {
                    (Values::Set(data1), Values::Set( data2)) => {
                        let ret = data1.union(&data2).cloned().collect();
                        let list = Values::Set(ret);
                        values.push(list);
                    }
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }

                Ok(Flow::Ok)
            },
            Primitives::Difference => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let to_append = values.pop().unwrap();
                let orig  = values.pop().unwrap();
                match (orig, to_append) {
                    (Values::Set(data1), Values::Set( data2)) => {
                        let ret = data1.difference(&data2).cloned().collect();
                        let list = Values::Set(ret);
                        values.push(list);
                    }
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                    
                }

                Ok(Flow::Ok)
            },
            Primitives::SymmetricDifference => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let to_append = values.pop().unwrap();
                let orig  = values.pop().unwrap();
                match (orig, to_append) {
                    (Values::Set(data1), Values::Set( data2)) => {
                        let ret = data1.symmetric_difference(&data2).cloned().collect();
                        let list = Values::Set(ret);
                        values.push(list);
                    }
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                    
                }

                Ok(Flow::Ok)
            },
            Primitives::Subset => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let to_append = values.pop().unwrap();
                let orig  = values.pop().unwrap();
                match (orig, to_append) {
                    (Values::Set(data1), Values::Set( data2)) => {
                        let ret = data1.is_subset(&data2);
                        let list = Values::Bool(ret);
                        values.push(list);
                    }
                    (ty1, ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }

                Ok(Flow::Ok)
            },
            Primitives::Intersection => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let to_append = values.pop().unwrap();
                let orig  = values.pop().unwrap();
                match (orig, to_append) {
                    (Values::Set(data1), Values::Set( data2)) => {
                        let ret = data1.intersection(&data2).cloned().collect();
                        let list = Values::Set(ret);
                        values.push(list);
                    }
                    (ty1,ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                }

                Ok(Flow::Ok)
            },
            Primitives::Superset => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let to_append = values.pop().unwrap();
                let orig  = values.pop().unwrap();
                match (orig, to_append) {
                    (Values::Set(data1), Values::Set( data2)) => {
                        let ret = data1.is_subset(&data2);
                        let list = Values::Bool(ret);
                        values.push(list);
                    }
                    (ty1,ty2) => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {} {} ",ty1.get_type(),ty2.get_type()))),
                    
                }

                Ok(Flow::Ok)
            },
            Primitives::SetToList => {
                if values.is_empty() {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let list = values.pop().unwrap();
                match list {
                    Values::Set(data) => {
                        let list = Values::List(data.into_iter().collect());
                        values.push(list);
                    },
                    ty => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {}",ty.get_type()))),
                }
                Ok(Flow::Ok)
            },
            Primitives::ListToSet => {
                if values.is_empty() {
                    return Err(EvalError::PrimitiveUnderflow(self.clone()));
                }
                let list = values.pop().unwrap();
                match list {
                    Values::List(data) => {
                        let list = Values::Set(data.into_iter().collect());
                        values.push(list);
                    },
                    ty => return Err(EvalError::PrimitiveTypeErr(self.clone(),format!("got {}",ty.get_type()))),
                }
                Ok(Flow::Ok)
            },
        }
    }

    fn get_free_vars(&self,_vars:&mut std::collections::HashSet<usize>) {
        
    }

    fn get_vars(&self,_vars:&mut std::collections::HashSet<usize>) {
        
    }

    fn replace_vars(self,_free_vars:& std::collections::HashSet<usize>,_vars:&ChainMap)->Self {
       self
    }

}

use crate::language::parse::{ParseCtx, Parse};
use crate::language::repr::Representation;
impl Representation<(), ParseCtx> for Primitives {
    fn get_repr(&self, _context: &ParseCtx) -> String {
        match self {
            Primitives::Add                 => "add"            ,
            Primitives::Sub                 => "sub"            ,
            Primitives::Mult                => "mul"            ,
            Primitives::Div                 => "div"            ,
            Primitives::Eq                  => "eq"             ,
            Primitives::Ge                  => "ge"             ,
            Primitives::Le                  => "le"             ,
            Primitives::Geq                 => "geq"            ,
            Primitives::Leq                 => "leq"            ,
            Primitives::And                 => "and"            ,
            Primitives::Or                  => "or"             ,
            Primitives::Not                 => "not"            ,
            Primitives::Eval                => "apply"          ,
            Primitives::IntToFloat          => "i2f"            ,
            Primitives::FloatToInt          => "f2i"            ,
            Primitives::StackSize           => "stack_size"     ,
            Primitives::Get                 => "get"            ,
            Primitives::Set                 => "set"            ,
            Primitives::Concat              => "concat"         ,
            Primitives::Push                => "push_first"     ,
            Primitives::PushFirst           => "push"           ,
            Primitives::PopFirst            => "pop"            ,
            Primitives::Pop                 => "pop_first"      ,
            Primitives::Delete              => "delete"         ,
            Primitives::Insert              => "insert"         ,
            Primitives::Len                 => "len"            ,
            Primitives::Contains            => "contains"       ,
            Primitives::Intersection        => "intersect"      ,
            Primitives::Union               => "union"          ,
            Primitives::Difference          => "difference"     ,
            Primitives::SymmetricDifference => "symmetric_diff" ,
            Primitives::Subset              => "is_subset"      ,
            Primitives::Superset            => "is_superset"    ,
            Primitives::SetToList           => "s2l"            ,
            Primitives::ListToSet           => "l2s"            ,
        }
        .to_string()
    }
}

impl Parse for Primitives{
    fn parse(pairs: pest::iterators::Pair<'_, crate::language::parse::Rule>, _ctx: &mut ParseCtx) -> Self {
                        let ret = match pairs.as_str() {

                       "add"                  =>Primitives::Add                 ,
                       "sub"                  =>Primitives::Sub                 ,
                       "mul"                  =>Primitives::Mult                ,
                       "div"                  =>Primitives::Div                 ,
                       "eq"                   =>Primitives::Eq                  ,
                       "ge"                   =>Primitives::Ge                  ,
                       "le"                   =>Primitives::Le                  ,
                       "geq"                  =>Primitives::Geq                 ,
                       "leq"                  =>Primitives::Leq                 ,
                       "and"                  =>Primitives::And                 ,
                       "or"                   =>Primitives::Or                  ,
                       "not"                  =>Primitives::Not                 ,
                       "apply"                =>Primitives::Eval                ,
                       "i2f"                  =>Primitives::IntToFloat          ,
                       "f2i"                  =>Primitives::FloatToInt          ,
                       "stack_size"           =>Primitives::StackSize           ,
                       "get"                  =>Primitives::Get                 ,
                       "set"                  =>Primitives::Set                 ,
                       "concat"               =>Primitives::Concat              ,
                       "push_first"           =>Primitives::Push                ,
                       "push"                 =>Primitives::PushFirst           ,
                       "pop"                  =>Primitives::PopFirst            ,
                       "pop_first"            =>Primitives::Pop                 ,
                       "delete"               =>Primitives::Delete              ,
                       "insert"               =>Primitives::Insert              ,
                       "len"                  =>Primitives::Len                 ,
                       "contains"             =>Primitives::Contains            ,
                       "intersect"            =>Primitives::Intersection        ,
                       "union"                =>Primitives::Union               ,
                       "difference"           =>Primitives::Difference          ,
                       "symmetric_diff"       =>Primitives::SymmetricDifference ,
                       "is_subset"            =>Primitives::Subset              ,
                       "is_superset"          =>Primitives::Superset            ,
                       "s2l"                  =>Primitives::SetToList           ,
                       "l2s"                  =>Primitives::ListToSet           ,




                            _ => unreachable!(),
                };
                ret

    }
}
