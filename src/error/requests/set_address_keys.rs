use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::error::lib::LibError;

#[derive(PartialEq, Debug, Clone)]
pub enum SetAddressKeysError {
    CannotSendRequest { url: String },
    ResponseStatusIsNotSuccessful { url: String, status: u16 },
    CannotGetTextFromTheResponse { url: String },
    FailedToParseTheResponse { url: String, response: String },
    ResponseCodeIsNotSuccessful { url: String, code: String },
    CannotConvertKeysAsJSON { url: String, keys: HashMap<String, String> }
}

impl Display for SetAddressKeysError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SetAddressKeysError::CannotSendRequest { url } => {
                write!(f, "Cannot send the set address keys request. Url: {url}")
            },
            SetAddressKeysError::ResponseStatusIsNotSuccessful { url, status } => {
                write!(f, "Set address keys response's status is not successful: {status}, url: {url}")
            },
            SetAddressKeysError::CannotGetTextFromTheResponse  { url } => {
                write!(f, "No text received in the set address keys response. Url: {url}")
            },
            SetAddressKeysError::FailedToParseTheResponse { url, response } => {
                write!(f, "Cannot parse the received set address keys response: {response}, url: {url}")
            },
            SetAddressKeysError::ResponseCodeIsNotSuccessful { url, code } => {
                write!(f, "Set address keys response's code is not successful: {code}, url: {url}")
            },
            SetAddressKeysError::CannotConvertKeysAsJSON { url, keys } => {
                write!(f, "Error while creating the body for POST {url}, keys: {keys:?}")
            },
        }
    }
}

impl From<SetAddressKeysError> for LibError {
    fn from(value: SetAddressKeysError) -> Self {
        LibError::SetAddressKeys(value)
    }
}