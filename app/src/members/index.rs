use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use deadpool_postgres::Pool;

pub async fn index(
    Path((team_id, vault_id)): Path<(i32, i32)>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let team = queries::organisations::organisation(&client, &team_id).await?;

    let org =
        queries::organisations::get_primary_organisation(&client, &(current_user.user_id as i32))
            .await?;

    // Blow up if the user doesn't have access to the vault
    queries::user_vaults::get(&client, &(current_user.user_id as i32), &vault_id).await?;

    let members = queries::user_vaults::get_users_dangerous(&client, &vault_id).await?;

    let non_members =
        queries::user_vaults::get_non_members_dangerous(&client, &vault_id, &org.id).await?;

    let user_vault =
        queries::user_vaults::get(&client, &(current_user.user_id as i32), &vault_id).await?;

    let environments =
        queries::environments::get_all(&client, &user_vault.vault_id, &(current_user.user_id as i32))
            .await?;

    let user = queries::users::get_dangerous(&client, &(current_user.user_id as i32)).await?;
    let initials = crate::layout::initials(&user.email, user.first_name, user.last_name);

    let mut buf = Vec::new();
    crate::templates::members::index_html(
        &mut buf, 
        &initials,
        "Members", 
        user_vault,
        members,
        non_members,
        environments,
        &team
    ).unwrap();
    let html = format!("{}", String::from_utf8_lossy(&buf));

    Ok(Html(html))
}