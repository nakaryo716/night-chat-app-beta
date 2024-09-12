use axum::routing::{delete, post};
use axum::{routing::get, Router};

use crate::app::AppState;
use crate::controllers::rooms::{create_room_handler, delete_room_handler, room_list_handler};
use crate::controllers::view::index;
use crate::controllers::websocket::websocket_upgrade_handler;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/create_room", post(create_room_handler))
        .route("/room_ls", get(room_list_handler))
        .route("/delete_room/:room_id", delete(delete_room_handler))
        .route("/websocket/:room_id", get(websocket_upgrade_handler))
        .with_state(state)
}
