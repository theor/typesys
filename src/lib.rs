#[macro_use]
extern crate nom;
extern crate nom_locate;

pub mod errors;
pub mod ast;
pub mod types;
pub mod parser;
mod span;


#[cfg(test)]
mod tests {
    use parser::{NomSpan, Span};
   
    #[test]
    fn parse2() {
        let input = NomSpan::new("qwe");
        println!("{:?}", ::parser::type_id(input));
        // assert_eq!(::parser::type_id(b"qwe"),  ::parser::IResult::Done(&b""[..], "qwe"));
    }
    #[test]
    fn parse3() {
        let input = NomSpan::new("42");
        println!("{:?}", ::parser::prim(input));
        // assert_eq!(::parser::type_id(b"qwe"),  ::parser::IResult::Done(&b""[..], "qwe"));
    }
    #[test]
    fn parse_expr() {
        let input = NomSpan::new("42+2-3");
        println!("{:?}", ::parser::expr(input));
        // assert_eq!(::parser::type_id(b"qwe"),  ::parser::IResult::Done(&b""[..], "qwe"));
    }
    #[test]
    fn parse_parens() {
        let input = NomSpan::new("42+(2-3)");
        println!("{:?}", ::parser::expr(input));
        // assert_eq!(::parser::type_id(b"qwe"),  ::parser::IResult::Done(&b""[..], "qwe"));
    }
    #[test]
    fn parse_arrow() {
        use parser::Span;
        let input = NomSpan::new("a->b");
        println!("{:?}", ::parser::arrow(input));
        // assert_eq!(::parser::type_id(b"qwe"),  ::parser::IResult::Done(&b""[..], "qwe"));
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
            use super::ast::Expr::*;
            use super::ast::*;
            App(ra(Fun), ra(Expr(Prim(Number(2)))))
        };
        // use super::types::*;
        // use super::types::Primitive::*;
        // use super::types::Type::*;
        println!("{:?}", t);
        // assert_eq!(rt(Fun(rt(Prim(Number)), rt(Prim(Number)))),check(&a).unwrap());
    }
}
