use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::cornucopia::types;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use lettre::Message;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use validator::Validate;
use crate::cornucopia::types::public::{AuditAction, AuditAccessType};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewInvite {
    #[validate(length(min = 1, message = "The email is mandatory"))]
    pub email: String,
    pub admin: Option<String>
}

pub async fn create_invite(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<crate::config::Config>,
    Form(new_invite): Form<NewInvite>,
    authentication: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    let invite_hash = create(&pool, &authentication, &new_invite).await?;

    let invitation_verifier_base64 = invite_hash.0;
    let invitation_selector_base64 = invite_hash.1;

    if let Some(smtp_config) = &config.smtp_config {
        let url = format!(
            "{}/app/team/accept_invite/?invite_selector={}&invite_validator={}",
            smtp_config.domain, invitation_selector_base64, invitation_verifier_base64
        );

        let body = format!(
            "
                Click {} to accept the invite
            ",
            url
        )
        .trim()
        .to_string();

        let email = Message::builder()
            .from(smtp_config.from_email.clone())
            .to(new_invite.email.parse().unwrap())
            .subject("You are invited to a Cloak Team")
            .body(body)
            .unwrap();

        crate::email::send_email(&config, email)
    }

    let client = pool.get().await?;
    queries::audit::insert(
        &client,
        &(current_user.user_id as i32),
        &organisation_id,
        &AuditAction::CreateInvite,
        &AuditAccessType::Web,
        &format!("{} invited", &new_invite.email)
    )
    .await?;

    let team = queries::organisations::organisation(&client, &organisation_id).await?;

    crate::layout::redirect_and_snackbar(&super::index_route(team.id), "Invitation Created")
}

pub async fn create(
    pool: &Pool,
    current_user: &Authentication,
    new_invite: &NewInvite,
) -> Result<(String, String), CustomError> {
    let client = pool.get().await?;

    let org =
        queries::organisations::get_primary_organisation(&client, &(current_user.user_id as i32))
            .await?;

    let invitation_selector = rand::thread_rng().gen::<[u8; 6]>();
    let invitation_selector_base64 = base64::encode_config(invitation_selector, base64::URL_SAFE_NO_PAD);
    let invitation_verifier = rand::thread_rng().gen::<[u8; 8]>();
    let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
    let invitation_verifier_hash_base64 =
        base64::encode_config(invitation_verifier_hash, base64::URL_SAFE_NO_PAD);
    let invitation_verifier_base64 = base64::encode_config(invitation_verifier, base64::URL_SAFE_NO_PAD);

    let roles = if new_invite.admin.is_some() {
        vec!(types::public::Role::Administrator, types::public::Role::Collaborator)
    } else {
        vec!(types::public::Role::Collaborator)
    };

    queries::invitations::insert_invitation(
        &client,
        &org.id,
        &new_invite.email,
        &invitation_selector_base64,
        &invitation_verifier_hash_base64,
        &roles
    )
    .await?;

    Ok((invitation_verifier_base64, invitation_selector_base64))
}
