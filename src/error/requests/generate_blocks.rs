use std::fmt::{Display, Formatter};
use crate::error::lib::LibError;

#[derive(PartialEq, Debug, Clone)]
pub enum GenerateBlocksError {
    CannotSendRequest { url: String },
    ResponseStatusIsNotSuccessful { url: String, status: u16 },
    CannotGetTextFromTheResponse { url: String },
    FailedToParseTheResponse { url: String, response: String },
    ResponseCodeIsNotSuccessful { url: String, code: String },
}

impl Display for GenerateBlocksError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateBlocksError::CannotSendRequest { url } => {
                write!(f, "Cannot send the generate blocks request. Url: {url}")
            },
            GenerateBlocksError::ResponseStatusIsNotSuccessful { url, status } => {
                write!(f, "Generate blocks response's status is not successful: {status}, url: {url}")
            },
            GenerateBlocksError::CannotGetTextFromTheResponse  { url } => {
                write!(f, "No text received in the generate blocks response. Url: {url}")
            },
            GenerateBlocksError::FailedToParseTheResponse { url, response } => {
                write!(f, "Cannot parse the received generate blocks response: {response}, url: {url}")
            },
            GenerateBlocksError::ResponseCodeIsNotSuccessful { url, code } => {
                write!(f, "Generate blocks response's code is not successful: {code}, url: {url}")
            },
        }
    }
}

impl From<GenerateBlocksError> for LibError {
    fn from(value: GenerateBlocksError) -> Self {
        LibError::GenerateBlocks(value)
    }
}