use super::{segment::MessageSegment, types::MessageType};
use super::wf_codec::common::{crop_bits, append_bits};

pub struct BasicMessage {
    message_type: MessageType,
    header: MessageSegment,
    body: MessageSegment,
}

impl BasicMessage {
    pub fn new(
        message_type: MessageType,
        header: MessageSegment,
        body: MessageSegment,
    ) -> BasicMessage {
        BasicMessage {
            message_type,
            header,
            body,
        }
    }

    pub fn encode(&self) -> (Vec<u8>, usize) {
        let mut buffer: Vec<u8> = vec![];
        let mut len = 0;

        let (header_buffer, header_len) = self.header.encode();
        let (body_buffer, body_len) = self.body.encode();

        (buffer, len) = append_bits(&buffer, len, &header_buffer, header_len);
        (buffer, len) = append_bits(&buffer, len, &body_buffer, body_len);

        (buffer, len)
    }
}
