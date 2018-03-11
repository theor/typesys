use std::rc::Rc;

pub enum Primitive {
    Number(i32),
    String(i32),
}
pub enum Ast {
    Prim(Primitive),
    Fun,
    App(RAst, RAst)
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
