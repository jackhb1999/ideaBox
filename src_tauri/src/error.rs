

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
    Io(#[from] std::io::Error)
}


impl ApiError {
}

