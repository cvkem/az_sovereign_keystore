use azure_core::credentials::TokenCredential;
use azure_core::http::ClientMethodOptions;
use azure_identity::{ClientAssertion, ClientAssertionCredential, ManagedIdentityCredential};
use azure_security_keyvault_secrets::SecretClient;
use std::sync::{Arc, Mutex};


#[derive(Debug)]
struct AccessTokenAssertion {
    credential: Arc<dyn TokenCredential>,
}

#[async_trait::async_trait]
impl ClientAssertion for AccessTokenAssertion {
    async fn secret(&self, _: Option<ClientMethodOptions<'_>>) -> azure_core::Result<String> {
        Ok(self
            .credential
            .get_token(&[&"api://AzureADTokenExchange/.default"], None)
            .await?
            .token
            .secret()
            .to_string())
    }
}

static MANAGED_ID:  Mutex<Option<Arc<ClientAssertionCredential<AccessTokenAssertion>>>> = Mutex::new(None);

/// get a credential of the current application based on the Managed Identity, using a configured tenant_ID and client_ID
pub async fn get_credential() -> Result<Arc<ClientAssertionCredential<AccessTokenAssertion>>, Box<dyn std::error::Error>> {

    let mut locked_token = MANAGED_ID.lock().expect("Trying to lock the managed-ID");

    // if no credential is available create a new one based on the Managed Identity
    if locked_token.is_none() {
        let assertion = AccessTokenAssertion {
            credential: ManagedIdentityCredential::new(None).expect("Managed Identity should be available"),
        };

        // this credential exchanges the managed identity's tokens for the specified Entra application's tokens
        let credential = ClientAssertionCredential::new(
            String::from("tenant ID"),   // get from params or get from environment, or are these extracted from the Managed identity
            String::from("client ID"),
            assertion,
            None,
        )?;
        // add the newly obtained AccessTokenAssertion
        *locked_token = Some(credential);
    }

    let credential_clone = locked_token.as_ref().expect("Credential should be set").clone();

    Ok(credential_clone)
}

/// Get a secret_client from the configured value
pub async fn get_secret_client() ->  Result<SecretClient, Box<dyn std::error::Error>> {
    let credential = get_credential().await?;
    let client = SecretClient::new("https://TODO.vault.azure.net/", credential, None)?;
    Ok(client)
}