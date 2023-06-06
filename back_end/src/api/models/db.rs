use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use utoipa::ToSchema;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, ToSchema, sqlx::FromRow, Clone)]
pub struct Badge {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub read_key: Vec<u8>,
    pub hash: Vec<u8>,
    pub created_date: NaiveDateTime,
    pub last_modified: NaiveDateTime,
    pub last_accessed: NaiveDateTime,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, ToSchema, sqlx::FromRow, Clone)]
pub struct BadgeData {
    pub left_label: String,
    pub left_color: String,
    pub right_label: String,
    pub right_color: String,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub read_key: Vec<u8>,
}
impl From<BadgeData> for rsbadges::Badge {
    fn from(value: BadgeData) -> Self {
        rsbadges::Badge {
            label_text: value.left_label,
            label_color: format!("{}{}", "#", value.left_color),
            msg_text: value.right_label,
            msg_color: format!("{}{}", "#", value.right_color),
            ..rsbadges::Badge::default()
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, ToSchema, sqlx::FromRow, Clone)]
pub struct BadgeDataRaw {
    pub left_label: Vec<u8>,
    pub left_color: Vec<u8>,
    pub right_label: Vec<u8>,
    pub right_color: Vec<u8>,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub read_key: Vec<u8>,
}

impl TryFrom<BadgeDataRaw> for BadgeData {
    type Error = String;

    fn try_from(value: BadgeDataRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            left_label: String::from_utf8(value.left_label).map_err(|e| e.to_string())?,
            left_color: String::from_utf8(value.left_color).map_err(|e| e.to_string())?,
            right_label: String::from_utf8(value.right_label).map_err(|e| e.to_string())?,
            right_color: String::from_utf8(value.right_color).map_err(|e| e.to_string())?,
            read_key: value.read_key,
        })
    }
}
