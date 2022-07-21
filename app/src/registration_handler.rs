use crate::cornucopia::queries;
use crate::{authentication::Authentication, errors::CustomError};
use axum::{
    extract::Extension,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub static INDEX: &str = "/app/post_registration";

pub fn routes() -> Router {
    Router::new().route(INDEX, get(post_registration))
}

// After a user has logged in or registered, check they have an entry in
// the organisation table. If not, then create one.
pub async fn post_registration(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    let org = queries::organisations::get_primary_organisation(&client, &(current_user.user_id as i32)).await;

    if let Ok(org) = org {
        return Ok(Redirect::to(&crate::vaults::index_route(org.id)))
    } else {

        let inserted_org_id =
            queries::organisations::insert_organisation(&client, &(current_user.user_id as i32))
                .await?;

        queries::organisations::insert_user_into_org(
            &client,
            &(current_user.user_id as i32),
            &inserted_org_id,
            &true,
        )
        .await?;
        return Ok(Redirect::to(&crate::vaults::index_route(inserted_org_id)))
    } 
}
