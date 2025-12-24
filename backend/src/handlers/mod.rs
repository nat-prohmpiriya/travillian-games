mod army;
mod auth;
mod building;
mod troop;
mod village;

use axum::{middleware, routing::{delete, get, post, put}, Router};

use crate::middleware::auth_middleware;
use crate::AppState;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes(state.clone()))
        .nest("/villages", village_routes(state.clone()))
        .nest("/map", map_routes(state.clone()))
        .nest("/troops", troop_routes(state.clone()))
        .nest("/reports", report_routes(state.clone()))
        .nest("/scout-reports", scout_report_routes(state.clone()))
        // Public routes (no auth required)
        .merge(public_routes())
}

fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/troops/definitions", get(troop::get_definitions))
}

fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(auth::me))
        .route("/sync", post(auth::sync_user))
        .route("/profile", put(auth::update_profile))
        .route("/account", delete(auth::delete_account))
        .route("/logout", delete(auth::logout))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn village_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(village::list_villages))
        .route("/", post(village::create_village))
        .route("/{id}", get(village::get_village))
        .route("/{id}", put(village::update_village))
        // Building routes nested under village
        .route("/{village_id}/buildings", get(building::list_buildings))
        .route("/{village_id}/buildings/queue", get(building::get_build_queue))
        .route("/{village_id}/buildings/{slot}", post(building::build))
        .route("/{village_id}/buildings/{slot}/upgrade", post(building::upgrade))
        .route("/{village_id}/buildings/{slot}", delete(building::demolish))
        // Troop routes nested under village
        .route("/{village_id}/troops", get(troop::list_troops))
        .route("/{village_id}/troops/queue", get(troop::get_training_queue))
        .route("/{village_id}/troops/train", post(troop::train_troops))
        .route("/{village_id}/troops/queue/{queue_id}", delete(troop::cancel_training))
        // Army routes nested under village
        .route("/{village_id}/armies", post(army::send_army))
        .route("/{village_id}/armies/outgoing", get(army::list_outgoing))
        .route("/{village_id}/armies/incoming", get(army::list_incoming))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn map_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(village::get_map))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn troop_routes(_state: AppState) -> Router<AppState> {
    // Troop definitions moved to public_routes
    // Protected troop routes are nested under /villages/{village_id}/troops
    Router::new()
}

fn report_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(army::list_reports))
        .route("/unread-count", get(army::get_unread_count))
        .route("/{report_id}", get(army::get_report))
        .route("/{report_id}/read", post(army::mark_report_read))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn scout_report_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(army::list_scout_reports))
        .route("/{report_id}", get(army::get_scout_report))
        .route("/{report_id}/read", post(army::mark_scout_report_read))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}
