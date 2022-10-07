use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub async fn add_file(file_content: &str) -> String {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-put
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("data", file_content);

    let res = client.post("http://127.0.0.1:5001/api/v0/block/put?cid-codec=raw&mhtype=sha2-256&mhlen=-1&pin=false&allow-big-block=false")
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    res
}

pub async fn get_file(cid: &str) -> String {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-get
    let client = reqwest::Client::new();

    let res = client
        .post(format!("http://127.0.0.1:5001/api/v0/block/get?arg={}", cid).as_str())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    res
}

pub async fn del_file(cid: &str) -> bool {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-rm
    let client = reqwest::Client::new();

    let res = client
        .post(format!("http://127.0.0.1:5001/api/v0/block/rm?arg={}", cid).as_str())
        .send()
        .await
        .unwrap()
        .status();
    res.is_success()
}
