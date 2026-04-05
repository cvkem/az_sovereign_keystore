use axum::{
    body::Bytes, extract::Path, http::{StatusCode, header::{HeaderMap}}
};


/// Azure set_secret documentation: https://learn.microsoft.com/en-us/rest/api/keyvault/secrets/set-secret/set-secret 
/// Specs:
/// PUT {vaultBaseUrl}/secrets/{secret-name}?api-version=2025-07-01
/// 


pub async fn set_secret(Path((key_vault, secret_name)): Path<(String, String)>, headers: HeaderMap, body_content: Bytes) -> (StatusCode, HeaderMap) {

    (StatusCode::INTERNAL_SERVER_ERROR, HeaderMap::new())
}

/// Azure get_secret documentation: https://learn.microsoft.com/en-us/rest/api/keyvault/secrets/get-secret/get-secret
/// Specs:
/// GET {vaultBaseUrl}/secrets/{secret-name}/{secret-version}?api-version=2025-07-01

pub async fn get_secret(Path((key_vault, secret_name)): Path<(String, String)>, headers: HeaderMap) -> (StatusCode, HeaderMap, Bytes) {

    (StatusCode::INTERNAL_SERVER_ERROR, HeaderMap::new(), Bytes::new())
}
