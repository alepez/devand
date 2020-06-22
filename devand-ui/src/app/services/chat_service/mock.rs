use super::{NewMessagesCallback, OtherUserLoadedCallback};
use chrono::offset::TimeZone;
use devand_core::chat::ChatMessage;
use devand_core::UserId;
use fake::faker::lorem::en::*;
use fake::Fake;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub struct ChatService {
    chat_members: Option<Vec<UserId>>,
    new_messages_callback: NewMessagesCallback,
    other_user_loaded_callback: OtherUserLoadedCallback,

    rng: StdRng,
}

impl ChatService {
    pub fn new(
        new_messages_callback: NewMessagesCallback,
        other_user_loaded_callback: OtherUserLoadedCallback,
    ) -> Self {
        let rng = StdRng::seed_from_u64(42);

        Self {
            chat_members: None,
            new_messages_callback,
            other_user_loaded_callback,
            rng,
        }
    }

    pub fn load_other_user(&mut self, username: &str) {
        self.other_user_loaded_callback
            .emit(Some(devand_core::PublicUserProfile {
                id: UserId(2),
                languages: devand_core::Languages::default(),
                username: username.into(),
                visible_name: "Foo Bar".into(),
            }))
    }

    pub fn load_history(&mut self, mut chat_members: Vec<UserId>) {
        chat_members.sort();
        self.chat_members = Some(chat_members.clone());
        self.new_messages_callback.emit(fake_messages(
            &mut self.rng,
            10,
            chat_members[0],
            chat_members[1],
        ))
    }

    pub fn send_message(&mut self, txt: String) {
        let t: i64 = 1592475298;
        self.new_messages_callback.emit(vec![ChatMessage {
            created_at: chrono::Utc.timestamp(t, 0),
            author: UserId(1),
            txt,
        }])
    }

    pub fn poll(&mut self, _last_message: Option<&ChatMessage>) {
        log::debug!("poll {:?}", &self.chat_members);
        if let Some(_chat_members) = &self.chat_members {
            self.new_messages_callback
                .emit(vec![fake_message(&mut self.rng, UserId(2))])
        }
    }
}

fn fake_messages(rng: &mut StdRng, n: usize, me: UserId, other: UserId) -> Vec<ChatMessage> {
    let mut history = Vec::new();
    let mut t: i64 = 1592475298;

    for _ in 0..n {
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

fn fake_message(rng: &mut StdRng, author: UserId) -> ChatMessage {
    let t: i64 = 1592475298;

    ChatMessage {
        created_at: chrono::Utc.timestamp(t, 0),
        author,
        txt: Sentence(1..30).fake_with_rng(rng),
    }
}
