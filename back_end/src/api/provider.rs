use crate::api::models::db::{Badge, BadgeData, BadgeDataRaw};
use crate::api::models::web::{CreateBadge, KeyResult, UpdateBadge};
use crate::api::password::{generate_password, verify_password};
use sqlx::{Acquire, MySqlPool};
use std::error;

pub async fn update_last_accessed(pool: &MySqlPool, read_key: &Vec<u8>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE badge
        SET last_accessed = NOW()
        WHERE read_key = ?;
    "#,
    )
    .bind(read_key)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_badge_data_from_database(
    pool: &MySqlPool,
    read_key: &Vec<u8>,
) -> Result<BadgeData, Box<dyn error::Error>> {
    update_last_accessed(pool, read_key).await?;
    Ok(
        sqlx::query_as::<_, BadgeDataRaw>("SELECT * FROM badge_data WHERE read_key = ?")
            .bind(read_key)
            .fetch_one(pool)
            .await?
            .try_into()?,
    )
}

pub async fn create_badge_in_database(
    pool: &MySqlPool,
    badge: CreateBadge,
) -> Result<KeyResult, Box<dyn error::Error>> {
    let password = generate_password()?;

    let mut connection = pool.acquire().await?;
    let mut tx = connection.begin().await?;

    sqlx::query(
        r#"
            INSERT INTO badge (read_key, hash)
            VALUES (?, ?);
        "#,
    )
    .bind(&password.read_key)
    .bind(&password.hash)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        r#"
            INSERT INTO badge_data (left_label, left_color, right_label, right_color, read_key)
            VALUES (?, ?, ?, ?, ?);
    "#,
    )
    .bind(badge.left_label)
    .bind(badge.left_color.trim_start_matches('#').to_string())
    .bind(badge.right_label)
    .bind(badge.right_color.trim_start_matches('#').to_string())
    .bind(&password.read_key)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(KeyResult {
        read_key: password.read_key,
        write_key: password.write_key,
    })
}

pub async fn update_badge_in_database(
    pool: &MySqlPool,
    read_key: &Vec<u8>,
    badge: UpdateBadge,
) -> Result<BadgeData, Box<dyn error::Error>> {
    let mut db_badge = get_badge_data_from_database(pool, read_key).await?;
    {
        let mut connection = pool.acquire().await?;
        let mut tx = connection.begin().await?;

        db_badge.left_label = badge.left_label.unwrap_or(db_badge.left_label);
        db_badge.left_color = badge
            .left_color
            .unwrap_or(db_badge.left_color)
            .trim_start_matches('#')
            .to_string();
        db_badge.right_label = badge.right_label.unwrap_or(db_badge.right_label);
        db_badge.right_color = badge
            .right_color
            .unwrap_or(db_badge.right_color)
            .trim_start_matches('#')
            .to_string();

        sqlx::query(
            r#"
            UPDATE badge_data SET
            left_label = ?,
            left_color = ?,
            right_label = ?,
            right_color = ?
            WHERE read_key = ?;
    "#,
        )
        .bind(db_badge.left_label)
        .bind(db_badge.left_color)
        .bind(db_badge.right_label)
        .bind(db_badge.right_color)
        .bind(read_key)
        .execute(&mut *tx)
        .await?;
        sqlx::query(
            r#"
            UPDATE badge SET
            last_modified = NOW()
            WHERE read_key = ?;"#,
        )
        .bind(read_key)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
    }
    get_badge_data_from_database(pool, read_key).await
}

pub async fn get_badge_from_database(
    pool: &MySqlPool,
    read_key: &Vec<u8>,
) -> Result<Badge, sqlx::Error> {
    update_last_accessed(pool, read_key).await?;
    sqlx::query_as::<_, Badge>("SELECT * FROM badge WHERE read_key = ?")
        .bind(read_key)
        .fetch_one(pool)
        .await
}

pub async fn check_write_key(
    pool: &MySqlPool,
    read_key: &Vec<u8>,
    write_key: &Vec<u8>,
) -> Result<bool, sqlx::Error> {
    let badge = get_badge_from_database(pool, read_key).await?;
    Ok(verify_password(write_key.as_slice(), badge.hash.as_slice()))
}
