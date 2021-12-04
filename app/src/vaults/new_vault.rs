use crate::errors::CustomError;
use crate::vault::{vault_client::VaultClient, VaultRequest};
use actix_web::{http, web, HttpResponse};
use serde::Deserialize;
use tonic::{metadata::MetadataValue, transport::Channel, Request};
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewVault {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn new(
    form: web::Form<NewVault>,
    config: web::Data<crate::config::Config>,
    logged_user: crate::user_id::UserId,
) -> Result<HttpResponse, CustomError> {
    let channel = Channel::builder(config.vault_server_uri.clone())
        .connect()
        .await?;

    let token = MetadataValue::from_str(&format!("x-user-id {}", logged_user.user_id))?;

    let mut client = VaultClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(VaultRequest {
        name: form.name.clone(),
    });

    let response = client.create_vault(request).await?;

    println!("RESPONSE={:?}", response);
    dbg!(&form);

    Ok(HttpResponse::SeeOther()
        .append_header((http::header::LOCATION, super::INDEX))
        .finish())
}
