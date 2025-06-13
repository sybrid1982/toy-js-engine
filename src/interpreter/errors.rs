use std::fmt::Display;

pub enum InterpreterErrorKind {
    ReferenceError(String),
    SyntaxError(Option<SyntaxErrorKind>),
    NaN
}

pub enum SyntaxErrorKind {
    LeftSideAssignmentMustBeIdentifier,
    InvalidLeftSidePrefix,
}

impl SyntaxErrorKind {
    pub fn to_string(&self) -> String {
        match self {
            Self::LeftSideAssignmentMustBeIdentifier => {
                "Left side of assignment must be identifier".to_string()
            }
            Self::InvalidLeftSidePrefix => {
                "Invalid left-hand side expression in prefix operation".to_string()
            }
        }
    }
}

impl Display for SyntaxErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct InterpreterError {
    pub kind: InterpreterErrorKind
}

impl InterpreterError {
    pub fn to_string(&self) -> String {
        match &self.kind {
            InterpreterErrorKind::ReferenceError(identifier) => {
                format!("Uncaught ReferenceError: {} is not defined", identifier).to_string()
            },
            InterpreterErrorKind::SyntaxError(message) => {
                match message {
                    Some(error_text) => format!("Uncaught SyntaxError: {}", error_text),
                    None => "Uncaught SyntaxError".to_string(),
                }
            },
            InterpreterErrorKind::NaN => {
                "NaN".to_string()
            },
        }
    }
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
