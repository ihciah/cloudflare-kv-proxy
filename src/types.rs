use serde::{de::DeserializeOwned, Deserialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("key not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("bad request")]
    BadRequest,
    #[error("api error code {0}: {1}")]
    Api(u16, String),

    #[error("reqwest error")]
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        match e.status() {
            Some(code) if code == 400 => Self::BadRequest,
            Some(code) if code == 401 => Self::Unauthorized,
            Some(code) if code == 404 => Self::NotFound,
            _ => Self::Reqwest(e),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ApiResult<T>
where
    T: DeserializeOwned,
{
    Ok {
        #[serde(with = "serde_with::json::nested")]
        result: T,
    },
    Err {
        code: u16,
        error: String,
    },
}

impl<T> From<ApiResult<T>> for Result<T>
where
    T: DeserializeOwned,
{
    fn from(r: ApiResult<T>) -> Self {
        match r {
            ApiResult::Ok { result } => Ok(result),
            ApiResult::Err { code, error } => Err(Error::Api(code, error)),
        }
    }
}
