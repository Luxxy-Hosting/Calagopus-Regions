use crate::{
    models::{ApiRegion, CreateRegionRequest, UpdateRegionRequest},
    services::manager,
};
use axum::{extract::Path, http::StatusCode};
use serde::Serialize;
use shared::{
    ApiError, GetState, Payload,
    models::{admin_activity::GetAdminActivityLogger, user::GetPermissionManager},
    response::{ApiResponse, ApiResponseResult},
};
use utoipa::ToSchema;
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};
use uuid::Uuid;

use super::State;

mod list_regions {
    use super::*;

    #[derive(ToSchema, Serialize)]
    struct Response {
        regions: Vec<ApiRegion>,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = FORBIDDEN, body = ApiError),
    ))]
    pub async fn route(state: GetState, permissions: GetPermissionManager) -> ApiResponseResult {
        permissions.has_admin_permission("regions.view")?;

        let regions = manager::list_regions(&state).await?;

        ApiResponse::new_serialized(Response { regions }).ok()
    }
}

mod get_region {
    use super::*;

    #[derive(ToSchema, Serialize)]
    struct Response {
        region: ApiRegion,
    }

    #[utoipa::path(get, path = "/{region}", responses(
        (status = OK, body = inline(Response)),
        (status = NOT_FOUND, body = ApiError),
        (status = FORBIDDEN, body = ApiError),
    ), params(("region" = Uuid, Path, description = "The region UUID")))]
    pub async fn route(
        state: GetState,
        permissions: GetPermissionManager,
        Path(region_uuid): Path<Uuid>,
    ) -> ApiResponseResult {
        permissions.has_admin_permission("regions.view")?;

        match manager::get_region(&state, region_uuid).await? {
            Some(region) => ApiResponse::new_serialized(Response { region }).ok(),
            None => ApiResponse::new_serialized(ApiError::new_value(&["Region not found."]))
                .with_status(StatusCode::NOT_FOUND)
                .ok(),
        }
    }
}

mod create_region {
    use super::*;

    #[derive(ToSchema, Serialize)]
    struct Response {
        region: ApiRegion,
    }

    #[utoipa::path(post, path = "/", request_body = CreateRegionRequest, responses(
        (status = OK, body = inline(Response)),
        (status = BAD_REQUEST, body = ApiError),
        (status = FORBIDDEN, body = ApiError),
    ))]
    pub async fn route(
        state: GetState,
        permissions: GetPermissionManager,
        activity_logger: GetAdminActivityLogger,
        Payload(request): Payload<CreateRegionRequest>,
    ) -> ApiResponseResult {
        permissions.has_admin_permission("regions.manage")?;

        if let Err(errors) = shared::utils::validate_data(&request) {
            return ApiResponse::new_serialized(ApiError::new_strings_value(errors))
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        let region = manager::create_region(&state, request).await?;

        activity_logger
            .log(
                "regions:region.create",
                serde_json::json!({
                    "region_uuid": region.uuid,
                    "name": region.name,
                    "country_code": region.country_code,
                }),
            )
            .await;

        ApiResponse::new_serialized(Response { region }).ok()
    }
}

mod update_region {
    use super::*;

    #[derive(ToSchema, Serialize)]
    struct Response {
        region: ApiRegion,
    }

    #[utoipa::path(patch, path = "/{region}", request_body = UpdateRegionRequest, responses(
        (status = OK, body = inline(Response)),
        (status = BAD_REQUEST, body = ApiError),
        (status = NOT_FOUND, body = ApiError),
        (status = FORBIDDEN, body = ApiError),
    ), params(("region" = Uuid, Path, description = "The region UUID")))]
    pub async fn route(
        state: GetState,
        permissions: GetPermissionManager,
        activity_logger: GetAdminActivityLogger,
        Path(region_uuid): Path<Uuid>,
        Payload(request): Payload<UpdateRegionRequest>,
    ) -> ApiResponseResult {
        permissions.has_admin_permission("regions.manage")?;

        if let Err(errors) = shared::utils::validate_data(&request) {
            return ApiResponse::new_serialized(ApiError::new_strings_value(errors))
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        match manager::update_region(&state, region_uuid, request).await? {
            Some(region) => {
                activity_logger
                    .log(
                        "regions:region.update",
                        serde_json::json!({
                            "region_uuid": region.uuid,
                            "name": region.name,
                        }),
                    )
                    .await;

                ApiResponse::new_serialized(Response { region }).ok()
            }
            None => ApiResponse::new_serialized(ApiError::new_value(&["Region not found."]))
                .with_status(StatusCode::NOT_FOUND)
                .ok(),
        }
    }
}

mod delete_region {
    use super::*;

    #[utoipa::path(delete, path = "/{region}", responses(
        (status = NO_CONTENT),
        (status = NOT_FOUND, body = ApiError),
        (status = FORBIDDEN, body = ApiError),
    ), params(("region" = Uuid, Path, description = "The region UUID")))]
    pub async fn route(
        state: GetState,
        permissions: GetPermissionManager,
        activity_logger: GetAdminActivityLogger,
        Path(region_uuid): Path<Uuid>,
    ) -> ApiResponseResult {
        permissions.has_admin_permission("regions.manage")?;

        if !manager::delete_region(&state, region_uuid).await? {
            return ApiResponse::new_serialized(ApiError::new_value(&["Region not found."]))
                .with_status(StatusCode::NOT_FOUND)
                .ok();
        }

        activity_logger
            .log(
                "regions:region.delete",
                serde_json::json!({ "region_uuid": region_uuid }),
            )
            .await;

        ApiResponse::new(axum::body::Body::empty()).with_status(StatusCode::NO_CONTENT).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(list_regions::route))
        .routes(routes!(get_region::route))
        .routes(routes!(create_region::route))
        .routes(routes!(update_region::route))
        .routes(routes!(delete_region::route))
        .with_state(state.clone())
}
