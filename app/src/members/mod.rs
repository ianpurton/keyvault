mod add_member;
mod index;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/vault/:id/members";
pub static ADD: &str = "/app/vault/:id/members/add";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(ADD, post(add_member::add))
}

pub fn member_route(vault_id: u32) -> String {
    format!("/app/vault/{}/members", vault_id)
}

pub fn add_route(vault_id: i32) -> String {
    format!("/app/vault/{}/members/add", vault_id)
}
