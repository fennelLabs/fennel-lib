use super::{segment::MessageSegment, types::MessageType};

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
        let (mut header_buffer, mut header_len) = self.header.encode();
        let (mut body_buffer, body_len) = self.body.encode();

        header_buffer.append(&mut body_buffer);
        header_len += body_len;

        (header_buffer, header_len)
    }
}
