use crate::app::components::affinities_table::view_affinities_table;
use crate::app::elements::busy_indicator;
use crate::app::services::AffinitiesService;
use crate::app::{AppRoute, RouterAnchor};
use devand_core::UserAffinity;
use yew::{prelude::*, Properties};
use yewtil::NeqAssign;

#[derive(Default)]
pub struct State {
    affinities: Option<Vec<UserAffinity>>,
}

pub enum Msg {
    AffinitiesFetchOk(Vec<UserAffinity>),
    AffinitiesFetchErr,
}

pub struct AffinitiesPage {
    props: Props,
    state: State,
    #[allow(dead_code)]
    affinities_service: AffinitiesService,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for AffinitiesPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();

        let callback = link.callback(|result: Result<Vec<UserAffinity>, anyhow::Error>| {
            if let Ok(affinities) = result {
                Msg::AffinitiesFetchOk(affinities)
            } else {
                Msg::AffinitiesFetchErr
            }
        });

        let mut affinities_service = AffinitiesService::new(callback);

        affinities_service.restore();

        Self {
            props,
            state,
            affinities_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AffinitiesFetchOk(mut affinities) => {
                affinities.sort_unstable_by_key(|x| x.affinity);
                self.state.affinities = Some(affinities);
                true
            }
            Msg::AffinitiesFetchErr => {
                log::error!("Affinities fetch error");
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
        <h1>{ "Affinities" }</h1>
        {
            if let Some(affinities) = &self.state.affinities {
                view_affinities(affinities)
            } else {
                busy_indicator()
            }
        }
        </>
        }
    }
}

fn view_affinities(affinities: &[UserAffinity]) -> Html {
    if affinities.is_empty() {
        view_no_affinities()
    } else {
        html! {
        <>
            <p>{ "In the table below, you can see a list of developers who love the same languages as you. Just click the chat icon to start chatting and organize your next pair-programming session." }</p>
            { view_affinities_table(affinities) }
        </>
        }
    }
}

fn view_no_affinities() -> Html {
    html! {
        <div class=("alert", "alert-warning")>
            {"Sorry, no matching users found. You can try to "} <RouterAnchor route=AppRoute::Settings >{ "extend your languages selection." }</RouterAnchor>
        </div>
    }
}
