/// The Errors that may occur when processing a Request.
#[derive(Debug, Fail)]
pub enum PixelaClientError {
    #[fail(display = "request failed: {}", _0)]
    HttpClientError(reqwest::Error),
    #[fail(display = "failed parse json: {}", _0)]
    ResponseParseFailed(serde_json::Error),
    #[fail(display = "request is not success: {}", _0)]
    RequestNotSuccess(String),
}

impl From<reqwest::Error> for PixelaClientError {
    fn from(e: reqwest::Error) -> Self {
        PixelaClientError::HttpClientError(e)
    }
}

impl From<serde_json::Error> for PixelaClientError {
    fn from(e: serde_json::Error) -> Self {
        PixelaClientError::ResponseParseFailed(e)
    }
}
