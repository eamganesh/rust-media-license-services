use std::{collections::HashSet, sync::Arc};

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use super::AppState;

pub const JWT_PASS_CODE: &'static str = "12345";
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtBodyData {
    pub baseUrl: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaim {
    pub sub: String,
    pub iss: String,
    pub data: JwtBodyData,
    pub iat: i64,
}

pub async fn authenticate<B>(
    State(app_data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });
    let token = token.ok_or_else(|| {
        let json_error: ErrorResponse = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;
    let mut validation_options = Validation::new(Algorithm::HS256);
    validation_options.validate_exp = false;
    validation_options.required_spec_claims = HashSet::with_capacity(0);
    let claim: JwtClaim =
        decode::<JwtClaim>(&token, &app_data.jwt_decoding_key, &validation_options)
            .map_err(|_| {
                let json_error: ErrorResponse = ErrorResponse {
                    status: "fail",
                    message: "Invalid token".to_string(),
                };
                (StatusCode::UNAUTHORIZED, Json(json_error))
            })?
            .claims;
    req.extensions_mut().insert(claim);
    Ok(next.run(req).await)
}
