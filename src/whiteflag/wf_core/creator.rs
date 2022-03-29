//use super::definitions::message_code;
use super::segment::MessageSegment;
use super::types::MessageType;
/* public final WfBasicMessage create() {
    return new WfBasicMessage(messageType, header, body);
} */

pub const PREFIX: &str = "WF";
pub const PROTOCOL_VERSION: &str = "1";
pub const FIELD_PREFIX: &str = "Prefix";
pub const FIELD_VERSION: &str = "Version";
pub const FIELD_MESSAGETYPE: &str = "MessageCode";
pub const FIELD_TESTMESSAGETYPE: &str = "PseudoMessageCode";

pub fn compile(data: Vec<String>) {
    let mut header: MessageSegment = MessageSegment::generic_header_segment();
    header.set_all(data, 0);

    let message_code = match &header.get(&FIELD_MESSAGETYPE) {
        Some(x) => x.chars().next(),
        _ => None,
    };

    let message_type = MessageType::from_code_option(message_code.as_ref());
}
