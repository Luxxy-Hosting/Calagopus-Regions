use crate::{models::ApiServerRegion, services::manager};
use serde::Serialize;
use shared::{
    ApiError, GetState,
    models::{server::GetServer, user::GetPermissionManager},
    response::{ApiResponse, ApiResponseResult},
};
use utoipa::ToSchema;
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

use super::State;

mod get_server_region {
    use super::*;

    #[derive(ToSchema, Serialize)]
    struct Response {
        region: Option<ApiServerRegion>,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = FORBIDDEN, body = ApiError),
    ))]
    pub async fn route(
        state: GetState,
        permissions: GetPermissionManager,
        server: GetServer,
    ) -> ApiResponseResult {
        permissions.has_server_permission("server.read")?;

        let region = manager::get_server_region(&state, server.node.uuid).await?;

        ApiResponse::new_serialized(Response { region }).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get_server_region::route))
        .with_state(state.clone())
}
