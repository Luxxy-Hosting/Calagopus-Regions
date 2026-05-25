use indexmap::IndexMap;
use shared::{
    State,
    extensions::{Extension, ExtensionPermissionsBuilder, ExtensionRouteBuilder},
    permissions::PermissionGroup,
};

mod models;
mod routes;
mod services;

#[derive(Default)]
pub struct ExtensionStruct;

#[async_trait::async_trait]
impl Extension for ExtensionStruct {
    async fn initialize(&mut self, _state: State) {
        tracing::info!("region extension initialized");
    }

    async fn initialize_router(
        &mut self,
        state: State,
        builder: ExtensionRouteBuilder,
    ) -> ExtensionRouteBuilder {
        builder
            .add_admin_api_router(|router| router.nest("/regions", routes::admin_router(&state)))
            .add_client_server_api_router(|router| {
                router.nest("/region", routes::client_server_router(&state))
            })
    }

    async fn initialize_permissions(
        &mut self,
        _state: State,
        builder: ExtensionPermissionsBuilder,
    ) -> ExtensionPermissionsBuilder {
        builder.add_admin_permission_group(
            "regions",
            PermissionGroup {
                description: "Permissions for managing server regions.",
                permissions: IndexMap::from([
                    ("view", "Allows viewing configured regions."),
                    (
                        "manage",
                        "Allows creating, updating, and deleting regions and node assignments.",
                    ),
                ]),
            },
        )
    }
}
