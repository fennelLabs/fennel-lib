//! Fennel RPC Connection

mod error;

use std::sync::{Arc, Mutex};

use crate::{get_identity_database_handle, get_message_database_handle};
use rocksdb::DB;
use sp_keyring::AccountKeyring;
use subxt::{sp_core::sr25519::Pair, ClientBuilder, DefaultConfig, DefaultExtra, PairSigner};

pub use self::error::Error;

type RawIdentity = [u8; 4];

/// To run this example, a local fennel node should be running.
///
/// ```bash
/// curl "https://github.com/paritytech/polkadot/releases/download/v0.9.13/polkadot" --output /usr/local/bin/polkadot --location
/// ./fennel --dev --tmp
///
/// # to fetch the metadata from a running dev node
/// curl -sX POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"state_getMetadata", "id": 1}' localhost:9933 \
///                 | jq .result \
///                 | cut -d '"' -f 2 \
///                 | xxd -r -p > ./fennel-metadata.scale
/// ```
#[subxt::subxt(runtime_metadata_path = "src/fennel/fennel-metadata.scale")]
pub mod fennel {}

type RuntimeApi = fennel::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>;

pub struct TransactionHandler {
    runtime: RuntimeApi,
    // FIXME:: its not the best to mix blocking ops (rocksdb gets) with async, since
    // long-running operations will most certainly block the async executor.
    // however with our limited data (single gets/retrieves) this should be fast
    // enough that it is not noticeable.
    // alternatively, the database struct could spawn all getes onto the executor as a blocking op
    //identity_db: Arc<Mutex<DB>>,
    //messages_db: Arc<Mutex<DB>>,
}

impl TransactionHandler {
    /// Set up a new transaction handler with all needed resources.
    pub async fn new() -> Result<Self, Error> {
        println!("Instantiate TransactionHandler");
        let runtime = ClientBuilder::new()
            .set_url(String::from("ws://127.0.0.1:9944"))
            .build()
            .await?
            .to_runtime_api::<fennel::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();
            //Maybe we shouldn't couple
            //database functionality here
            //let identity_db = get_identity_database_handle();
            //let messages_db = get_message_database_handle();

        Ok(Self {
            runtime,
            //identity_db,
            //messages_db,
        })
    }

    /// submit an identity to the network
    // pub async fn add_or_update(&self, id: RawIdentity, signer: Pair) -> Result<(), Error> {
    //     let signer = PairSigner::<DefaultConfig, DefaultExtra<DefaultConfig>, _>::new(signer);

    //     // NOTE: identity module should probably be snake case, or just named `identity`
    //     // api
    //     Ok(())
    // }

    /// Submit a new identity to the Fennel network.
    //pub async fn create_identity(&self, pair: Pair) -> Result<fennel::identity_module::events::IdentityCreated, Error> {
        pub async fn create_identity(&self, pair: Pair) -> Result<u32, Error> {
       
        println!("Submit a new identity to the Fennel network.");
        env_logger::init();

        let signer = PairSigner::<DefaultConfig, DefaultExtra<DefaultConfig>, _>::new(pair);

        let identity = self
            .runtime
            .tx()
            .identity_module()
            .create_identity()
            .sign_and_submit_then_watch(&signer)
            .await?
            .wait_for_finalized_success()
            .await
            .unwrap();

        let identity_event =
            identity.find_first_event::<fennel::identity_module::events::IdentityCreated>()?;

        if let Some(event) = identity_event {
            match event {
                fennel::identity_module::events::IdentityCreated(a, b) => {
                    println!("{}", a);
                    return Ok(a);
                }
            }
            //println!("Identity Create success: {event:?}");
        } else {
            println!("Failed to find identity_module::Transfer Event");
        }

        Ok(0)

        
    }

    /// Retrieve the full list of identities from Fennel Protocol.
    pub async fn fetch_identities(&self) -> Result<(), Box<dyn std::error::Error>> {
        env_logger::init();

        let api = ClientBuilder::new()
            .build()
            .await?
            .to_runtime_api::<fennel::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

        let mut iter = api
            .storage()
            .identity_module()
            .identity_list_iter(None)
            .await?;

        while let Some((key, identity)) = iter.next().await? {
            println!("{}: {:?}", hex::encode(key), identity);
        }
        Ok(())
    }

    /// Upload a public key to Fennel Protocol.
    pub async fn announce_public_key(
        &self,
        signer: Pair,
        fingerprint: Vec<u8>,
        location: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        env_logger::init();
        let signer = PairSigner::<DefaultConfig, DefaultExtra<DefaultConfig>, _>::new(signer);

        let identity = self
            .runtime
            .tx()
            .keystore_module()
            .announce_key(fingerprint, location)
            .sign_and_submit_then_watch(&signer)
            .await?
            .wait_for_finalized_success()
            // FIXME: Should be in error enum with GenericError
            .await
            .unwrap();

        let identity_event =
            identity.find_first_event::<fennel::identity_module::events::IdentityCreated>()?;

        if let Some(event) = identity_event {
            println!("Identity Create success: {event:?}");
        } else {
            println!("Failed to find identity_module::Transfer Event");
        }
        Ok(())
    }

    /// Retrieve all known public keys from Fennel Protocol.
    pub async fn fetch_public_keys() -> Result<(), Box<dyn std::error::Error>> {
        env_logger::init();

        let api = ClientBuilder::new()
            .build()
            .await?
            .to_runtime_api::<fennel::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

        let mut iter = api
            .storage()
            .keystore_module()
            .issued_keys_iter(None)
            .await?;

        while let Some((key, public_key)) = iter.next().await? {
            println!("{}: {:?}", hex::encode(key), public_key);
        }
        Ok(())
    }
}
