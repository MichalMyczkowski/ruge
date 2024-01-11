//! Error type definitions and Result<T, GameError> wrapper
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    UnNamedError(String),
    /// First String is for scene name, second for error message
    SceneError(String, String),
    GameLogicError(String),
    BackendError(String),
    EngineError(String),
    Error(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GameError::UnNamedError(ref s) => write!(f, "UnNamed GameError: {}", s),
            GameError::SceneError(ref name, ref s) => write!(f, "Scene '{name}' GameError: {s}"),
            GameError::GameLogicError(ref s) => write!(f, "GameLogic GameError: {}", s),
            GameError::BackendError(ref s) => write!(f, "Backend GameError: {}", s),
            GameError::EngineError(ref s) => {
                write!(f, "MicroEngine Error: {}\ni'm sorry :( ~dev", s)
            }
            GameError::Error(ref s) => {
                write!(f, "Error: {}", s)
            }
        }
    }
}

impl Error for GameError {}

pub type GameResult<T = ()> = Result<T, GameError>;
