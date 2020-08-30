use super::{MainWorker, Request, Response};
use chrono::offset::TimeZone;
use devand_core::*;
use fake::faker::internet::raw::*;
use fake::faker::lorem::en::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use maplit::btreeset;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::Rng;
use rand::SeedableRng;
use strum::IntoEnumIterator;

pub fn request(worker: &mut MainWorker, msg: Request) {
    log::info!("Request: {:?}", msg);

    let mut rng = StdRng::seed_from_u64(100);

    let link = worker.link.clone();

    match msg {
        Request::Init => {
            link.send_message(Response::SelfUserFetched(Box::new(fake_user(&mut rng))));
        }

        Request::SaveSelfUser(user) => {
            link.send_message(Response::SelfUserFetched(user));
        }

        Request::VerifyEmail => {
            link.send_message(Response::Done(()));
        }

        Request::LoadCodeNow => {
            link.send_message(Response::CodeNowFetched(Box::new(fake_code_now(&mut rng))));
        }

        Request::LoadPublicUserProfile(user_id) => {
            let mut rng = StdRng::seed_from_u64(user_id.0 as u64);
            let mut u = fake_public_profile(&mut rng);
            u.id = user_id;
            link.send_message(Response::PublicUserProfileFetched(Box::new(u)));
        }

        Request::LoadPublicUserProfileByUsername(username) => {
            let mut u = fake_public_profile(&mut rng);
            u.username = username.clone();
            link.send_message(Response::PublicUserProfileFetched(Box::new(u)));
        }

        Request::LoadAffinities => {
            link.send_message(Response::AffinitiesFetched(fake_affinities(&mut rng)));
        }

        Request::LoadAvailabilityMatch => {
            link.send_message(Response::AvailabilityMatchFetched(Box::new(fake_matches(
                &mut rng,
            ))));
        }

        Request::CheckOldPassword(_old_password) => {
            link.send_message(Response::OldPasswordChecked(true));
        }

        Request::EditPassword(_old_password, _new_password) => {
            link.send_message(Response::PasswordEdited(()));
        }

        Request::ChatSendMessage(members, txt) => {
            let t: i64 = 1592475298;
            let new_message = chat::ChatMessage {
                id: fake_uuid(&mut rng),
                created_at: chrono::Utc.timestamp(t, 0),
                author: members[0],
                txt,
            };
            link.send_message(Response::ChatNewMessagesLoaded(vec![new_message]));
        }

        Request::ChatPoll(members, from_created_at) => {
            let t = 1 + from_created_at.map(|x| x.timestamp()).unwrap_or(1592475298);
            let seed = t as u64;
            let mut rng = StdRng::seed_from_u64(seed);
            let author = members[1];

            let msg = chat::ChatMessage {
                id: fake_uuid(&mut rng),
                created_at: chrono::Utc.timestamp(t, 0),
                author,
                txt: Sentence(1..30).fake_with_rng(&mut rng),
            };

            link.send_message(Response::ChatNewMessagesLoaded(vec![msg]));
        }

        Request::ChatLoadHistory(_members) => {
            link.send_message(Response::ChatHistoryLoaded(fake_chat_info(&mut rng)));
        }

        Request::LoadAllChats => {
            link.send_message(Response::AllChatsLoaded(fake_chats(&mut rng)));
        }

        // Program should never hit this
        Request::Lazy(_) => unimplemented!(),
    }
}

fn fake_user(rng: &mut StdRng) -> User {
    let name: String = Name(EN).fake_with_rng(rng);
    let user_id: i32 = rng.gen_range(1, 1_000_000_000);

    let mut languages = std::collections::BTreeMap::default();

    for lang in Language::iter() {
        if rng.gen_bool(0.2) {
            let level = Level::iter().choose(rng).unwrap();
            let priority = Priority::iter().choose(rng).unwrap();
            languages.insert(lang, LanguagePreference { level, priority });
        }
    }

    let email: String = SafeEmail(EN).fake_with_rng(rng);
    let email_verified = rng.gen_bool(0.7);

    User {
        id: UserId(user_id),
        username: name
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect(),
        email,
        email_verified,
        visible_name: name,
        settings: UserSettings {
            languages: Languages(languages),
            schedule: Availability::default(),
            vacation_mode: false,
            spoken_languages: SpokenLanguages(btreeset![SpokenLanguage::English]),
        },
        unread_messages: 5,
        bio: "This is the bio".to_string(),
    }
}

fn fake_code_now(rng: &mut StdRng) -> CodeNow {
    let current_user = fake_user(rng);

    let mut all_users = Vec::new();
    let n = rng.gen_range(0, 20);

    for _ in 0..n {
        all_users.push(fake_public_profile(rng));
    }

    CodeNow {
        current_user,
        all_users,
    }
}

fn fake_public_profiles(rng: &mut StdRng) -> Vec<PublicUserProfile> {
    let n = rng.gen_range(0, 20);
    let mut v = Vec::default();
    for _ in 0..n {
        v.push(fake_public_profile(rng));
    }
    v
}

fn fake_public_profile(rng: &mut StdRng) -> PublicUserProfile {
    let user = fake_user(rng);
    user.into()
}

fn fake_affinities(rng: &mut StdRng) -> Vec<UserAffinity> {
    let n = rng.gen_range(1, 10);
    let mut v = Vec::default();
    for _ in 0..n {
        v.push(UserAffinity {
            user: fake_public_profile(rng),
            affinity: Affinity::from_number(rng.gen_range(0.0, 1.0)),
        })
    }
    v
}

fn fake_matches(rng: &mut StdRng) -> schedule_matcher::AvailabilityMatch {
    let n = rng.gen_range(1, 10);
    let mut slots = Vec::default();
    let t0: i64 = 1598810400;
    for _ in 0..n {
        let t = t0 + (3600 * rng.gen_range(1, 24 * 7));
        let d = chrono::Utc.timestamp(t, 0);
        let users_count = rng.gen_range(1, 10);
        let mut users = Vec::default();
        for _ in 0..users_count {
            users.push(UserId(rng.gen_range(1, 1000)));
        }
        slots.push((d, users))
    }
    schedule_matcher::AvailabilityMatch { slots }
}

fn fake_message(rng: &mut StdRng, author: UserId) -> chat::ChatMessage {
    let t: i64 = 1592475298;

    chat::ChatMessage {
        id: fake_uuid(rng),
        created_at: chrono::Utc.timestamp(t, 0),
        author,
        txt: Sentence(1..30).fake_with_rng(rng),
    }
}

fn fake_messages(rng: &mut StdRng, n: usize, me: UserId, other: UserId) -> Vec<chat::ChatMessage> {
    let mut history = Vec::new();
    let mut t: i64 = 1592475298;

    for _ in 0..n {
        let t_diff: i64 = rng.gen_range(0, 5000);
        let from_me: bool = rng.gen();
        t += t_diff;

        history.push(chat::ChatMessage {
            id: fake_uuid(rng),
            created_at: chrono::Utc.timestamp(t, 0),
            author: if from_me { me } else { other },
            txt: Sentence(1..30).fake_with_rng(rng),
        });
    }

    history
}

fn fake_chat_info(rng: &mut StdRng) -> chat::ChatInfo {
    let me = fake_user(rng);
    let other = fake_user(rng);

    let members = vec![me.clone(), other.clone()];

    let members_info = members
        .iter()
        .map(|user| chat::ChatMemberInfo {
            user_id: user.id,
            verified_email: user.email_verified,
        })
        .collect();

    let messages = fake_messages(rng, 10, me.id, other.id);

    chat::ChatInfo {
        members_info,
        messages,
    }
}

fn fake_chats(rng: &mut StdRng) -> UserChats {
    let n = rng.gen_range(1, 10);
    let mut v = Vec::default();
    for _ in 0..n {
        let members: Vec<PublicUserProfile> =
            fake_public_profiles(rng).into_iter().take(1).collect();
        v.push(UserChat {
            chat: chat::Chat {
                id: chat::ChatId(fake_uuid(rng)),
                members: members.iter().map(|x| x.id).collect(),
            },
            unread_messages: rng.gen_range(0, 100),
            members,
        });
    }
    UserChats(v)
}

fn fake_uuid(rng: &mut StdRng) -> uuid::Uuid {
    let bytes: [u8; 16] = rng.gen();
    uuid::Uuid::from_bytes(&bytes).unwrap()
}
