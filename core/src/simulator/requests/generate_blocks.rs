use serde::Deserialize;

#[derive(Deserialize)]
pub struct GenerateBlocksResponseEmpty {}

#[derive(Deserialize)]
pub struct GenerateBlocksResponse {
    pub data: GenerateBlocksResponseEmpty,
    pub error: String,
    pub code: String
}