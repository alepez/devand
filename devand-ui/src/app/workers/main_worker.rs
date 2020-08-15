use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use yew::services::interval::IntervalService;
use yew::services::Task;
use yew::worker::*;
use maplit::btreeset;
use devand_core::User;

const INTERVAL_MS: u64 = 2_000;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Init,
    SaveSelfUser(User),
}

// TODO Add Error
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    SelfUserFetched(User),
}

pub enum Msg {
    Updating,
}

pub struct MainWorker {
    link: AgentLink<MainWorker>,
    _interval_task: Box<dyn Task>,
}

impl Agent for MainWorker {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let duration = Duration::from_millis(INTERVAL_MS);
        let callback = link.callback(|_| Msg::Updating);
        let task = IntervalService::spawn(duration, callback);
        MainWorker {
            link,
            _interval_task: Box::new(task),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                log::info!("Tick...");
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        log::info!("Request: {:?}", msg);
        match msg {
            Request::Init => {
                log::info!("Initializing...");
                // TODO get actual data
                self.link.respond(who, Response::SelfUserFetched(fake_user()));
            }
            Request::SaveSelfUser(user) => {
                log::info!("Saving user...");
                // TODO put/get actual data
                self.link.respond(who, Response::SelfUserFetched(user));
            }
        }
    }
}

fn fake_user() -> devand_core::User {
    use devand_core::*;
    use std::collections::BTreeMap;
    use std::convert::TryFrom;

    let mut languages = BTreeMap::default();

    languages.insert(
        Language::C,
        LanguagePreference {
            level: Level::Expert,
            priority: Priority::Low,
        },
    );
    languages.insert(
        Language::JavaScript,
        LanguagePreference {
            level: Level::Proficient,
            priority: Priority::Low,
        },
    );
    languages.insert(
        Language::CPlusPlus,
        LanguagePreference {
            level: Level::Expert,
            priority: Priority::Low,
        },
    );
    languages.insert(
        Language::Rust,
        LanguagePreference {
            level: Level::Proficient,
            priority: Priority::High,
        },
    );
    languages.insert(
        Language::Go,
        LanguagePreference {
            level: Level::Novice,
            priority: Priority::No,
        },
    );

    let languages = Languages(languages);

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
