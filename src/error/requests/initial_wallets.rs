use std::fmt::{Display, Formatter};
use crate::error::lib::LibError;

#[derive(PartialEq, Debug, Clone)]
pub enum InitialWalletsError {
    CannotSendRequest { url: String },
    ResponseStatusIsNotSuccessful { url: String, status: u16 },
    CannotGetTextFromTheResponse { url: String },
    FailedToParseTheResponse { url: String, response: String },
    ResponseCodeIsNotSuccessful { url: String, code: String },
}

impl Display for InitialWalletsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InitialWalletsError::CannotSendRequest { url } => {
                write!(f, "Cannot send the initial wallets request. Url: {url}")
            },
            InitialWalletsError::ResponseStatusIsNotSuccessful { url, status } => {
                write!(f, "Initial wallets response's status is not successful: {status}, url: {url}")
            },
            InitialWalletsError::CannotGetTextFromTheResponse  { url } => {
                write!(f, "No text received in the initial wallets response. Url: {url}")
            },
            InitialWalletsError::FailedToParseTheResponse { url, response } => {
                write!(f, "Cannot parse the received initial wallets response: {response}, url: {url}")
            },
            InitialWalletsError::ResponseCodeIsNotSuccessful { url, code } => {
                write!(f, "Initial wallets response's code is not successful: {code}, url: {url}")
            },
        }
    }
}

impl From<InitialWalletsError> for LibError {
    fn from(value: InitialWalletsError) -> Self {
        LibError::InitialWallets(value)
    }
}