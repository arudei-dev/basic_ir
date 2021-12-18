pub mod symbols;

pub enum Token {
    Atom,
    Int(i32),
    Long(i64),
    Float(f64),
    Symbols(symbols::Symbols),
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self {
            &Self::Atom => ("Atom", None),
            &Self::Int(v) => ("Int", Some(format!("{}", v))),
            &Self::Float(f) => ("Float", Some(format!("{}", f))),
            &Self::Symbols(s) => ("Symbol", Some(format!("{:?}", s))),
            _ => ("", None),
        };

        write!(
            f,
            "[{}{}]",
            output.0,
            match output.1 {
                Some(s) => format!(":{}", s),
                None => format!(""),
            }
        )
    }
}
