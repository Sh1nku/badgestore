use crate::api::models::verify_badge::VerifyBadge;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub fn get_badge_errors(s: &impl VerifyBadge) -> Option<Response> {
    s.get_errors()
        .map(|e| (StatusCode::BAD_REQUEST, Json(e)).into_response())
}
