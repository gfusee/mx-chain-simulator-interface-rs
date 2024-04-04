use serde::Deserialize;

#[derive(Deserialize)]
pub struct SetAddressKeysEmpty {}

#[derive(Deserialize)]
pub struct SetAddressKeysResponse {
    pub data: SetAddressKeysEmpty,
    pub error: String,
    pub code: String
}