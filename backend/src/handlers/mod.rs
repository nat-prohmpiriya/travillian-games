use axum::Router;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
    // Routes will be added here as we implement handlers
    // .nest("/auth", auth::routes())
    // .nest("/servers", server::routes())
    // .nest("/villages", village::routes())
}
