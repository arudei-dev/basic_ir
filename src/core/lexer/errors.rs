use super::pos;

pub struct Error {
    file_name: String,
    file_text: String,
    err_details: String,
    pos_start: pos::Position,
    pos_end: pos::Position,
}

impl Error {
    pub fn new(
        details: String,
        start: pos::Position,
        end: pos::Position,
        file_name: String,
        file_text: String,
    ) -> Self {
        Self {
            err_details: details,
            pos_start: start,
            pos_end: end,
            file_name,
            file_text,
        }
    }
}

pub enum Errors {
    IllegalChar(Error),
}

impl std::fmt::Debug for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (err_ty, err_obj) = match &self {
            Self::IllegalChar(d) => ("IllegalCharacter", d),
        };

        let Error {
            err_details,
            file_name,
            pos_start,
            ..
        } = err_obj;

        let ln_1 = format!("<SyntaxError::{}> {}", err_ty, err_details);
        let ln_2 = format!(
            "File {}, line {} col {}",
            file_name, pos_start.line, pos_start.col
        );
        write!(f, "{}\n{}", ln_1, ln_2)
    }
}
