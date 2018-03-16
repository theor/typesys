use std::rc::Rc;
use ast::{Ast, Primitive as AstPrimitive, tnum, tstr};
use errors::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    Number,
    String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Prim(Primitive),
    Fun (RType, RType)
}

pub type RType = Rc<Type>;
pub fn rt(t:Type) -> RType {
    Rc::new(t)
}

pub type TypeCheckResult = Result<::types::RType, Error>;

pub fn check(t:&Ast) -> TypeCheckResult {
    // Ok(::types::Type::Prim(::types::Primitive::Number).into())

    match t {
        &Ast::Prim(ref p) => 
            match p {
                &AstPrimitive::Number(_) => Ok(tnum().into()),
                &AstPrimitive::String(_) => Ok(tstr().into()),
            },
        &Ast::Fun => Ok(Rc::new(::types::Type::Fun(Rc::new(tnum()), Rc::new(tstr())))),
        // &Ast:: App(ref , ref b) => ::types::Type::Prim(::types::Primitive::Number),
        &Ast:: App(ref app, ref _arg) => {
            let c = check(app.as_ref())?;
            match c.as_ref() {
                &::types::Type::Fun(ref _a, ref b) => Ok(b.clone()),
                _ => Err(Error::Typecheck),
            }
        },
        // _ => Err(Error::Typecheck),
    }
}