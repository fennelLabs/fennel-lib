#[cfg(test)]
mod tests;

use curl::easy::Easy;
use std::fs;
use std::io::Read;

pub fn add_file(filename: &str) {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-put
    let file_content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut data = file_content.as_bytes();

    let mut easy = Easy::new();
    easy.url("http://127.0.0.1:5001/api/v0/block/put?cid-codec=raw&mhtype=sha2-256&mhlen=-1&pin=false&allow-big-block=false").unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
        .unwrap();
    transfer.perform().unwrap();
}

pub fn get_file(cid: &str) {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-get
    let mut easy = Easy::new();
    easy.url(format!("http://127.0.0.1:5001/api/v0/block/get?arg={}", cid).as_str())
        .unwrap();
    easy.post(true).unwrap();

    let transfer = easy.transfer();
    transfer.perform().unwrap();
}

pub fn del_file(cid: &str) {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-rm
    let mut easy = Easy::new();
    easy.url(format!("http://127.0.0.1:5001/api/v0/block/rm?arg={}", cid).as_str())
        .unwrap();
    easy.post(true).unwrap();

    let transfer = easy.transfer();
    transfer.perform().unwrap();
}
