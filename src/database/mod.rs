#[cfg(test)]
mod tests;

use crate::rsa_tools::hash;
use codec::{Decode, Encode};
use rocksdb::Error;
use rocksdb::IteratorMode;
use rocksdb::DB;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Encode, Decode, Debug)]
pub struct Identity {
    pub id: [u8; 4],
    pub fingerprint: [u8; 16],
    pub public_key: [u8; 526],
    pub shared_secret_key: [u8; 32],
}

#[derive(Encode, Decode, Debug)]
pub struct Message {
    pub sender_id: [u8; 4],
    pub fingerprint: [u8; 16],
    pub message: [u8; 512],
    pub signature: [u8; 512],
    pub public_key: [u8; 526],
    pub recipient_id: [u8; 4],
    pub message_type: [u8; 1],
}

/// Grab a mutex-wrapped message database handle.
pub fn get_message_database_handle() -> Arc<Mutex<DB>> {
    Arc::new(Mutex::new(DB::open_default("./message.db").unwrap()))
}

/// Grab a mutex-wrapped database identity handle.
pub fn get_identity_database_handle() -> Arc<Mutex<DB>> {
    Arc::new(Mutex::new(DB::open_default("./identity.db").unwrap()))
}

/// Submit a single message. Used to send a single message.
pub fn insert_message(db_lock: Arc<Mutex<DB>>, message: Message) -> Result<(), Error> {
    let db = db_lock.lock().unwrap();
    let message_bytes = message.encode();
    let m: Vec<u8> = message
        .recipient_id
        .iter()
        .cloned()
        .chain(hash(&message_bytes))
        .collect();
    db.put(m, message_bytes).unwrap();
    Ok(())
}

/// Insert a full list of retrieved messages. Used to download remote messages.
pub fn insert_message_list(
    messages_db: Arc<Mutex<DB>>,
    messages_list: Vec<Message>,
) -> Result<(), Error> {
    for message in messages_list {
        let messages_db_clone = Arc::clone(&messages_db);
        insert_message(messages_db_clone, message).unwrap();
    }
    Ok(())
}

/// Retrieve all messages for id. This is INCREDIBLY inefficient. We'll need to retool this.
pub fn retrieve_messages(db_lock: Arc<Mutex<DB>>, identity: Identity) -> Vec<Message> {
    let db = db_lock.lock().unwrap();
    let mut message_list: Vec<Message> = Vec::new();
    for (key, value) in db.iterator(IteratorMode::Start) {
        if key[0..4] == identity.id {
            message_list.push(Decode::decode(&mut &(*value)).unwrap());
        }
    }
    message_list
}

/// Commit a new identity to the local database.
pub fn insert_identity(db_lock: Arc<Mutex<DB>>, identity: &Identity) -> Result<(), Error> {
    let db = db_lock.lock().unwrap();
    db.put::<_, Vec<_>>(identity.id, identity.encode()).unwrap();
    Ok(())
}

/// Retrieve an identity from the local database by id array.
pub fn retrieve_identity(db_lock: Arc<Mutex<DB>>, id: [u8; 4]) -> Identity {
    let db = db_lock.lock().unwrap();
    let return_value = db.get(id).expect("failed to retrieve identity");
    let value = return_value.unwrap();
    Decode::decode(&mut &*value).unwrap()
}
