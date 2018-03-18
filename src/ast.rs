use std::rc::Rc;


#[derive(Debug, Eq, PartialEq)]
pub enum Primitive {
    Number(i32),
    String(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Ast {
    Expr(Expr),
    Fun,
    App(RAst, RAst)
}

#[derive(Debug, Eq, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    Prim(Primitive),
    BinOp(BinOp, RAst, RAst),
}

pub type RAst = Rc<Ast>;

pub fn ra(t:Ast) -> RAst {
    Rc::new(t)
}
pub fn tnum() -> ::types::Type {
    ::types::Type::Prim(::types::Primitive::Number)
}
pub fn tstr() -> ::types::Type {
    ::types::Type::Prim(::types::Primitive::String)
}
