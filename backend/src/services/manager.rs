use anyhow::anyhow;
use chrono::NaiveDateTime;
use shared::{State, database::DatabaseError};
use uuid::Uuid;

use crate::models::{ApiRegion, ApiServerRegion, CreateRegionRequest, UpdateRegionRequest};

#[derive(sqlx::FromRow)]
struct RegionRow {
    uuid: Uuid,
    name: String,
    country_code: String,
    city: Option<String>,
    visible: bool,
    node_uuids: Option<Vec<Uuid>>,
    created: NaiveDateTime,
    updated: NaiveDateTime,
}

fn row_to_api(row: RegionRow) -> ApiRegion {
    ApiRegion {
        uuid: row.uuid,
        name: row.name,
        country_code: row.country_code,
        city: row.city,
        visible: row.visible,
        node_uuids: row.node_uuids.unwrap_or_default(),
        created: row.created.and_utc(),
        updated: row.updated.and_utc(),
    }
}

const REGION_SELECT: &str = r#"
    SELECT
        r.uuid,
        r.name,
        r.country_code,
        r.city,
        r.visible,
        r.created,
        r.updated,
        ARRAY_REMOVE(ARRAY_AGG(nr.node_uuid), NULL) as node_uuids
    FROM ext_region_regions r
    LEFT JOIN ext_region_node_regions nr ON nr.region_uuid = r.uuid
"#;

pub async fn list_regions(state: &State) -> Result<Vec<ApiRegion>, DatabaseError> {
    let rows = sqlx::query_as::<_, RegionRow>(sqlx::AssertSqlSafe(format!(
        "{REGION_SELECT} GROUP BY r.uuid ORDER BY r.name"
    )))
    .fetch_all(state.database.read())
    .await?;

    Ok(rows.into_iter().map(row_to_api).collect())
}

pub async fn get_region(state: &State, uuid: Uuid) -> Result<Option<ApiRegion>, DatabaseError> {
    let row = sqlx::query_as::<_, RegionRow>(sqlx::AssertSqlSafe(format!(
        "{REGION_SELECT} WHERE r.uuid = $1 GROUP BY r.uuid"
    )))
    .bind(uuid)
    .fetch_optional(state.database.read())
    .await?;

    Ok(row.map(row_to_api))
}

pub async fn create_region(
    state: &State,
    req: CreateRegionRequest,
) -> Result<ApiRegion, anyhow::Error> {
    let mut tx = state.database.write().begin().await?;

    let uuid: Uuid = sqlx::query_scalar(
        "INSERT INTO ext_region_regions (name, country_code, city, visible)
         VALUES ($1, $2, $3, $4)
         RETURNING uuid",
    )
    .bind(&req.name)
    .bind(req.country_code.to_uppercase())
    .bind(&req.city)
    .bind(req.visible.unwrap_or(true))
    .fetch_one(&mut *tx)
    .await?;

    if let Some(node_uuids) = &req.node_uuids {
        set_node_assignments(&mut tx, uuid, node_uuids).await?;
    }

    tx.commit().await?;

    get_region(state, uuid)
        .await?
        .ok_or_else(|| anyhow!("region not found after creation"))
}

pub async fn update_region(
    state: &State,
    uuid: Uuid,
    req: UpdateRegionRequest,
) -> Result<Option<ApiRegion>, anyhow::Error> {
    let mut tx = state.database.write().begin().await?;

    let updated: Option<Uuid> = sqlx::query_scalar(
        "UPDATE ext_region_regions SET
            name = COALESCE($1, name),
            country_code = COALESCE(UPPER($2), country_code),
            city = CASE WHEN $3::boolean THEN $4 ELSE city END,
            visible = COALESCE($5, visible),
            updated = NOW()
         WHERE uuid = $6
         RETURNING uuid",
    )
    .bind(&req.name)
    .bind(&req.country_code)
    .bind(req.city.is_some() || req.country_code.as_deref() == Some(""))
    .bind(&req.city)
    .bind(req.visible)
    .bind(uuid)
    .fetch_optional(&mut *tx)
    .await?;

    if updated.is_none() {
        return Ok(None);
    }

    if let Some(node_uuids) = &req.node_uuids {
        set_node_assignments(&mut tx, uuid, node_uuids).await?;
    }

    tx.commit().await?;

    Ok(get_region(state, uuid).await?)
}

pub async fn delete_region(state: &State, uuid: Uuid) -> Result<bool, DatabaseError> {
    let result = sqlx::query("DELETE FROM ext_region_regions WHERE uuid = $1")
        .bind(uuid)
        .execute(state.database.write())
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_server_region(
    state: &State,
    node_uuid: Uuid,
) -> Result<Option<ApiServerRegion>, DatabaseError> {
    let row = sqlx::query_as::<_, (Uuid, String, String, Option<String>)>(
        r#"
        SELECT r.uuid, r.name, r.country_code, r.city
        FROM ext_region_regions r
        JOIN ext_region_node_regions nr ON nr.region_uuid = r.uuid
        WHERE nr.node_uuid = $1
          AND r.visible = TRUE
        "#,
    )
    .bind(node_uuid)
    .fetch_optional(state.database.read())
    .await?;

    Ok(row.map(|(uuid, name, country_code, city)| ApiServerRegion {
        uuid,
        name,
        country_code,
        city,
    }))
}

async fn set_node_assignments(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    region_uuid: Uuid,
    node_uuids: &[Uuid],
) -> Result<(), anyhow::Error> {
    sqlx::query("DELETE FROM ext_region_node_regions WHERE region_uuid = $1")
        .bind(region_uuid)
        .execute(&mut **tx)
        .await?;

    for &node_uuid in node_uuids {
        sqlx::query(
            "INSERT INTO ext_region_node_regions (node_uuid, region_uuid)
             VALUES ($1, $2)
             ON CONFLICT (node_uuid) DO UPDATE SET region_uuid = $2",
        )
        .bind(node_uuid)
        .bind(region_uuid)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}
