use crate::response::ApiResponse;
use serde::Serialize;
use tauri::http::StatusCode;
use tauri::ipc::InvokeResponseBody::Json;
use tauri::ipc::{InvokeResponseBody, IpcResponse, Response};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("Database Error:{0}")]
    DataBase(#[from] sea_orm::DbErr),
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
    #[error("0")]
    Biz(String),
    #[error("{0}")]
    Init(#[from] rusqlite::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Self::DataBase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Biz(_) => StatusCode::OK,
            Self::Init(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IpcResponse for ApiError {
    fn body(self) -> tauri::Result<InvokeResponseBody> {
        let status_code = self.status_code();
        let body = Json(self.to_string());
        Ok(body)
    }
}
