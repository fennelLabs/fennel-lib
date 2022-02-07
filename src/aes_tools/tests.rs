use crate::aes_tools::*;

#[test]
fn test_key_gen() {
    generate_key();
}

#[test]
fn test_encrypt() {
    let message = "
    {
        \"id\": 1,
        \"name\": \"xyzab\"
    }"
    .to_string();

    encrypt(message);

    ()
}
