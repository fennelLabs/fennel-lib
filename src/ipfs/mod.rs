#[cfg(test)]
mod tests;

pub fn add_file(filename: &str) -> String {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-put
    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new()
        .file("data", filename)
        .unwrap();

    let res = client.post("http://127.0.0.1:5001/api/v0/block/put?cid-codec=raw&mhtype=sha2-256&mhlen=-1&pin=false&allow-big-block=false")
        .multipart(form)
        .send()
        .unwrap()
        .text()
        .unwrap();
    res
}

pub fn get_file(cid: &str) -> String {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-get
    let client = reqwest::blocking::Client::new();

    let res = client
        .post(format!("http://127.0.0.1:5001/api/v0/block/get?arg={}", cid).as_str())
        .send()
        .unwrap()
        .text()
        .unwrap();
    res
}

pub fn del_file(cid: &str) -> bool {
    // https://docs.ipfs.io/reference/http/api/#api-v0-block-rm
    let client = reqwest::blocking::Client::new();

    let res = client
        .post(format!("http://127.0.0.1:5001/api/v0/block/rm?arg={}", cid).as_str())
        .send()
        .unwrap()
        .status();
    res.is_success()
}
