use axum::{
    body::Bytes, extract::Path, http::{StatusCode, header::HeaderMap}
};
use axum::extract;
use azure_security_keyvault_secrets::models::SetSecretParameters;

mod set_secret;
mod get_secret;

use set_secret::set_secret_aux;
use get_secret::get_secret_aux;


pub async fn set_secret(Path((_key_vault, secret_name)): Path<(String, String)>, _headers: HeaderMap, extract::Json(body_content): extract::Json<SetSecretParameters>) -> (StatusCode, HeaderMap, Bytes) {
    match set_secret_aux(&secret_name, body_content).await {
        Ok(res) => {
            // ensure headers and results are returned
            (StatusCode::OK, HeaderMap::new(), Bytes::new())
        },
        Err(err) => {
            // log the error
            (StatusCode::BAD_REQUEST, HeaderMap::new(), Bytes::new())
        }
    }
}

/// Azure get_secret documentation: https://learn.microsoft.com/en-us/rest/api/keyvault/secrets/get-secret/get-secret
/// Specs:
/// GET {vaultBaseUrl}/secrets/{secret-name}/{secret-version}?api-version=2025-07-01

pub async fn get_secret(Path((key_vault, secret_name)): Path<(String, String)>, headers: HeaderMap) -> (StatusCode, HeaderMap, Bytes) {
    match get_secret_aux(&secret_name).await {
        Ok(res) => {
            // ensure headers and results are returned
            (StatusCode::OK, HeaderMap::new(), Bytes::copy_from_slice(res.as_bytes()))
        },
        Err(err) => {
            // log the error
            (StatusCode::INTERNAL_SERVER_ERROR, HeaderMap::new(), Bytes::new())
        }
    }
}
