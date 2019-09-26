use std::fmt;
use std::mem;

#[derive(Copy, Clone, Eq, Hash, Debug)]
pub enum Color {
    White,
    Black
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}