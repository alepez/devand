use crate::app::services::{ScheduleService, ScheduleServiceContent};
use crate::app::{AppRoute, RouterButton};
use chrono::{DateTime, Utc};
use devand_core::schedule_matcher::AvailabilityMatch;
use devand_core::{Affinity, AffinityParams, PublicUserProfile, UserId};
use yew::{prelude::*, Properties};
// use crate::app::components::LanguageTag;
use crate::app::RouterAnchor;
use yewtil::NeqAssign;

pub struct SchedulePage {
    props: Props,
    #[allow(dead_code)]
    service: ScheduleService,
    state: State,
    #[allow(dead_code)]
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub me: PublicUserProfile,
}

pub enum Msg {
    Loaded(Result<ScheduleServiceContent, anyhow::Error>),
    LoadUser(UserId),
}

#[derive(Default)]
struct State {
    schedule: Option<AvailabilityMatch>,
    users: std::collections::BTreeMap<UserId, PublicUserProfile>,
    user_requests: std::collections::BTreeSet<UserId>,
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
            Msg::Loaded(result) => {
                match result {
                    Ok(content) => match content {
                        ScheduleServiceContent::AvailabilityMatch(schedule) => {
                            self.state.schedule = Some(schedule);
                        }
                        ScheduleServiceContent::PublicUserProfile(user) => {
                            self.state.users.insert(user.id, user);
                        }
                    },
                    Err(err) => log::error!("Error: {:?}", err),
                };
                true
            }
            Msg::LoadUser(user_id) => {
                if !self.state.user_requests.contains(&user_id) {
                    self.state.user_requests.insert(user_id);
                    self.service.load_public_profile(user_id);
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
        <>
        <h1>{ "Schedule" }</h1>
        <p>{ "Here you find a list of users available at the same time as you." }</p>
        <p>{ "Just choose someone to pair-program with and start chatting" }</p>
        {
        if let Some(schedule) = &self.state.schedule {
            self.view_schedule(schedule)
        } else {
            crate::app::elements::busy_indicator()
        }
        }
        </>
        }
    }
}

impl SchedulePage {
    fn view_schedule(&self, schedule: &AvailabilityMatch) -> Html {
        if schedule.slots.is_empty() {
            self.view_no_slots()
        } else {
            self.view_slots(&schedule.slots)
        }
    }

    fn view_no_slots(&self) -> Html {
        html! {
            <div class=("alert", "alert-warning")>
                {"Sorry, there are no available users. You can try to "} <RouterAnchor route=AppRoute::Settings >{ "extend your availability." }</RouterAnchor>
            </div>
        }
    }

    fn view_slots(&self, slots: &Vec<(DateTime<Utc>, Vec<UserId>)>) -> Html {
        let slots: Vec<_> = slots
            .iter()
            .map(|(t, users)| {
                let mut users: Vec<_> = users
                    .into_iter()
                    .filter_map(|&u| self.expand_user(u))
                    .filter(|u| !u.affinity.is_zero())
                    .collect();

                (t, users)
            })
            .collect();

        let slots_view = slots
            .into_iter()
            .map(|(t, users)| html! { <li> { self.view_slot(t, users) } </li> });

        html! {
            <ul class="devand-schedule-slots">
                { for slots_view }
            </ul>
        }
    }

    fn view_slot(&self, t: &DateTime<Utc>, users: Vec<ExpandedUser>) -> Html {
        html! {
            <>
            <span class="devand-slot-time">{ t.to_string() }</span>
            <span class="devand-slot-users">
            { for users.into_iter().map(|u| self.view_user_profile(u)) }
            </span>
            </>
        }
    }

    fn view_user_profile(&self, u: ExpandedUser) -> Html {
        let ExpandedUser { user, affinity } = u;

        html! {
        <span class="devand-slot-user">
            <span class="devand-start-chat"><RouterButton route=AppRoute::Chat(user.username)>{ "💬" }</RouterButton></span>
            <span class="devand-visible-name">{ &user.visible_name }</span>
            <span class="devand-affinity">{ affinity.to_string() }</span>
        </span>
        }
    }

    fn expand_user(&self, user_id: UserId) -> Option<ExpandedUser> {
        if let Some(user) = self.state.users.get(&user_id) {
            let my_aff_params =
                AffinityParams::new().with_languages(self.props.me.languages.clone());

            let u_aff_params = AffinityParams::new().with_languages(user.languages.clone());
            let affinity = Affinity::from_params(&my_aff_params, &u_aff_params);

            // TODO [optimization] Avoid clone
            Some(ExpandedUser {
                user: user.clone(),
                affinity,
            })
        } else {
            // Load user public profile, but only if loading has not already started
            self.link.send_message(Msg::LoadUser(user_id));
            None
        }
    }
}

struct ExpandedUser {
    user: devand_core::PublicUserProfile,
    affinity: Affinity,
}
