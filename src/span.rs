use ::nom_locate::LocatedSpan;
use ::std::fmt;

pub type NomSpan<'a> = LocatedSpan<&'a str>;

pub trait Spanned {
    fn get_span(&self) -> Span;
}

macro_rules! impl_spanned {
    ($t:ty) => (
        impl Spanned for $t {
            fn get_span(&self) -> Span {
                self.span
            }
        }
    )
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Span {
    pub offset: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize, line: usize, column: usize) -> Span {
        Span {
            offset: offset,
            length: length,
            line: line,
            column: column,
        }
    }

    pub fn empty() -> Span {
        Span {
            offset: 0,
            length: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn from_nom_span(span: &NomSpan) -> Span {
        Span {
            offset: span.offset,
            length: span.fragment.len(),
            line: span.line as usize,
            column: span.get_column(), // TODO get_column_utf8 ?
        }
    }

    pub fn from_to(from: Span, to: Span) -> Span {
        Span {
            offset: from.offset,
            length: to.offset - from.offset + to.length,
            line: from.line,
            column: from.column,
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Offset {} Line {} Column {} Lenght {}", self.offset, self.line, self.column, self.length)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Identifier {
    pub span: Span,
    pub name: String,
}

impl_spanned!(Identifier);

impl Identifier {
    pub fn new(name: &str, span: Span) -> Identifier {
        Identifier {
            span: span,
            name: name.to_string(),
        }
    }

    pub fn from_nom_span(span: NomSpan) -> Identifier {
        Identifier {
            span: Span::new(span.offset, span.fragment.len(), span.line as usize, span.get_column() as usize),
            name: span.fragment.to_string(),
        }
    }
}

pub type TypeIdentifier = Identifier;