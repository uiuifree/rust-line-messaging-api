use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub enum LineError {
    ApiError(LineApiError),
    HttpError(LineHttpError),
    SystemError(LineSystemError),
}

impl LineError {
    pub fn status(&self) -> u16 {
        match self {
            LineError::ApiError(e) => e.status,
            LineError::HttpError(e) => e.status,
            LineError::SystemError(_) => 0,
        }
    }
    pub fn api_error(&self) -> Option<&LineApiErrorResponse> {
        match self {
            LineError::ApiError(e) => Some(&e.error),
            LineError::HttpError(_) => None,
            LineError::SystemError(_) => None,
        }
    }
}

impl From<LineApiError> for LineError {
    fn from(value: LineApiError) -> Self {
        LineError::ApiError(value)
    }
}

impl From<LineHttpError> for LineError {
    fn from(value: LineHttpError) -> Self {
        LineError::HttpError(value)
    }
}

impl From<LineSystemError> for LineError {
    fn from(value: LineSystemError) -> Self {
        LineError::SystemError(value)
    }
}

#[derive(Debug)]
pub struct LineApiError {
    pub status: u16,
    pub error: LineApiErrorResponse,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

/// https://developers.line.biz/ja/reference/messaging-api/#error-responses
#[derive(Debug, Serialize, Deserialize)]
pub struct LineApiErrorResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<LineApiErrorResponseDetail>>,
}

/// https://developers.line.biz/ja/reference/messaging-api/#error-responses
#[derive(Debug, Serialize, Deserialize)]
pub struct LineApiErrorResponseDetail {
    pub message: String,
    pub property: String,
}

impl LineApiErrorResponse {
    pub fn debug_print(&self) {
        println!("{}", self.message);
        if let Some(details) = &self.details {
            for detail in details {
                println!(" {}: {}", detail.property, detail.message);
            }
        }
    }
}

#[derive(Debug)]
pub struct LineHttpError {
    pub status: u16,
    pub http_response_body: Option<String>,
}

impl LineHttpError {
    pub fn new(status: u16, http_response_body: String) -> LineHttpError {
        LineHttpError {
            status,
            http_response_body: Some(http_response_body),
        }
    }
}

#[derive(Debug)]
pub struct LineSystemError {
    pub message: Option<String>,
}

impl LineSystemError {
    pub fn new(message: String) -> LineSystemError {
        LineSystemError {
            message: Some(message),
        }
    }
}
