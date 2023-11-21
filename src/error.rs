use std::any::Any;
use std::error::Error;
use std::num::ParseIntError;
use thiserror::Error;
use wled_json_api_library::errors::WledJsonApiError;

#[derive(Error, Debug)]
pub enum WledControllerError {
    #[error("Json Api Error: {0}")]
    JsonApiError(#[from] WledJsonApiError),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("error parsing integer from a string: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Error in DDP library: {0}")]
    DdpError(#[from] ddp_rs::error::DDPError),
    #[error("Error parsing url")]
    UrlParseError,
    #[error("std IO Error: {0}")]
    StdIoError(#[from] std::io::Error),
    #[error("Attempted to read a key that doesn't exist \
            (either you need to read it from the server, or the server didn't send one)")]
    MissingKey,
    #[error("Attempted to convert a string to an IP but failed to convert vec to array. \
        This is likely because WLED has provided an IPv6 or because the WLED is in AP mode \
        (neither has support yet)")]
    BadIp,
    #[error("Used as a placeholder in development, you should never see this")]
    TempError,
    #[error("Wled returned a capability that I dont like: {0}
        the u8 is a bitmap with masks defined in ```SegmentLightCapability``` and currently \
        the only error value is a light with no rgb or w support")]
    InvalidCapability(u8),
}