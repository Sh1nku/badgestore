extern crate core;

use axum::{routing, Router, Server};
use badgestore::api::config::{seed, setup_from_env, ApiDoc};
use badgestore::api::routes::badge_get::{badge_badgen, badge_local, badge_preview, badge_shields};
use badgestore::api::routes::badge_post::create_badge;
use badgestore::api::routes::badge_put::update_badge;
use log::{error, info};
use sqlx::MySqlPool;
use std::process::exit;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
pub async fn main() {
    env_logger::init();

    let config = match setup_from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("Error while parsing .env:\n{}", e);
            exit(1);
        }
    };
    info!("Trying to connect to mysql server");
    let pool = MySqlPool::connect(config.db_url.as_str()).await.unwrap();
    seed(&pool).await.unwrap();
    info!("Connection to mysql server succeeded");

    let app = Router::new()
        .merge(SwaggerUi::new("/").url("/openapi.json", ApiDoc::openapi()))
        .route("/badge/:read_key/shields", routing::get(badge_shields))
        .route("/badge/:read_key/badgen", routing::get(badge_badgen))
        .route("/badge/:read_key/local", routing::get(badge_local))
        .route("/badge/preview", routing::get(badge_preview))
        .route("/badge", routing::post(create_badge))
        .route("/badge", routing::put(update_badge))
        .layer(CorsLayer::permissive())
        .with_state(pool)
        .with_state(config.clone());

    info!("Starting server on {}", &config.server_host);
    Server::bind(&config.server_host)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
