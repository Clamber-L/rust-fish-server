use crate::error::AppError;
use crate::jwt::verification_jwt;
use axum::{extract::Request, middleware::Next, response::Response};

pub async fn verification_header(mut request: Request, next: Next) -> Result<Response, AppError> {
    // check request header token
    let token = request
        .headers()
        .get("token")
        .and_then(|token| token.to_str().ok());

    if let Some(token) = token {
        let jwt_user = verification_jwt(token);
        match jwt_user {
            Ok(user) => {
                request.extensions_mut().insert(user.id);
                Ok(next.run(request).await)
            }
            Err(_) => Err(AppError::Unauthorized),
        }
    } else {
        Err(AppError::Unauthorized)
    }
}
