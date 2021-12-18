pub enum Symbols {
    Plus,
    Minus,
    Multiply,
    Division,
    LeftParenthesis,
    RightParenthesis,
    QuoteSingle,
    QuoteDouble,
}

impl std::fmt::Debug for Symbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sym = match self {
            &Self::Plus => "+",
            &Self::Minus => "-",
            &Self::Multiply => "*",
            &Self::Division => "/",
            &Self::LeftParenthesis => "(",
            &Self::RightParenthesis => ")",
            &Self::QuoteSingle => "'",
            &Self::QuoteDouble => "\"",
        };

        write!(f, "{}", sym)
    }
}
