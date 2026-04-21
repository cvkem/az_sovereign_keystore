use axum::{
    body::{self, Bytes}, extract::Path, http::{StatusCode, header::HeaderMap}
};
use axum::extract;
use crate::managed_id::get_secret_client;
use crate::obfusicate::mask_string;
use azure_security_keyvault_secrets::{models::SetSecretParameters, ResourceExt};
use anyhow::{Result, anyhow};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SetSecret {
    value: String,
}



/// The generic REST guidelines for Azure are https://learn.microsoft.com/en-us/rest/api/azure/
/// This also shows how to authorize interactive (Authorization code grant) and for systems (client credentials grant).
///  The client credentials grans is detailled in: https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-client-creds-grant-flow#get-a-token
/// 

/// Azure set_secret documentation: https://learn.microsoft.com/en-us/rest/api/keyvault/secrets/set-secret/set-secret 
/// Specs:
/// PUT {vaultBaseUrl}/secrets/{secret-name}?api-version=2025-07-01
/// 

pub async fn set_secret_aux(secret_name: &str, body_content: SetSecret) -> Result<()> {

    let client = get_secret_client().await.map_err(|e| anyhow!("failed with error '{e:?}'"))?;

    let secret_value = body_content.value;
    let masked_secret = mask_string(secret_name, &secret_value)?;

    // Create a new secret using the secret client.
    let secret_set_parameters = SetSecretParameters {
        value: Some(masked_secret.into()),
        ..Default::default()
    };

    let secret_result = client
        .set_secret(&secret_name, secret_set_parameters.try_into()?, None)
        .await?
        .into_model()?;

    println!(
        "Secret Name: {:?}, Value: {:?}, Version: {:?}",
        secret_result.resource_id()?.name,

        secret_result.value,
        secret_result.resource_id()?.version
    );

    Ok(())
}



pub async fn set_secret(Path((key_vault, secret_name)): Path<(String, String)>, headers: HeaderMap, extract::Json(body_content): extract::Json<SetSecret>) -> (StatusCode, HeaderMap) {

    if set_secret_aux(&secret_name, body_content).await.is_ok() {
        (StatusCode::OK, HeaderMap::new())
    } else {
        (StatusCode::BAD_REQUEST, HeaderMap::new())
    }

}

/// Azure get_secret documentation: https://learn.microsoft.com/en-us/rest/api/keyvault/secrets/get-secret/get-secret
/// Specs:
/// GET {vaultBaseUrl}/secrets/{secret-name}/{secret-version}?api-version=2025-07-01

pub async fn get_secret(Path((key_vault, secret_name)): Path<(String, String)>, headers: HeaderMap) -> (StatusCode, HeaderMap, Bytes) {

    (StatusCode::INTERNAL_SERVER_ERROR, HeaderMap::new(), Bytes::new())
}
