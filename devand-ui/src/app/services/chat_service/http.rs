use super::NewMessagesCallback;
use devand_core::chat::{ChatId, ChatMessage};
use devand_core::UserId;

pub struct ChatService {
    chat_id: ChatId,
    new_messages_callback: NewMessagesCallback,
}

impl ChatService {
    pub fn new(chat_id: ChatId, new_messages_callback: NewMessagesCallback) -> Self {
        Self {
            chat_id,
            new_messages_callback,
        }
    }

    pub fn load_old_messages(&mut self) {
        self.new_messages_callback
            .emit(mock_history(self.chat_id.user_me, self.chat_id.user_other))
    }
}

fn mock_history(_me: UserId, _other: UserId) -> Vec<ChatMessage> {
    Vec::default()
}
