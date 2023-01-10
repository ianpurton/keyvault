mod accept_invite;
mod create_invite;
mod delete_member;
mod index;
mod new_team;
mod set_name;
mod switch;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id", get(index::index))
        .route("/app/team/:organisation_id/switch", get(switch::switch))
        .route(
            "/app/invite/:invite_selector/:invite_validator",
            get(accept_invite::invite),
        )
        .route(
            "/app/team/:organisation_id/create_invite",
            post(create_invite::create_invite),
        )
        .route(
            "/app/team/:organisation_id/delete",
            post(delete_member::delete),
        )
        .route(
            "/app/team/:organisation_id/set_name",
            post(set_name::set_name),
        )
        .route("/app/team/:organisation_id/new", post(new_team::new_team))
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}", organisation_id)
}

pub fn switch_route(organisation_id: i32) -> String {
    format!("/app/team/{}/switch", organisation_id)
}
