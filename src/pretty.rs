use crate::term::*;
use std::fmt;

/// Pretty prints a term.
/// Important, note function def, should return string, not print it
pub fn pretty_print(term: &Term) -> String {
    match term {
        Term::Var(s) => s.clone(),
        Term::Abs(s, newterm) => format!("λ{}.{}", s, pretty_print(newterm)),
        Term::App(term1, term2) => format!("({} {})", pretty_print(term1), pretty_print(term2)),
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}