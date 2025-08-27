use std::fmt::{Debug, Display};

use crate::lexer::Token;

pub enum InterpreterErrorKind {
    ReferenceError(String),
    SyntaxError(Option<SyntaxErrorKind>),
    NaN,
    DivisionByZero
}

#[derive(PartialEq)]
pub enum ParserErrorKind {
    SyntaxError(Option<SyntaxErrorKind>)
}

#[derive(PartialEq)]
pub enum SyntaxErrorKind {
    LeftSideAssignmentMustBeIdentifier,
    InvalidLeftSidePrefix,
    UnexpectedToken(Token),
    UnexpectedIdentifier(String)
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
            Self::UnexpectedToken(token) => {
                format!("Unexpected token '{:#?}'", token)
            }
            Self::UnexpectedIdentifier(identifier) => {
                format!("Unexpected identifier '{}'", identifier)
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
            InterpreterErrorKind::DivisionByZero => {
                "Infinity".to_string()
            }
        }
    }
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq)]
pub struct ParserError {
    pub kind: ParserErrorKind
}

impl ParserError {
    pub fn to_string(&self) -> String {
        match &self.kind {
            ParserErrorKind::SyntaxError(message) => {
                match message {
                    Some(error_text) => format!("Uncaught SyntaxError: {}", error_text),
                    None => "Uncaught SyntaxError".to_string(),
                }
            },
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}