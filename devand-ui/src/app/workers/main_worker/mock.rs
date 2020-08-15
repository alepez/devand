use super::{MainWorker, Request, Response};

use maplit::{btreemap, btreeset};
use yew::worker::*;

pub fn handle_input(worker: &mut MainWorker, msg: Request, who: HandlerId) {
    log::info!("Request: {:?}", msg);
    match msg {
        Request::Init => {
            log::info!("Initializing...");
            // TODO get actual data
            worker
                .link
                .respond(who, Response::SelfUserFetched(fake_user()));
        }
        Request::SaveSelfUser(user) => {
            log::info!("Saving user...");
            // TODO put/get actual data
            worker.link.respond(who, Response::SelfUserFetched(user));
        }
        Request::VerifyEmail => {
            log::info!("Verifing email...");
            // TODO post actual data
        }
    }
}

fn fake_user() -> devand_core::User {
    use devand_core::*;
    use std::convert::TryFrom;

    let languages = Languages(btreemap![
        Language::C => LanguagePreference { level: Level::Expert, priority: Priority::Low, },
        Language::JavaScript => LanguagePreference { level: Level::Proficient, priority: Priority::Low, },
        Language::CPlusPlus => LanguagePreference { level: Level::Expert, priority: Priority::Low, },
        Language::Rust => LanguagePreference { level: Level::Proficient, priority: Priority::High, },
        Language::Go => LanguagePreference { level: Level::Novice, priority: Priority::No, }
    ]);

    User {
        id: UserId(1),
        username: "alepez".into(),
        visible_name: "Alessandro Pezzato".into(),
        email: "alessandro@pezzato.net".into(),
        email_verified: false,
        settings: UserSettings {
            languages,
            vacation_mode: false,
            schedule: Availability::Weekly(WeekSchedule {
                mon: DaySchedule::try_from("21,22,23").unwrap(),
                tue: DaySchedule::never(),
                wed: DaySchedule::never(),
                thu: DaySchedule::never(),
                fri: DaySchedule::never(),
                sat: DaySchedule::always(),
                sun: DaySchedule::never(),
            }),
            spoken_languages: SpokenLanguages(btreeset![devand_core::SpokenLanguage::English]),
        },
        unread_messages: 5,
        bio: "This is the bio".to_string(),
    }
}
