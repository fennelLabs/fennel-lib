use subxt::{
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
};



/// To run this example, a local fennel node should be running.
///
/// ```bash
/// curl "https://github.com/paritytech/polkadot/releases/download/v0.9.13/polkadot" --output /usr/local/bin/polkadot --location
/// fennel --dev --tmp
/// 
/// curl -sX POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"state_getMetadata", "id": 1}' localhost:9933 \
///                 | jq .result \
///                 | cut -d '"' -f 2 \
///                 | xxd -r -p > ./fennel.scale
/// ```
#[subxt::subxt(runtime_metadata_path = "src/fennel/fennel-metadata.scale")]
pub mod fennel {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<fennel::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    let mut iter = api.storage().system().account_iter(None).await?;

    while let Some((key, account)) = iter.next().await? {
        println!("{}: {}", hex::encode(key), account.data.free);
    }
    Ok(())
}
