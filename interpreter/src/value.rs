use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Text(String),
    Array(Vec<Value>),
    Range {
        start: i64,
        end: i64,
        inclusive: bool,
    },
    Nil,
}

impl Value {
    pub fn as_number(&self) -> Option<f64> {
        if let Self::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::Number(n) => *n != 0.0,
            Self::Text(s) => !s.is_empty(),
            Self::Array(items) => !items.is_empty(),
            Self::Range {
                start,
                end,
                inclusive,
            } => {
                if *inclusive {
                    start <= end
                } else {
                    start < end
                }
            }
            Self::Nil => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Text(s) => write!(f, "{s}"),
            Self::Array(items) => {
                write!(f, "[")?;
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Self::Range {
                start,
                end,
                inclusive,
            } => {
                if *inclusive {
                    write!(f, "{start}..={end}")
                } else {
                    write!(f, "{start}..{end}")
                }
            }
            Self::Nil => write!(f, "nil"),
        }
    }
}
