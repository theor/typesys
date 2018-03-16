#[macro_use]
extern crate nom;
extern crate nom_locate;

pub mod errors;
pub mod ast;
pub mod types;
pub mod parser;


#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(::parser::expr(b"12+6-4+3"), ::parser::IResult::Done(&b""[..], 17));
    }
    #[test]
    fn parse2() {
        use parser::Span;
        let input = Span::new("qwe");
        println!("{:?}", ::parser::ptype(input));
        // assert_eq!(::parser::ptype(b"qwe"),  ::parser::IResult::Done(&b""[..], "qwe"));
    }
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
        use super::types::*;
        use super::types::Primitive::*;
        use super::types::Type::*;
        println!("{:?}", t);
        assert_eq!(rt(Fun(rt(Prim(Number)), rt(Prim(Number)))),check(&a).unwrap());
    }
}
