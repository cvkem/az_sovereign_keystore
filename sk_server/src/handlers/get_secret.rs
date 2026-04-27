use crate::managed_id::get_secret_client;
use crate::obfuscate::mask_string;
use azure_security_keyvault_secrets::ResourceExt;
use anyhow::{Result, anyhow, bail};



pub async fn get_secret_aux(secret_name: &str) -> Result<String> {

    let client = get_secret_client().await.map_err(|e| anyhow!("failed with error '{e:?}'"))?;

    let secret_result = client
        .get_secret(&secret_name, None)
        .await?
        .into_model()?;

    let masked_secret = secret_result.value.ok_or(bail!("No secret with name {secret_name} found"))?;
    println!(
        "Result Get_secret -> Name: {:?}, Value: {:?}, Version: {:?}",
        secret_result.resource_id()?.name,
        secret_result.value,
        secret_result.resource_id()?.version
    );

    let unmasked_secret = mask_string(secret_name, &masked_secret)?;

    Ok(unmasked_secret)
}
