use crate::api::password::generate_password;
use dotenvy::dotenv;
use sqlx::{Acquire, MySqlPool};
use std::net::SocketAddr;
use std::{env, error};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(Clone)]
pub struct Config {
    pub db_url: String,
    pub server_host: SocketAddr,
}

pub fn setup_from_env() -> Result<Config, Box<dyn error::Error>> {
    dotenv().ok();
    Ok(Config {
        db_url: env::var("DATABASE_URL")?,
        server_host: match env::var("BADGE_SERVER") {
            Ok(value) => value.parse()?,
            Err(_) => "0.0.0.0:8080".parse()?,
        },
    })
}

pub async fn seed(pool: &MySqlPool) -> Result<(), Box<dyn error::Error>> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS badge (
            read_key BINARY(8) UNIQUE NOT NULL,
            hash VARBINARY(128) NOT NULL,
            created_date DATETIME NOT NULL DEFAULT NOW(),
            last_modified DATETIME NOT NULL DEFAULT NOW(),
            last_accessed DATETIME NOT NULL DEFAULT NOW(),
            PRIMARY KEY (read_key)
        );
    "#,
    )
    .execute(pool)
    .await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS badge_data (
            left_label VARBINARY(96) NOT NULL,
            left_color VARBINARY(6) NOT NULL,
            right_label VARBINARY(96) NOT NULL,
            right_color VARBINARY(6) NOT NULL,
            read_key BINARY(8) UNIQUE NOT NULL,
            FOREIGN KEY (read_key) REFERENCES badge(read_key)
        );
    "#,
    )
    .execute(pool)
    .await?;

    let mut connection = pool.acquire().await?;
    let mut tx = connection.begin().await?;
    let password = generate_password()?;
    sqlx::query(
        r#"
        INSERT IGNORE INTO badge (read_key, hash, created_date, last_modified)
        VALUES (
                0x0, ?, NOW(), NOW()
               );
    "#,
    )
    .bind(password.hash)
    .execute(&mut tx)
    .await?;
    sqlx::query(
        r#"
        INSERT IGNORE INTO badge_data (left_label, left_color, right_label, right_color, read_key)
        VALUES (
                'Lines of code', '555', '-100', '010101', 0x0
               );
    "#,
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::routes::badge_get::badge_shields,
        crate::api::routes::badge_get::badge_badgen,
        crate::api::routes::badge_get::badge_local,
        crate::api::routes::badge_get::badge_preview,
        crate::api::routes::badge_post::create_badge,
        crate::api::routes::badge_put::update_badge
    ),
    components(
        schemas(
            crate::api::models::web::ShieldsBadge,
            crate::api::models::web::BadgenBadge,
            crate::api::models::web::KeyResult,
            crate::api::models::web::CreateBadge,
            crate::api::models::web::UpdateBadge,
            crate::api::models::db::BadgeData
        )
    ),
    tags(
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;
struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "basic",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
            )
        }
    }
}
