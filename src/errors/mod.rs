
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum CommandError {
    InvalidCommand,
    CmdErr(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::InvalidCommand => write!(f, "Invalid command"),
            CommandError::CmdErr(s) => write!(f, "{} Invalid command.", s),
        }
    }
}

impl Error for CommandError {}



#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseCommandError: {}", self.details)
    }
}

impl Error for ParseError {}
