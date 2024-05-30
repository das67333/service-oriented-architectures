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
    UserNotFound,
    InvalidCategory,
}

impl AppError {
    fn get_code_and_message(&self) -> (StatusCode, &'static str) {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occured",
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "Missing credential"),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token"),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "User does not exist"),
            Self::UserAlreadyExits => (StatusCode::BAD_REQUEST, "User already exists"),
            Self::PostNotFound => (StatusCode::NOT_FOUND, "Post not found"),
            Self::AccessDenied => (StatusCode::FORBIDDEN, "Access denied"),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            Self::InvalidCategory => (StatusCode::BAD_REQUEST, "Invalid category"),
        };
        (status, err_msg)
    }
}

pub fn internal_server_error(err: impl std::fmt::Debug) -> AppError {
    tracing::error!("Error: {:?}", err);
    AppError::InternalServerError
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        self.get_code_and_message().into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apperror() {
        for err in [
            AppError::InternalServerError,
            AppError::InvalidToken,
            AppError::MissingCredential,
            AppError::WrongCredential,
            AppError::TokenCreation,
            AppError::UserDoesNotExist,
            AppError::UserAlreadyExits,
            AppError::PostNotFound,
            AppError::AccessDenied,
            AppError::UserNotFound,
            AppError::InvalidCategory,
        ] {
            let (code, msg) = err.get_code_and_message();
            assert_ne!(code, StatusCode::OK);
            assert_ne!(msg, "");
        }
    }
}
