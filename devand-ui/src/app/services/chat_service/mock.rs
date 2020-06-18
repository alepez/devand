use super::NewMessagesCallback;
use chrono::offset::TimeZone;
use devand_core::chat::ChatMessage;
use devand_core::UserId;

pub struct ChatService {
    chat_members: Vec<UserId>,
    new_messages_callback: NewMessagesCallback,
}

impl ChatService {
    pub fn new(chat_members: Vec<UserId>, new_messages_callback: NewMessagesCallback) -> Self {
        Self {
            chat_members,
            new_messages_callback,
        }
    }

    pub fn load_history(&mut self) {
        self.new_messages_callback
            .emit(mock_history(self.chat_members[0], self.chat_members[1]))
    }

    pub fn send_message(&mut self, txt: String) {}
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
        let from_me: bool = rng.gen();
        t += t_diff;

        history.push(ChatMessage {
            created_at: chrono::Utc.timestamp(t, 0),
            author: if from_me { me } else { other },
            txt: Sentence(1..30).fake_with_rng(rng),
        });
    }

    history
}
