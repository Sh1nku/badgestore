use crate::api::models::db::BadgeData;
use crate::api::models::verify_badge::{validate_color, VerifyBadge};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema, sqlx::FromRow, Clone)]
pub struct CreateBadge {
    #[schema(example = "Lines of code")]
    pub left_label: String,
    #[schema(example = "555555")]
    pub left_color: String,
    #[schema(example = "-100")]
    pub right_label: String,
    #[schema(example = "999999")]
    pub right_color: String,
}

impl From<CreateBadge> for rsbadges::Badge {
    fn from(value: CreateBadge) -> Self {
        rsbadges::Badge {
            label_text: value.left_label,
            // Append # if missing
            label_color: match value.left_color.starts_with('#') {
                true => value.left_color,
                false => format!("{}{}", "#", value.left_color),
            },
            msg_text: value.right_label,
            msg_color: match value.right_color.starts_with('#') {
                true => value.right_color,
                false => format!("{}{}", "#", value.right_color),
            },
            ..rsbadges::Badge::default()
        }
    }
}

impl VerifyBadge for CreateBadge {
    fn get_errors(&self) -> Option<HashMap<String, String>> {
        let mut errors = HashMap::new();
        if !validate_color(self.left_color.as_str()) {
            errors.insert("left_color".to_string(), "Invalid color".to_string());
        }
        if !validate_color(self.right_color.as_str()) {
            errors.insert("right_color".to_string(), "Invalid color".to_string());
        }
        match errors.is_empty() {
            true => None,
            false => Some(errors),
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, ToSchema, sqlx::FromRow, Clone)]
pub struct UpdateBadge {
    #[schema(example = "Lines of code")]
    pub left_label: Option<String>,
    #[schema(example = "555555")]
    pub left_color: Option<String>,
    #[schema(example = "-100")]
    pub right_label: Option<String>,
    #[schema(example = "999")]
    pub right_color: Option<String>,
}

impl VerifyBadge for UpdateBadge {
    fn get_errors(&self) -> Option<HashMap<String, String>> {
        let mut errors = HashMap::new();
        match &self.left_color {
            None => {}
            Some(c) => {
                if !validate_color(c.as_str()) {
                    errors.insert("left_color".to_string(), "Invalid color".to_string());
                }
            }
        }
        match &self.right_color {
            None => {}
            Some(c) => {
                if !validate_color(c.as_str()) {
                    errors.insert("right_color".to_string(), "Invalid color".to_string());
                }
            }
        }
        match errors.is_empty() {
            true => None,
            false => Some(errors),
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, ToSchema, sqlx::FromRow, Clone)]
pub struct KeyResult {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub read_key: Vec<u8>,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub write_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShieldsBadge {
    #[schema(example = "1")]
    pub schema_version: u8,
    #[schema(example = "Lines of code")]
    pub label: String,
    #[schema(example = "#555555")]
    pub label_color: String,
    #[schema(example = "-100")]
    pub message: String,
    #[schema(example = "#999999")]
    pub color: String,
}

impl From<BadgeData> for ShieldsBadge {
    fn from(value: BadgeData) -> Self {
        Self {
            schema_version: 1,
            label: value.left_label,
            label_color: format!("#{}", value.left_color),
            message: value.right_label,
            color: format!("#{}", value.right_color),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BadgenBadge {
    pub subject: String,
    pub status: String,
    pub color: String,
    pub label_color: String,
}

impl From<BadgeData> for BadgenBadge {
    fn from(value: BadgeData) -> Self {
        Self {
            subject: value.left_label,
            label_color: value.left_color,
            status: value.right_label,
            color: value.right_color,
        }
    }
}
