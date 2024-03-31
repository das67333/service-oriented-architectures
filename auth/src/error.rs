use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    InvalidToken,
    MissingCredential,
    WrongCredential,
    TokenCreation,
    UserDoesNotExist,
    UserAlreadyExits,
    PostNotFound,
    AccessDenied,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "Missing credential"),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token"),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "User does not exist"),
            Self::UserAlreadyExits => (StatusCode::BAD_REQUEST, "User already exists"),
            Self::PostNotFound => (StatusCode::NOT_FOUND, "Post not found"),
            Self::AccessDenied => (StatusCode::FORBIDDEN, "Access denied"),
        };
        (status, err_msg).into_response()
    }
}
