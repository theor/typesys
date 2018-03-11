use ast::*;
use errors::*;

use std::str;
use std::str::FromStr;
pub use nom::IResult;
use nom::{digit, is_alphabetic};

pub type ParseResult<I> = IResult<I, Ast, Error>;

named!(pub ptype<&str>,
    map_res!(
        ws!(take_while!(is_alphabetic)),
        str::from_utf8
    )
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