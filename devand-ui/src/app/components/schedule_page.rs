use crate::app::components::user_affinity_bubble;
use crate::app::workers::{main_worker, main_worker::MainWorker};
use crate::app::AppRoute;
use crate::app::RouterAnchor;
use chrono::{DateTime, Utc};
use devand_core::schedule_matcher::AvailabilityMatch;
use devand_core::{Affinity, AffinityParams, PublicUserProfile, UserAffinity, UserId};
use yew::{prelude::*, Properties};
use yewtil::NeqAssign;

pub struct SchedulePage {
    props: Props,
    state: State,
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    main_worker: Box<dyn Bridge<MainWorker>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub me: PublicUserProfile,
}

pub enum Msg {
    LoadUser(devand_core::UserId),
    MainWorkerRes(main_worker::Response),
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

        let mut main_worker = MainWorker::bridge(link.callback(Msg::MainWorkerRes));
        main_worker.send(main_worker::Request::LoadAvailabilityMatch);

        Self {
            props,
            state,
            link,
            main_worker,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MainWorkerRes(res) => {
                use main_worker::Response;

                match res {
                    Response::PublicUserProfileFetched(user) => {
                        self.state.users.insert(user.id, user);
                        true
                    }
                    Response::AvailabilityMatchFetched(schedule) => {
                        self.state.schedule = Some(schedule);
                        true
                    }
                    _ => false,
                }
            }
            Msg::LoadUser(user_id) => {
                if !self.state.user_requests.contains(&user_id) {
                    self.state.user_requests.insert(user_id);
                    self.main_worker
                        .send(main_worker::Request::LoadPublicUserProfile(user_id));
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

    fn view_all_slots_empty(&self) -> Html {
        html! {
            <div class=("alert", "alert-warning")>
                {"Sorry, there are no available users. You can try to "} <RouterAnchor route=AppRoute::Settings >{ "extend your languages selection." }</RouterAnchor>
            </div>
        }
    }

    fn view_slots(&self, slots: &[(DateTime<Utc>, Vec<UserId>)]) -> Html {
        let mut at_least_one_non_empty_slot = false;

        let slots: Vec<_> = slots
            .iter()
            .map(|(t, users)| {
                let mut users: Vec<_> = users
                    .iter()
                    .filter_map(|&u| self.expand_user(u))
                    .filter(|u| !u.affinity.is_zero())
                    .collect();

                at_least_one_non_empty_slot |= !users.is_empty();

                users.sort_by_key(|u| std::cmp::Reverse(u.affinity));

                (t, users)
            })
            .filter(|(_, users)| !users.is_empty())
            .collect();

        let slots_view = slots
            .into_iter()
            .map(|(t, users)| html! { <li> { self.view_slot(t, users) } </li> });

        if at_least_one_non_empty_slot {
            html! {
                <ul class="devand-schedule-slots">
                    { for slots_view }
                </ul>
            }
        } else {
            self.view_all_slots_empty()
        }
    }

    fn view_slot(&self, t: &DateTime<Utc>, users: Vec<UserAffinity>) -> Html {
        html! {
            <>
            <span class="devand-slot-time">{ view_timestamp(t) }</span>
            <span class="devand-slot-users">
            { for users.into_iter().map(|u| user_affinity_bubble(&u)) }
            </span>
            </>
        }
    }

    fn expand_user(&self, user_id: UserId) -> Option<UserAffinity> {
        if let Some(user) = self.state.users.get(&user_id) {
            let my_aff_params =
                AffinityParams::new().with_languages(self.props.me.languages.clone());

            let u_aff_params = AffinityParams::new().with_languages(user.languages.clone());
            let affinity = Affinity::from_params(&my_aff_params, &u_aff_params);

            // TODO [optimization] Avoid clone
            Some(UserAffinity {
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

fn view_timestamp(t: &chrono::DateTime<chrono::Utc>) -> impl ToString {
    t.format("%A, %B %d - %R UTC")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn timestamp_format() {
        use chrono::offset::TimeZone;
        let t = chrono::Utc.ymd(2020, 8, 2).and_hms_milli(20, 0, 1, 444);
        let formatted_t = view_timestamp(&t);
        assert_eq!("Sunday, August 02 - 20:00 UTC", formatted_t.to_string());
    }
}
