use crate::app::services::{ScheduleService, ScheduleServiceContent};
use chrono::{DateTime, Utc};
use devand_core::schedule_matcher::AvailabilityMatch;
use devand_core::{Affinity, AffinityParams, PublicUserProfile, UserId};
use yew::{prelude::*, Properties};
// use crate::app::components::LanguageTag;

pub struct SchedulePage {
    props: Props,
    #[allow(dead_code)]
    service: ScheduleService,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub me: PublicUserProfile,
}

pub enum Msg {
    Loaded(Result<ScheduleServiceContent, anyhow::Error>),
}

#[derive(Default)]
struct State {
    schedule: Option<AvailabilityMatch>,
    users: std::collections::BTreeMap<UserId, PublicUserProfile>,
}

impl Component for SchedulePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();
        let schedule_loaded = link.callback(Msg::Loaded);
        let mut service = ScheduleService::new(schedule_loaded);
        service.load();

        Self {
            props,
            state,
            link,
            service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded(result) => match result {
                Ok(content) => match content {
                    ScheduleServiceContent::AvailabilityMatch(schedule) => {
                        self.state.schedule = Some(schedule);
                    }
                    ScheduleServiceContent::PublicUserProfile(user) => {
                        self.state.users.insert(user.id, user);
                    }
                },
                Err(err) => log::error!("Error: {:?}", err),
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if let Some(schedule) = &self.state.schedule {
            self.view_schedule(schedule)
        } else {
            crate::app::elements::busy_indicator()
        }
    }
}

impl SchedulePage {
    fn view_schedule(&self, schedule: &AvailabilityMatch) -> Html {
        let slots = schedule
            .slots
            .iter()
            .map(|(t, users)| html! { <li> { self.view_item(t, users) } </li> });

        html! {
            <ul>
                { for slots }
            </ul>
        }
    }

    fn view_item(&self, t: &DateTime<Utc>, users: &Vec<UserId>) -> Html {
        html! {
            <>
            <span class="devand-slot-time">{ t.to_string() }</span>
            <span class="devand-slot-users">
            { for users.iter().map(|&u| self.view_user_profile(u)) }
            </span>
            </>
        }
    }

    fn view_user_profile(&self, user_id: UserId) -> Html {
        if let Some(user) = self.state.users.get(&user_id) {
            // TODO Showing languages takes too long
            // let languages = &user.languages;
            // let lang_tags = languages.iter().map(|(lang, pref)| {
            //     html! { <LanguageTag lang=lang pref=pref /> }
            // });

            let my_aff_params = AffinityParams::new().with_languages(self.props.me.languages.clone());
            let u_aff_params = AffinityParams::new().with_languages(user.languages.clone());

            let affinity = Affinity::from_params(&my_aff_params, &u_aff_params);

            html! {
            <>
                <span>{ &user.visible_name }</span>
                <span class="devand-affinity">{ affinity.to_string() }</span>
                // <span>{ for lang_tags }</span>
            </>
            }
        } else {
            // TODO Trigger loading of user profile
            self.service.load_public_profile(user_id);
            html! { <></> }
        }
    }
}
