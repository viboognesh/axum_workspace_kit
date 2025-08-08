use std::sync::Arc;

use axum::{Extension, Router, middleware};
use tower_http::trace::TraceLayer;

use crate::{
    AppState,
    handlers::{
        auth::auth_handler, permissions::permissions_handler, role::role_handler,
        user::user_handler, workspace::workspace_handler, workspace_user::workspace_user_handler,
    },
    middleware::jwt_auth_middleware::auth_middleware,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/auth", auth_handler())
        .nest(
            "/user",
            user_handler().layer(middleware::from_fn(auth_middleware)),
        )
        .nest(
            "/workspace",
            workspace_handler().layer(middleware::from_fn(auth_middleware)),
        )
        .nest(
            "/role",
            role_handler().layer(middleware::from_fn(auth_middleware)),
        )
        .nest(
            "/permissions",
            permissions_handler().layer(middleware::from_fn(auth_middleware)),
        )
        .nest(
            "/workspace_user",
            workspace_user_handler().layer(middleware::from_fn(auth_middleware)),
        )
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new().nest("/api", api_route)
}
