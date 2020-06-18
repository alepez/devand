use super::NewMessagesCallback;
use devand_core::chat::ChatMessage;

pub struct ChatService {
    new_messages_callback: NewMessagesCallback,
}

impl ChatService {
    pub fn new(new_messages_callback: NewMessagesCallback) -> Self {
        Self {
            new_messages_callback,
        }
    }

    pub fn load_old_messages(&mut self) {
        self.new_messages_callback.emit(mock_history())
    }
}

fn mock_history() -> Vec<ChatMessage> {
    Vec::default()
}
