use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OllamaChatResponse {
    Success(OllamaChatSuccessResponse),
    Error { error: String },
}

#[derive(Deserialize, Debug)]
pub struct OllamaChatSuccessResponse {
    pub message: OllamaChatMessage,
}

#[derive(Deserialize, Debug)]
pub struct OllamaChatMessage {
    pub content: String,
}
