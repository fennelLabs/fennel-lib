#[cfg(test)]
mod tests;

use curl::easy::Easy;
use std::{env, io::Read};

pub fn add_file(file_content: &str) {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-put
    let mut data = file_content.as_bytes();

    let mut easy = Easy::new();
    let url_string = format!("http://{}:5001/api/v0/block/put?cid-codec=raw&mhtype=sha2-256&mhlen=-1&pin=false&allow-big-block=false", env::var("IPFS_HOST").unwrap());
    easy.url(&url_string).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
        .unwrap();
    transfer
        .write_function(|buf| {
            println!("{}", String::from_utf8(Vec::from(buf)).unwrap());
            Ok(buf.len())
        })
        .unwrap();
    transfer.perform().unwrap();
}

pub fn get_file(cid: &str) {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-get
    let mut easy = Easy::new();
    easy.url(
        format!(
            "http://{}:5001/api/v0/block/get?arg={}",
            env::var("IPFS_HOST").unwrap(),
            cid
        )
        .as_str(),
    )
    .unwrap();
    easy.post(true).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .write_function(|buf| {
            println!("{}", String::from_utf8(Vec::from(buf)).unwrap());
            Ok(buf.len())
        })
        .unwrap();
    transfer.perform().unwrap();
}

pub fn del_file(cid: &str) {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-rm
    let mut easy = Easy::new();
    easy.url(
        format!(
            "http://{}:5001/api/v0/block/rm?arg={}",
            env::var("IPFS_HOST").unwrap(),
            cid
        )
        .as_str(),
    )
    .unwrap();
    easy.post(true).unwrap();

    let transfer = easy.transfer();
    transfer.perform().unwrap();
}
