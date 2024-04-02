use std::fmt::{Display, Formatter};
use crate::error::lib::LibError;
use crate::simulator::requests::set_state::SetStateAddress;

#[derive(PartialEq, Debug, Clone)]
pub enum SetStateError {
    CannotSendRequest { url: String },
    ResponseStatusIsNotSuccessful { url: String, status: u16 },
    CannotGetTextFromTheResponse { url: String },
    FailedToParseTheResponse { url: String, response: String },
    ResponseCodeIsNotSuccessful { url: String, code: String },
    CannotConvertStateAsJSON { url: String, state: Vec<SetStateAddress> }
}

impl Display for SetStateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SetStateError::CannotSendRequest { url } => {
                write!(f, "Cannot send the set state request. Url: {url}")
            },
            SetStateError::ResponseStatusIsNotSuccessful { url, status } => {
                write!(f, "Set state response's status is not successful: {status}, url: {url}")
            },
            SetStateError::CannotGetTextFromTheResponse  { url } => {
                write!(f, "No text received in the set state response. Url: {url}")
            },
            SetStateError::FailedToParseTheResponse { url, response } => {
                write!(f, "Cannot parse the received set state response: {response}, url: {url}")
            },
            SetStateError::ResponseCodeIsNotSuccessful { url, code } => {
                write!(f, "Set state response's code is not successful: {code}, url: {url}")
            },
            SetStateError::CannotConvertStateAsJSON { url, state } => {
                write!(f, "Error while creating the set state body for POST {url}, keys: {state:?}")
            },
        }
    }
}

impl From<SetStateError> for LibError {
    fn from(value: SetStateError) -> Self {
        LibError::SetState(value)
    }
}