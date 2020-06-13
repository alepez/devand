use crate::app::services::CodeNowService;
use devand_core::UserAffinity;
use serde_derive::{Deserialize, Serialize};
use yew::{prelude::*, Properties};

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    affinities: Option<Vec<UserAffinity>>,
}

pub enum Msg {
    AffinitiesFetchOk(Vec<UserAffinity>),
    AffinitiesFetchErr,
}

pub struct CodeNowPage {
    props: Props,
    state: State,
    #[allow(dead_code)]
    affinities_service: CodeNowService,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for CodeNowPage {
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

        let mut affinities_service = CodeNowService::new(callback);

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
            }
            Msg::AffinitiesFetchErr => {
                log::error!("Affinities fetch error");
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
                {
                if let Some(affinities) = &self.state.affinities {
                    self.view_affinities(affinities)
                } else {
                    self.view_loading()
                }
                }
        }
    }
}

impl CodeNowPage {
    fn view_affinities(&self, affinities: &Vec<UserAffinity>) -> Html {
        html! {
            <table class="user-affinities">
            { for affinities.iter().rev().map(|a| self.view_affinity(a)) }
            </table>
        }
    }

    fn view_affinity(&self, affinity: &UserAffinity) -> Html {
        html! {
            <tr class="user-affinity">
                <td class="username">{ &affinity.user.username }</td>
                <td class="visible_name">{ &affinity.user.visible_name }</td>
                <td class="affinity">{ affinity.affinity }</td>
            </tr>
        }
    }

    fn view_loading(&self) -> Html {
        html! {
            <p>{ "Loading..."}</p>
        }
    }
}
