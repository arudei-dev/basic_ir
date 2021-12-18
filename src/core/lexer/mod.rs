pub mod errors;
pub mod pos;

use super::constants::regexes;
use super::token;

pub struct Lexer {
    text: String,
    current_char: Option<char>,
    pos: pos::Position,
}

impl Lexer {
    pub fn init(text: String) -> Self {
        let mut new_self = Self {
            text,
            pos: pos::Position::init(0, 0, 0),
            current_char: None,
        };

        new_self.current_char = new_self.text.chars().nth(new_self.pos.idx);

        return new_self;
    }

    pub fn advance(&mut self) {
        if let Some(c) = self.current_char {
            self.pos.advance(c);
        }
        self.current_char = self.text.chars().nth(self.pos.idx);
    }

    pub fn tokenize(&mut self) -> Result<Vec<token::Token>, errors::Errors> {
        use token::{symbols::Symbols, Token};

        let mut tokens: Vec<Token> = vec![];

        while let Some(curr_char) = self.current_char {
            let mut stay = false;

            let tok = match curr_char {
                '+' => Some(Token::Symbols(Symbols::Plus)),
                '-' => Some(Token::Symbols(Symbols::Minus)),
                '*' => Some(Token::Symbols(Symbols::Multiply)),
                '/' => Some(Token::Symbols(Symbols::Division)),
                '(' => Some(Token::Symbols(Symbols::LeftParenthesis)),
                ')' => Some(Token::Symbols(Symbols::RightParenthesis)),
                t if regexes::DIGITS_DOT.is_match(t.to_string().as_str()) => {
                    stay = true;
                    self.make_numbers()
                }
                t if regexes::SPACES_TABS.is_match(t.to_string().as_str()) => None,
                t => {
                    let pos_start = self.pos.clone();
                    self.advance();
                    return Err(errors::Errors::IllegalChar(errors::Error::new(
                        format!("Unknown token: {}", t),
                        pos_start,
                        self.pos.clone(),
                        format!("<stdin>"),
                        self.text.clone(),
                    )));
                }
            };

            if let Some(t) = tok {
                tokens.push(t);
            }

            if stay {
                continue;
            }

            self.advance();
        }

        return Ok(tokens);
    }

    fn make_numbers(&mut self) -> Option<token::Token> {
        let mut num_str = String::new();
        let mut dotted = false;

        while let Some(curr_char) = self.current_char {
            if !regexes::DIGITS_DOT.is_match(curr_char.to_string().as_str()) {
                break;
            }

            if dotted && curr_char == '.' {
                self.advance();
                continue;
            }

            if curr_char == '.' {
                dotted = true;
                num_str += ".";
            } else {
                num_str += format!("{}", curr_char).as_str();
            }

            self.advance();
        }

        if !dotted {
            return Some(token::Token::Int(num_str.parse::<i32>().unwrap()));
        } else {
            return Some(token::Token::Float(num_str.parse::<f64>().unwrap()));
        }
    }
}
