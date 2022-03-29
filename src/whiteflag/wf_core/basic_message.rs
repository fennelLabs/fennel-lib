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
}
