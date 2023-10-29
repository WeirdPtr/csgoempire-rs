pub mod bool;

#[derive(Debug)]
pub enum CSGOEmpireApiRequestError {
    MissingHeader(&'static str),
    MissingParameter(&'static str),
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
    Other(std::io::Error),
    RateLimited,
    InvalidResponse,
}

impl From<reqwest::Error> for CSGOEmpireApiRequestError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}

impl From<std::io::Error> for CSGOEmpireApiRequestError {
    fn from(error: std::io::Error) -> Self {
        Self::Other(error)
    }
}

impl From<serde_json::Error> for CSGOEmpireApiRequestError {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeError(error)
    }
}