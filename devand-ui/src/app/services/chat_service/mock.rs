use super::NewMessagesCallback;
use chrono::offset::TimeZone;
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

    pub fn load_history(&mut self) {
        self.new_messages_callback
            .emit(mock_history(self.chat_id.user_me, self.chat_id.user_other))
    }
}

fn mock_history(me: UserId, other: UserId) -> Vec<ChatMessage> {
    use fake::faker::lorem::en::*;
    use fake::Fake;
    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;

    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let ref mut rng = StdRng::from_seed(seed);

    let mut history = Vec::new();
    let mut t: i64 = 1592475298;

    for _ in 0..10 {
        let t_diff: i64 = rng.gen_range(0, 5000);
        let from_me : bool = rng.gen();
        t += t_diff;

        history.push(ChatMessage {
            created_at: chrono::Utc.timestamp(t, 0),
            from: if from_me { me } else { other },
            to: if from_me { other } else { me },
            txt: Sentence(1..30).fake_with_rng(rng),
        });
    }

    history
}
