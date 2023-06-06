use crate::api::models::web::{KeyResult, UpdateBadge};
use crate::api::provider::{check_write_key, update_badge_in_database};
use crate::api::routes::helpers::get_badge_errors;
use axum::extract::State;
use axum::headers::authorization::Basic;
use axum::headers::Authorization;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, TypedHeader,
};
use log::error;
use sqlx::MySqlPool;

/// Update badge data (Note: Requires Basic Auth with format "read_key:write_key")
#[utoipa::path(
    put,
    path = "/badge",
    request_body = UpdateBadge,
    tag = "Badge",
    responses(
        (status = 200, description = "Updated sucessfully", body = BadgeData),
        (status = 400, description = "Bad request"),
        (status = 403, description = "Invalid write key"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("basic" = [])
    )
)]
pub async fn update_badge(
    State(pool): State<MySqlPool>,
    TypedHeader(auth): TypedHeader<Authorization<Basic>>,
    Json(input): Json<UpdateBadge>,
) -> impl IntoResponse {
    if let Some(e) = get_badge_errors(&input) {
        return e;
    }
    let auth = match parse_auth(TypedHeader(auth)) {
        Ok(auth) => auth,
        Err(err) => return err,
    };
    match check_write_key(&pool, &auth.read_key, &auth.write_key).await {
        Ok(valid) => match valid {
            true => match update_badge_in_database(&pool, &auth.read_key, input).await {
                Ok(x) => Json(x).into_response(),
                Err(e) => {
                    error!("{}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
                }
            },
            false => (StatusCode::FORBIDDEN, "Key does not match value").into_response(),
        },
        Err(value) => {
            match value {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Read key not found"),
                e => {
                    error!("{}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "")
                }
            }
        }
        .into_response(),
    }
}

pub fn parse_auth(
    TypedHeader(auth): TypedHeader<Authorization<Basic>>,
) -> Result<KeyResult, Response> {
    let read_key = match hex::decode(auth.username()) {
        Ok(key) => key,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid read_key: {}", auth.username()),
            )
                .into_response())
        }
    };
    let write_key = match hex::decode(auth.password()) {
        Ok(key) => key,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid write_key").into_response()),
    };
    Ok(KeyResult {
        read_key,
        write_key,
    })
}
