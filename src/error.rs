use thiserror::Error;
use wled_json_api_library::errors::WledJsonApiError;

#[derive(Error, Debug)]
pub enum WledControllerError {
    #[error("Json Api Error: {0}")]
    JsonApiError(#[from] WledJsonApiError),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error in DDP library")]
    DdpError(#[from] ddp_rs::error::DDPError),
    #[error("Attempted to read a key that doesn't exist \
            (either you need to read it from the server, or the server didn't send one)")]
    MissingKey,
}
