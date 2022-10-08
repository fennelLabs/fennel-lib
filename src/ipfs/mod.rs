#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{Error, Write};

pub fn add_content_by_string(file_content: &str) -> Result<String, Error> {
    let mut output = File::create("upload.txt")?;
    write!(output, "{}", file_content)?;
    Ok(add_content_by_local_path("upload.txt").unwrap())
}

pub fn add_content_by_local_path(filename: &str) -> Result<String, reqwest::Error> {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-put
    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new()
        .file("data", filename)
        .unwrap();

    let res = client.post("http://127.0.0.1:5001/api/v0/block/put?cid-codec=raw&mhtype=sha2-256&mhlen=-1&pin=false&allow-big-block=false")
        .multipart(form)
        .send()?
        .text()?;
    Ok(res)
}

pub fn get_content_by_cid(cid: &str) -> Result<String, reqwest::Error> {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-get
    let client = reqwest::blocking::Client::new();

    let res = client
        .post(format!("http://127.0.0.1:5001/api/v0/block/get?arg={}", cid).as_str())
        .send()?
        .text()?;
    Ok(res)
}

pub fn delete_content_by_cid(cid: &str) -> Result<bool, reqwest::Error> {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-rm
    let client = reqwest::blocking::Client::new();

    let res = client
        .post(format!("http://127.0.0.1:5001/api/v0/block/rm?arg={}", cid).as_str())
        .send()?
        .status();
    Ok(res.is_success())
}
