use crate::api::models::db::BadgeData;
use crate::api::models::web::{BadgenBadge, CreateBadge, ShieldsBadge};
use crate::api::provider::get_badge_data_from_database;
use crate::api::routes::helpers::get_badge_errors;
use axum::extract::{Path, Query, State};
use axum::headers::HeaderValue;
use axum::http::header;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::error;
use rsbadges::Style;
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct LocalQueryParams {
    pub style: Option<String>,
}

/// Generate badge locally using rsbadges
#[utoipa::path(
    get,
    path = "/badge/{read_key}/local",
    tag = "Badge",
    responses(
        (status = 200, description = "Get a badge by read key", content_type = "image/svg+xml"),
        (status = 400, description = "Invalid read key"),
        (status = 404, description = "Read key not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("read_key" = String, Path, description = "Hex-encoded read key", example = "0000000000000000"),
        ("style" = Option<String>, Query, description = "Style of the badge", example = "flat"),
    )
)]
pub async fn badge_local(
    Path(read_key): Path<String>,
    Query(query_params): Query<LocalQueryParams>,
    State(pool): State<MySqlPool>,
) -> impl IntoResponse {
    match badge_get_helper(&pool, read_key.as_str()).await {
        Ok(result) => {
            let badge = rsbadges::Badge::from(result);
            match generate_badge(badge, query_params.style.as_deref()) {
                Ok(svg) => svg_response(svg.as_str()).into_response(),
                Err(e) => e.into_response(),
            }
        }
        Err(e) => e.into_response(),
    }
}

/// Preview creating a badge local badge using rsbadges
#[utoipa::path(
    get,
    path = "/badge/preview",
    tag = "Badge",
    responses(
        (status = 200, description = "Get a badge by read key", content_type = "image/svg+xml"),
        (status = 400, description = "Invalid read key"),
        (status = 404, description = "Read key not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("left_label" = String, Query, example = "Lines of code"),
        ("right_label" = String, Query, example = "-100"),
        ("left_color" = String, Query, example = "555"),
        ("right_color" = String, Query, example = "999"),
        ("style" = Option<String>, Query, description = "Style of the badge", example = "flat"),
    )
)]
pub async fn badge_preview(
    Query(input): Query<CreateBadge>,
    Query(params): Query<LocalQueryParams>,
) -> impl IntoResponse {
    if let Some(e) = get_badge_errors(&input) {
        return e.into_response();
    }
    let badge = rsbadges::Badge::from(input);
    match generate_badge(badge, params.style.as_deref()) {
        Ok(svg) => svg_response(svg.as_str()).into_response(),
        Err(e) => e.into_response(),
    }
}

/// Display badge in format of shields.io
#[utoipa::path(
    get,
    path = "/badge/{read_key}/shields",
    tag = "Badge",
    responses(
        (status = 200, description = "Get a badge by read key", body = ShieldsBadge),
        (status = 400, description = "Invalid read key"),
        (status = 404, description = "Read key not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("read_key" = String, Path, description = "Hex-encoded read key", example = "0000000000000000")
    )
)]
pub async fn badge_shields(
    Path(read_key): Path<String>,
    State(pool): State<MySqlPool>,
) -> impl IntoResponse {
    match badge_get_helper(&pool, read_key.as_str()).await {
        Ok(result) => Json(ShieldsBadge::from(result)).into_response(),
        Err(e) => e.into_response(),
    }
}

/// Display badge in format of badgen.net
#[utoipa::path(
    get,
    path = "/badge/{read_key}/badgen",
    tag = "Badge",
    responses(
        (status = 200, description = "Get a badge by read key", body = BadgenBadge),
        (status = 400, description = "Invalid read key"),
        (status = 404, description = "Read key not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("read_key" = String, Path, description = "Hex-encoded read key", example = "0000000000000000")
    )
)]
pub async fn badge_badgen(
    Path(read_key): Path<String>,
    State(pool): State<MySqlPool>,
) -> impl IntoResponse {
    match badge_get_helper(&pool, read_key.as_str()).await {
        Ok(result) => Json(BadgenBadge::from(result)).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn badge_get_helper(
    pool: &MySqlPool,
    read_key: &str,
) -> Result<BadgeData, impl IntoResponse> {
    let read_key = match hex::decode(read_key) {
        Ok(key) => key,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid read_key: {}", read_key),
            )
                .into_response())
        }
    };
    match get_badge_data_from_database(pool, &read_key).await {
        Ok(x) => Ok(x),
        Err(e) => {
            error!("{}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response())
        }
    }
}

pub fn generate_badge(
    badge: rsbadges::Badge,
    style: Option<&str>,
) -> Result<String, impl IntoResponse> {
    let badge_style = match style {
        Some(style) => match style {
            "flat" => Style::Flat(badge),
            "flat-square" => Style::FlatSquare(badge),
            "plastic" => Style::Plastic(badge),
            "for-the-badge" => Style::ForTheBadge(badge),
            "social" => Style::Social(badge),
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Invalid style: must be one of: \
                    flat, flat-square, plastic, for-the-badge, social"
                        .to_string(),
                )
                    .into_response())
            }
        },
        None => Style::Flat(badge),
    };
    match badge_style.generate_svg() {
        Ok(svg) => Ok(svg),
        Err(e) => {
            error!("{}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate SVG").into_response())
        }
    }
}

pub fn svg_response(svg: &str) -> impl IntoResponse {
    let mut response = Response::builder()
        .status(StatusCode::OK)
        .body(svg.to_string())
        .unwrap();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("image/svg+xml"),
    );
    response.into_response()
}
