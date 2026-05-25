use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize, Clone)]
pub struct ApiRegion {
    pub uuid: Uuid,
    pub name: String,
    pub country_code: String,
    pub city: Option<String>,
    pub visible: bool,
    pub node_uuids: Vec<Uuid>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(ToSchema, Validate, Deserialize, Clone)]
pub struct CreateRegionRequest {
    #[garde(length(min = 1, max = 100))]
    pub name: String,
    #[garde(length(min = 2, max = 2))]
    pub country_code: String,
    #[garde(inner(length(max = 100)))]
    pub city: Option<String>,
    #[garde(skip)]
    pub visible: Option<bool>,
    #[garde(skip)]
    pub node_uuids: Option<Vec<Uuid>>,
}

#[derive(ToSchema, Validate, Deserialize, Clone)]
pub struct UpdateRegionRequest {
    #[garde(inner(length(min = 1, max = 100)))]
    pub name: Option<String>,
    #[garde(inner(length(min = 2, max = 2)))]
    pub country_code: Option<String>,
    #[garde(inner(length(max = 100)))]
    pub city: Option<String>,
    #[garde(skip)]
    pub visible: Option<bool>,
    #[garde(skip)]
    pub node_uuids: Option<Vec<Uuid>>,
}

#[derive(ToSchema, Serialize, Clone)]
pub struct ApiServerRegion {
    pub uuid: Uuid,
    pub name: String,
    pub country_code: String,
    pub city: Option<String>,
}
