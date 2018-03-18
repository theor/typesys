use ast::*;
use errors::*;

use std::str;
use std::str::FromStr;
pub use nom::IResult;
use nom::{digit, is_alphabetic};
use ::nom_locate::LocatedSpan;
pub use span::*;


pub type ParseResult<I> = IResult<I, Ast, Error>;

// named!(pub ptype<Span, Span>,
//     map_res!(
//         ws!(take_while!(is_alphabetic)),
//         str::from_utf8
//     )
// );
// #[derive(Debug)]
// pub struct Type(Span);
#[derive(Debug)]
pub struct Arrow {
    a: Identifier,
    b: Identifier,
    loc: Span,
}

use ast::*;

named!(pub prim<NomSpan, Ast>,
    do_parse!(
        x: many1!(one_of!("0123456789")) >>
        (Ast::Prim(Primitive::Number(x.into_iter().collect::<String>().parse().unwrap())))
    )
);

named!(pub ltype<NomSpan, NomSpan>,
    recognize!(
        do_parse!(
            one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
            many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
            ()
        )
    )
);
named!(pub ptype<NomSpan, Identifier>,
    do_parse! (
        t: ltype >>
        (Identifier::from_nom_span(t))
    )
);

named!(pub arrow<NomSpan, Arrow>,
    // recognize!(
        do_parse!(
            a: ptype >>
            tag!("->") >>
            b: ptype >>
            (Arrow { loc: Span::from_to(a.span, b.span), a,b})
        )
    // )
);

// We parse any expr surrounded by parens, ignoring all whitespaces around those
named!(parens<i64>, ws!(delimited!( tag!("("), expr, tag!(")") )) );

// We transform an integer string into a i64, ignoring surrounding whitespaces
// We look for a digit suite, and try to convert it.
// If either str::from_utf8 or FromStr::from_str fail,
// we fallback to the parens parser defined above
named!(factor<i64>, alt!(
    map_res!(
      map_res!(
        ws!(digit),
        str::from_utf8
      ),
      FromStr::from_str
    )
  | parens
  )
);

// We read an initial factor and for each time we find
// a * or / operator followed by another factor, we do
// the math by folding everything
named!(term <i64>, do_parse!(
    init: factor >>
    res:  fold_many0!(
        pair!(alt!(tag!("*") | tag!("/")), factor),
        init,
        |acc, (op, val): (&[u8], i64)| {
            if (op[0] as char) == '*' { acc * val } else { acc / val }
        }
    ) >>
    (res)
  )
);

named!(pub expr <i64>, do_parse!(
    init: term >>
    res:  fold_many0!(
        pair!(alt!(tag!("+") | tag!("-")), term),
        init,
        |acc, (op, val): (&[u8], i64)| {
            if (op[0] as char) == '+' { acc + val } else { acc - val }
        }
    ) >>
    (res)
  )
);