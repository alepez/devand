use super::{MainWorker, Request, Response};
use devand_core::*;
use fake::faker::internet::raw::*;
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
            link.send_message(Response::PublicUserProfileFetched(Box::new(
                fake_public_profile(),
            )));
        }

        Request::LoadPublicUserProfileByUsername(username) => {
            link.send_message(Response::PublicUserProfileFetched(Box::new(
                fake_public_profile(),
            )));
        }

        Request::LoadAffinities => {
            link.send_message(Response::AffinitiesFetched(fake_affinities()));
        }

        Request::LoadAvailabilityMatch => {
            link.send_message(Response::AvailabilityMatchFetched(Box::new(fake_matches())));
        }

        Request::CheckOldPassword(old_password) => {
            link.send_message(Response::OldPasswordChecked(true));
        }

        Request::EditPassword(old_password, new_password) => {
            link.send_message(Response::PasswordEdited(()));
        }

        Request::ChatSendMessage(members, txt) => {
            link.send_message(Response::ChatNewMessagesLoaded(fake_messages()));
        }

        Request::ChatPoll(members, from_created_at) => {
            link.send_message(Response::ChatNewMessagesLoaded(fake_messages()));
        }

        Request::ChatLoadHistory(members) => {
            link.send_message(Response::ChatHistoryLoaded(fake_chat_info()));
        }

        Request::LoadAllChats => {
            link.send_message(Response::AllChatsLoaded(fake_chats()));
        }

        // Program should never hit this
        Request::Lazy(_) => unimplemented!(),
    }
}

fn fake_user(rng: &mut StdRng) -> devand_core::User {
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
        settings: devand_core::UserSettings {
            languages: Languages(languages),
            schedule: Availability::default(),
            vacation_mode: false,
            spoken_languages: SpokenLanguages(btreeset![devand_core::SpokenLanguage::English]),
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
        all_users.push(fake_user(rng).into());
    }

    CodeNow {
        current_user,
        all_users,
    }
}

fn fake_public_profile() -> devand_core::PublicUserProfile {
    todo!()
}

fn fake_affinities() -> Vec<devand_core::UserAffinity> {
    todo!()
}

fn fake_matches() -> devand_core::schedule_matcher::AvailabilityMatch {
    todo!()
}

fn fake_messages() -> Vec<devand_core::chat::ChatMessage> {
    todo!()
}

fn fake_chat_info() -> devand_core::chat::ChatInfo {
    todo!()
}

fn fake_chats() -> devand_core::UserChats {
    todo!()
}
