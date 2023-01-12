use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::{IntoResponse, Redirect},
};
use db::queries;
use db::Pool;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize, Debug)]
pub struct Invite {
    invite_selector: String,
    invite_validator: String,
}

pub async fn invite(
    Path(invite): Path<Invite>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    let team_id = accept_invitation(
        &pool,
        &current_user,
        &invite.invite_selector,
        &invite.invite_validator,
    )
    .await?;

    Ok(Redirect::to(&ui_components::routes::team::switch_route(
        team_id,
    )))
}

pub async fn accept_invitation(
    pool: &Pool,
    current_user: &Authentication,
    invitation_selector: &str,
    invitation_verifier: &str,
) -> Result<i32, CustomError> {
    let invitation_verifier = base64::decode_config(invitation_verifier, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CustomError::FaultySetup(e.to_string()))?;
    let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
    let invitation_verifier_hash_base64 =
        base64::encode_config(invitation_verifier_hash, base64::URL_SAFE_NO_PAD);

    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, current_user).await?;

    let invitation = queries::invitations::get_invitation()
        .bind(&transaction, &invitation_selector)
        .one()
        .await?;

    if invitation.invitation_verifier_hash == invitation_verifier_hash_base64 {
        let user = queries::users::user()
            .bind(&transaction, &current_user.user_id)
            .one()
            .await?;

        // Make sure the user accepting the invitation is the user that we emailed
        if user.email == invitation.email {
            let user = queries::users::get_by_email()
                .bind(&transaction, &user.email.as_ref())
                .one()
                .await?;

            queries::organisations::add_user_to_organisation()
                .bind(
                    &transaction,
                    &user.id,
                    &invitation.organisation_id,
                    &invitation.roles.as_ref(),
                )
                .await?;

            // I the user has not set their name yet, we do it for them based on the invitation.
            if (None, None) == (user.first_name, user.last_name) {
                queries::users::set_name()
                    .bind(
                        &transaction,
                        &invitation.first_name.as_ref(),
                        &invitation.last_name.as_ref(),
                        &current_user.user_id,
                    )
                    .await?;
            }

            queries::invitations::delete_invitation()
                .bind(
                    &transaction,
                    &invitation.email.as_ref(),
                    &invitation.organisation_id,
                )
                .await?;
        }
    }

    transaction.commit().await?;

    Ok(invitation.organisation_id)
}
