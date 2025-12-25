mod alliance;
mod army;
mod auth;
mod building;
mod hero;
mod message;
mod shop;
mod troop;
mod village;
pub mod ws;

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
        .nest("/armies", army_routes(state.clone()))
        .nest("/support-sent", support_routes(state.clone()))
        .nest("/alliances", alliance_routes(state.clone()))
        .nest("/messages", message_routes(state.clone()))
        .nest("/conversations", conversation_routes(state.clone()))
        .nest("/alliance-messages", alliance_message_routes(state.clone()))
        .nest("/shop", shop_routes(state.clone()))
        .nest("/heroes", hero_routes(state.clone()))
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
        .route("/{village_id}/stationed", get(army::list_stationed))
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

fn army_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/{army_id}/recall", post(army::recall_support))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn support_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(army::list_support_sent))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn alliance_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Alliance CRUD
        .route("/", post(alliance::create_alliance))
        .route("/", get(alliance::list_alliances))
        .route("/my", get(alliance::get_my_alliance))
        .route("/leave", post(alliance::leave_alliance))
        .route("/{id}", get(alliance::get_alliance))
        .route("/{id}", put(alliance::update_alliance))
        .route("/{id}", delete(alliance::disband_alliance))
        // Members
        .route("/{id}/members", get(alliance::list_members))
        .route("/{id}/invite", post(alliance::invite_player))
        .route("/{id}/members/{user_id}", delete(alliance::kick_member))
        .route("/{id}/members/{user_id}/role", put(alliance::update_member_role))
        // Invitations
        .route("/invitations", get(alliance::get_invitations))
        .route("/invitations/{invitation_id}/respond", post(alliance::respond_invitation))
        // Diplomacy
        .route("/{id}/diplomacy", get(alliance::list_diplomacy))
        .route("/{id}/diplomacy", post(alliance::set_diplomacy))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn message_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(message::send_message))
        .route("/inbox", get(message::get_inbox))
        .route("/sent", get(message::get_sent))
        .route("/unread-count", get(message::get_unread_count))
        .route("/{id}", get(message::get_message))
        .route("/{id}", delete(message::delete_message))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn conversation_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(message::get_conversations))
        .route("/{id}/messages", get(message::get_conversation_messages))
        .route("/{id}/reply", post(message::reply_to_conversation))
        .route("/{id}", delete(message::delete_conversation))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn alliance_message_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(message::send_alliance_message))
        .route("/", get(message::get_alliance_messages))
        .route("/{id}", get(message::get_alliance_message))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn shop_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/packages", get(shop::get_packages))
        .route("/subscriptions", get(shop::get_subscription_prices))
        // Stripe webhook (no auth, verified by signature)
        .route("/webhook", post(shop::stripe_webhook))
        // Protected routes
        .route("/balance", get(shop::get_balance))
        .route("/checkout", post(shop::create_checkout))
        .route("/subscriptions/buy", post(shop::buy_subscription))
        .route("/transactions", get(shop::get_transactions))
        // Gold features
        .route("/features/finish-now", post(shop::use_finish_now))
        .route("/features/npc-merchant", post(shop::use_npc_merchant))
        .route("/features/production-bonus", post(shop::use_production_bonus))
        .route("/features/book-of-wisdom", post(shop::use_book_of_wisdom))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn hero_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Hero CRUD
        .route("/", get(hero::list_heroes))
        .route("/", post(hero::create_hero))
        .route("/{id}", get(hero::get_hero))
        .route("/{id}/home", put(hero::change_home_village))
        .route("/{id}/attributes", put(hero::assign_attributes))
        // Hero Slots
        .route("/slots/buy", post(hero::buy_hero_slot))
        // Inventory
        .route("/{id}/inventory", get(hero::get_inventory))
        .route("/{id}/equip", post(hero::equip_item))
        .route("/{id}/unequip", post(hero::unequip_item))
        .route("/{id}/use-item", post(hero::use_item))
        .route("/{hero_id}/items/{item_id}", delete(hero::sell_item))
        // Adventures
        .route("/adventures/available", get(hero::get_available_adventures))
        .route("/{id}/adventures", post(hero::start_adventure))
        .route("/{id}/adventures/active", get(hero::get_active_adventure))
        // Revive
        .route("/{id}/revive-info", get(hero::get_revive_info))
        .route("/{id}/revive", post(hero::revive_hero))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}
