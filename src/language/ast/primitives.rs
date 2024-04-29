use crate::language::eval::{Eval,EvalError,ChainMap,Values};
use crate::language::env::Env;
use malachite::num::arithmetic::traits::{*};



#[derive(Debug, Clone)]
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
    IntToFloat,
    FloatToInt,
}


impl Eval<()> for Primitives {
    fn eval(
        &self,
        values: &mut Vec<Values>,
        env: &Env,
        vars: &mut ChainMap,
    ) -> Result<(), EvalError> {
        match self {
            Primitives::Add => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Float(a + b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a + b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::Sub => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Float(a - b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a - b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::Mult => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Float(a * b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a * b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::Div => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Float(a / b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a / b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }

            Primitives::Eq => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a == b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a == b)),
                    (Values::Bool(a), Values::Bool(b)) => values.push(Values::Bool(a == b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::Ge => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a > b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a > b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::Le => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a < b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a < b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::Geq => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a >= b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a >= b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::Leq => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Float(a), Values::Float(b)) => values.push(Values::Bool(a <= b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Bool(a <= b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }

            Primitives::And => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Bool(a), Values::Bool(b)) => values.push(Values::Bool(a && b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a & b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::Or => {
                if values.len() < 2 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                match (a, b) {
                    (Values::Bool(a), Values::Bool(b)) => values.push(Values::Bool(a || b)),
                    (Values::Int(a), Values::Int(b)) => values.push(Values::Int(a | b)),
                    (_, _) => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }

            Primitives::Not => {
                if values.len() < 1 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let a = values.pop().unwrap();
                match a {
                    Values::Bool(a) => values.push(Values::Bool(!a)),
                    Values::Int(a) => values.push(Values::Int(!a)),
                    _ => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::IntToFloat => {
                if values.len() < 1 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let a = values.pop().unwrap();
                match a {
                    Values::Int(a) => values.push(Values::Float(a.into() )),
                    _ => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }

            Primitives::FloatToInt => {
                if values.len() < 1 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                let a = values.pop().unwrap();
                match a {
                    Values::Float(a) => values.push(Values::Int(
                        a.ceiling().into()
                        )),
                    _ => return Err(EvalError::PrimitiveTypeErr),
                }
                Ok(())
            }
            Primitives::Eval => {
                if values.len() < 1 {
                    return Err(EvalError::PrimitiveUnderflow);
                }
                if let Values::Stack(stack) = values.pop().unwrap() {
                    match stack.to_owned().eval(values, env, vars) {
                        Ok(_) => {}
                        Err(_) => {
                            return Err(EvalError::PrimitiveEvalErr);
                        }
                    };
                } else {
                    return Err(EvalError::PrimitiveTypeErr);
                }

                Ok(())
            }
        }
    }
}


use crate::language::repr::Representation;
use crate::language::parse::ParseCtx;
impl Representation<(), ParseCtx> for Primitives {
    fn get_repr(&self, _context: &ParseCtx) -> String {
        match self {
            Primitives::Add => "add",
            Primitives::Sub => "sub",
            Primitives::Mult => "mul",
            Primitives::Div => "div",
            Primitives::Eq => "eq",
            Primitives::Ge => "ge",
            Primitives::Le => "le",
            Primitives::Geq => "geq",
            Primitives::Leq => "leq",
            Primitives::And => "and",
            Primitives::Or => "or",
            Primitives::Not => "not",
            Primitives::Eval => "apply",
            Primitives::IntToFloat => "i2f",
            Primitives::FloatToInt => "f2i",
        }
        .to_string()
    }
}
