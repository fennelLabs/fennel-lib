mod error;
#[cfg(test)]
mod tests;

use crate::rsa_tools::hash;
use codec::{Decode, Encode};
use parking_lot::Mutex;
use rocksdb::{ColumnFamilyDescriptor, DBPath, IteratorMode, Options, DB};
use std::{fs, path::Path, sync::Arc};

use error::Error;

#[derive(Encode, Decode, Debug)]
pub struct Identity {
    pub id: [u8; 4],
    pub fingerprint: [u8; 16],
    pub public_key: [u8; 1038],
}

#[derive(Encode, Decode, Debug)]
pub struct Message {
    pub sender_id: [u8; 4],
    pub fingerprint: [u8; 16],
    pub message: [u8; 1024],
    pub signature: [u8; 1024],
    pub public_key: [u8; 1038],
    pub recipient_id: [u8; 4],
}

pub(crate) const NUM_COLUMNS: u32 = 2;
pub(crate) mod columns {
    pub const MESSAGES: u32 = 0;
    pub const IDENTITIES: u32 = 1;
}

#[derive(Clone)]
pub struct FennelLocalDb {
    db: Arc<Mutex<DB>>,
}

impl FennelLocalDb {
    pub fn new() -> Result<Self, Error> {
        // TODO: Accept a path in the future
        let path = fs::create_dir_all("./_fennel_db")?;
        let path = Path::new("./_fennel_db");

        let opts = Options::default();
        opts.create_if_missing(true);

        let column_names: Vec<_> = (0..NUM_COLUMNS)
            .map(|c| format!("col{}", c).as_str())
            .collect();
        let db = Self::open(&opts, path, column_names.as_slice())?;

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }

    /// Internal api to open a database.
    fn open<P: AsRef<Path>>(
        opts: &Options,
        path: P,
        column_names: &[&str],
    ) -> Result<rocksdb::DB, Error> {
        // NOTE: Options can be optimized
        let cf_descriptors: Vec<_> = (0..NUM_COLUMNS)
            .map(|i| ColumnFamilyDescriptor::new(column_names[i as usize], *opts))
            .collect();

        let db = match DB::open_cf_descriptors(&opts, path.as_ref(), cf_descriptors) {
            Err(_) => {
                // retry and create CFs
                match DB::open_cf(&opts, path.as_ref(), &[] as &[&str]) {
                    Ok(mut db) => {
                        for (i, name) in column_names.iter().enumerate() {
                            let _ = db.create_cf(name, &opts)?;
                        }
                        Ok(db)
                    }
                    err => err,
                }
            }
            ok => ok,
        };
        Ok(db?)
    }

    fn insert_message(&self, message: Message) -> Result<(), Error> {
        let db = self.db.lock();
        let message_bytes = message.encode();
        let m: Vec<u8> = message
            .recipient_id
            .iter()
            .cloned()
            .chain(hash(&message_bytes))
            .collect();
        let cf = db
            .cf_handle(&cf(&columns::MESSAGES))
            .ok_or(Error::CfHandle(columns::IDENTITIES))?;
        db.put_cf(&cf, m, message_bytes)?;
        Ok(())
    }

    pub fn insert_message_list(&self, messages_list: Vec<Message>) -> Result<(), Error> {
        let db = self.db.lock();
        for message in messages_list {
            self.insert_message(message)?;
        }
        Ok(())
    }
    /// Retrieve all messages for id. This is INCREDIBLY inefficient. We'll need to retool this.
    pub fn retrieve_messages(&self, identity: Identity) -> Result<Vec<Message>, Error> {
        let db = self.db.lock();
        let mut message_list: Vec<Message> = Vec::new();
        let cf = db
            .cf_handle(&cf(&columns::MESSAGES))
            .ok_or(Error::CfHandle(columns::IDENTITIES))?;
        for (key, value) in db.iterator_cf(&cf, IteratorMode::Start) {
            if key[0..4] == identity.id {
                message_list.push(Decode::decode(&mut &(*value))?);
            }
        }
        Ok(message_list)
    }

    pub fn insert_identity(&self, identity: &Identity) -> Result<(), Error> {
        let db = self.db.lock();
        let cf = db
            .cf_handle(&cf(&columns::IDENTITIES))
            .ok_or(Error::CfHandle(columns::IDENTITIES))?;
        db.put_cf(&cf, identity.id, identity.encode())?;
        Ok(())
    }

    pub fn retrieve_identity(&self, id: [u8; 4]) -> Result<Identity, Error> {
        let db = self.db.lock();
        let cf = db
            .cf_handle(&cf(&columns::IDENTITIES))
            .ok_or(Error::CfHandle(columns::IDENTITIES))?;
        let value = db.get_cf(&cf, id)?.expect("Not present in database");
        Decode::decode(&mut &*value).map_err(Into::into)
    }
}

fn cf(col: &u32) -> String {
    format!("cf{}", col)
}
