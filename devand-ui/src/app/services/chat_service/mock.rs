use super::{ChatServiceCallback, ChatServiceContent};
use chrono::offset::TimeZone;
use devand_core::chat::ChatMessage;
use devand_core::{UserChats, UserId};
use fake::faker::lorem::en::*;
use fake::Fake;
use maplit::btreeset;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub struct ChatService {
    chat_members: Option<Vec<UserId>>,
    callback: ChatServiceCallback,

    rng: StdRng,
}

impl ChatService {
    pub fn new(callback: ChatServiceCallback) -> Self {
        let rng = StdRng::seed_from_u64(42);

        Self {
            chat_members: None,
            callback,
            rng,
        }
    }

    pub fn load_all_chats(&mut self) {
        self.callback
            .emit(ChatServiceContent::AllChats(fake_chats(&mut self.rng)));
    }
}

fn fake_uuid(rng: &mut StdRng) -> uuid::Uuid {
    let bytes: [u8; 16] = rng.gen();
    uuid::Uuid::from_bytes(&bytes).unwrap()
}

fn fake_messages(rng: &mut StdRng, n: usize, me: UserId, other: UserId) -> Vec<ChatMessage> {
    let mut history = Vec::new();
    let mut t: i64 = 1592475298;

    for _ in 0..n {
        let t_diff: i64 = rng.gen_range(0, 5000);
        let from_me: bool = rng.gen();
        t += t_diff;

        history.push(ChatMessage {
            id: fake_uuid(rng),
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
        id: fake_uuid(rng),
        created_at: chrono::Utc.timestamp(t, 0),
        author,
        txt: Sentence(1..30).fake_with_rng(rng),
    }
}

fn fake_chats(rng: &mut StdRng) -> UserChats {
    use devand_core::chat::{Chat, ChatId};
    use devand_core::{PublicUserProfile, UserChat};

    UserChats(vec![UserChat {
        chat: Chat {
            id: ChatId(fake_uuid(rng)),
            members: vec![UserId(rng.gen()), UserId(rng.gen())],
        },
        unread_messages: rng.gen_range(1, 100),
        members: vec![PublicUserProfile {
            id: UserId(2),
            languages: devand_core::Languages::default(),
            username: "foobar".into(),
            visible_name: "Foo Bar".into(),
            bio: "This is the bio".to_string(),
            spoken_languages: devand_core::SpokenLanguages(btreeset![
                devand_core::SpokenLanguage::English
            ]),
        }],
    }])
}
