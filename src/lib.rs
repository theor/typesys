pub mod errors;
pub mod ast;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::types::check;

        let t = {
            use super::types::Primitive::*;
            use super::types::Type::*;
            use super::types::*;
            Fun(rt(Prim(Number)), rt(Prim(Number)))
        };
        let a = {
            use super::ast::Primitive::*;
            use super::ast::Ast::*;
            use super::ast::*;
            App(ra(Fun), ra(Prim(Number(2))))
        };
        println!("{:?}", t);
        println!("{:?}",check(&a));
    }
}
