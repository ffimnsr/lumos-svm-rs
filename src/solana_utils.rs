use solana_account_decoder::parse_token::{
  TokenAccountType,
  is_known_spl_token_id,
  parse_token_v3,
};
use solana_account_decoder::parse_token_extension::{
  UiExtension,
  UiMetadataPointer,
  UiTokenMetadata,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr as _;

/// Get the owners of the given addresses.
pub async fn get_owners<T: AsRef<str>>(rpc_endpoint: &str, addresses: &[T]) -> anyhow::Result<Vec<String>> {
  let mut owners = Vec::with_capacity(addresses.len());

  for address in addresses {
    match get_owner(rpc_endpoint, address.as_ref()).await {
      Ok(owner) => {
        owners.push(owner);
      },
      Err(_) => continue,
    }
  }

  Ok(owners)
}

/// Get the owner of the given address.
pub async fn get_owner(rpc_endpoint: &str, address: &str) -> anyhow::Result<String> {
  let client = RpcClient::new(rpc_endpoint);
  let pubkey = Pubkey::from_str(address).map_err(|e| anyhow::anyhow!("Invalid address format: {}", e))?;

  let account = client.get_account(&pubkey)?;
  Ok(account.owner.to_string())
}

/// Get the token details of the given address.
#[derive(Debug)]
pub struct TokenDetails {
  pub owner: String,
  pub mint_authority: Option<String>,
  pub freeze_authority: Option<String>,
  pub update_authority: Option<String>,
  pub decimals: u8,
  pub supply: String,
  pub is_initialized: bool,
  pub extensions: bool,
  pub metadata: Option<TokenMetadata>,
}

/// Token metadata definition.
#[derive(Debug)]
pub struct TokenMetadata {
  pub authority: Option<String>,
  pub metadata_address: Option<String>,
  pub update_authority: Option<String>,
  pub name: Option<String>,
  pub symbol: Option<String>,
  pub uri: Option<String>,
}

/// Get the token details of the given address.
pub async fn get_token_details(rpc_endpoint: &str, address: &str) -> anyhow::Result<TokenDetails> {
  let client = RpcClient::new(rpc_endpoint);
  let pubkey =
    Pubkey::from_str(address).map_err(|e| anyhow::anyhow!("Invalid mint address format: {}", e))?;

  let account = client
    .get_account_with_commitment(&pubkey, client.commitment())?
    .value
    .ok_or_else(|| anyhow::anyhow!("Account not found"))?;

  if !is_known_spl_token_id(&account.owner) {
    return Err(anyhow::anyhow!("Not a token mint account"));
  }

  if let Ok(token_mint) = parse_token_v3(&account.data, None) {
    match token_mint {
      TokenAccountType::Mint(mint) => {
        let extensions = !mint.extensions.is_empty();
        let mut metadata = None;
        if extensions {
          for ext in mint.extensions.iter() {
            match ext {
              UiExtension::MetadataPointer(UiMetadataPointer {
                authority,
                metadata_address,
              }) => {
                metadata = Some(TokenMetadata {
                  authority: authority.clone(),
                  metadata_address: metadata_address.clone(),
                  update_authority: None,
                  name: None,
                  symbol: None,
                  uri: None,
                });
              },
              UiExtension::TokenMetadata(UiTokenMetadata {
                update_authority,
                name,
                symbol,
                uri,
                ..
              }) => {
                if let Some(meta) = metadata.as_mut() {
                  meta.update_authority = update_authority.clone();
                  meta.name = Some(name.clone());
                  meta.symbol = Some(symbol.clone());
                  meta.uri = Some(uri.clone());
                } else {
                  metadata = Some(TokenMetadata {
                    authority: None,
                    metadata_address: None,
                    update_authority: update_authority.clone(),
                    name: Some(name.clone()),
                    symbol: Some(symbol.clone()),
                    uri: Some(uri.clone()),
                  });
                }
              },
              _ => continue,
            }
          }
        }

        Ok(TokenDetails {
          owner: account.owner.to_string(),
          mint_authority: mint.mint_authority,
          freeze_authority: mint.freeze_authority,
          update_authority: None,
          decimals: mint.decimals,
          supply: mint.supply,
          is_initialized: mint.is_initialized,
          extensions,
          metadata,
        })
      },
      _ => Err(anyhow::anyhow!("Not a mint account")),
    }
  } else {
    Err(anyhow::anyhow!("Failed to parse token mint account"))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio::runtime::Runtime;

  #[test]
  fn test_get_owners() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
      let rpc_endpoint = "https://eclipse.lgns.net/";
      let accounts = [
        "GU7NS9xCwgNPiAdJ69iusFrRfawjDDPjeMBovhV1d4kn",
        "AKEWE7Bgh87GPp171b4cJPSSZfmZwQ3KaqYqXoKLNAEE",
      ];

      let owners = get_owners(rpc_endpoint, &accounts).await.unwrap();
      assert_eq!(owners.len(), accounts.len());
    });
  }
}
