use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewVault {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_vault_key: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub public_key: String,
}

pub async fn new(
    current_user: Authentication,
    Form(new_vault): Form<NewVault>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    let vault_id =
        queries::vaults::insert(&client, &(current_user.user_id as i32), &new_vault.name).await?;

    let envs = queries::environments::setup_environments(&client, &vault_id).await?;

    queries::vaults::insert_user_vaults(
        &client,
        &(current_user.user_id as i32),
        &vault_id,
        &new_vault.public_key,
        &new_vault.encrypted_vault_key,
    )
    .await?;

    for env in envs {
        queries::environments::connect_environment_to_user(
            &client,
            &(current_user.user_id as i32),
            &env.id,
        )
        .await?;
    }

    crate::layout::redirect_and_snackbar(super::INDEX, "Vault Created")
}

markup::define! {
    VaultForm {

        form.m_form[method = "post", action=super::NEW] {
            new_vault[label="Add Vault"] {
                template[slot="body"] {
                    p {
                        "Vaults keep related secrets together.
                        For example you could have a vault called My Project with all
                        the secrets related to your project."
                    }

                    fieldset {
                        label[for="name"] { "Name *" }
                        input[type="text", required="", name="name"] {}
                        span.a_help_text { "Give your vault a name" }

                        input[required="", type="hidden",
                            name="encrypted_vault_key",
                            id="new-vault-key", autocomplete="off"] {}

                        input[id="public-key", type="hidden", required="", name="public_key"] {}
                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success[type = "submit"] { "Create Vault" }
                }
            }
        }

    }
}
