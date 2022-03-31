use super::wf_codec::common::{crop_bits, to_hex};
use super::creator::compile;

#[test]
fn text_decode_hex_message() {
    let encoding_result: String = "57463130a6a1f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d7401009841882148a800000114c1e596006f04c050eca6420084".to_string();
    let field_values = vec![
        "WF",
        "1",
        "0",
        "1",
        "M",
        "4",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "80",
        "2013-08-31T04:29:15Z",
        "P00D00H00M",
        "22",
        "+30.79658",
        "-037.82602",
        "8765",
        "3210",
        "042",
    ];

    let basic_message = compile(field_values);
    let (message_encoded, len) = basic_message.encode();

    assert_eq!(
        encoding_result,
        to_hex(&crop_bits(message_encoded, len as isize)),
        "Encoding should be correct"
    );
}