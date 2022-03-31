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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = self.header.encode();
        buffer.append(&mut self.body.encode());
        buffer
    }
}
