use crate::api::models::web::CreateBadge;
use crate::api::provider::create_badge_in_database;
use crate::api::routes::helpers::get_badge_errors;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use log::error;
use sqlx::MySqlPool;

/// Create a badge
#[utoipa::path(
    post,
    path = "/badge",
    request_body = CreateBadge,
    tag = "Badge",
    responses(
        (status = 200, description = "Created successfully", body = KeyResult),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_badge(
    State(pool): State<MySqlPool>,
    Json(input): Json<CreateBadge>,
) -> impl IntoResponse {
    if let Some(e) = get_badge_errors(&input) {
        return e;
    }
    match create_badge_in_database(&pool, input).await {
        Ok(x) => (StatusCode::CREATED, Json(x)).into_response(),
        Err(e) => {
            error!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
        }
    }
}
