use crate::app::services::CodeNowService;
use devand_core::{PublicUserProfile, CodeNowUsers};
use serde_derive::{Deserialize, Serialize};
use yew::{prelude::*, Properties};

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    code_now_users: Option<CodeNowUsers>,
}

pub enum Msg {
    CodeNowUsersFetchOk(CodeNowUsers),
    CodeNowUsersFetchErr,
}

pub struct CodeNowPage {
    props: Props,
    state: State,
    #[allow(dead_code)]
    service: CodeNowService,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for CodeNowPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();

        let callback = link.callback(|result: Result<CodeNowUsers, anyhow::Error>| {
            if let Ok(code_now_users) = result {
                Msg::CodeNowUsersFetchOk(code_now_users)
            } else {
                Msg::CodeNowUsersFetchErr
            }
        });

        let mut service = CodeNowService::new(callback);

        service.restore();

        Self {
            props,
            state,
            service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CodeNowUsersFetchOk(code_now_users) => {
                self.state.code_now_users = Some(code_now_users);
            }
            Msg::CodeNowUsersFetchErr => {
                log::error!("CodeNowUsers fetch error");
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
                if let Some(code_now_users) = &self.state.code_now_users {
                    self.view_code_now_users(code_now_users)
                } else {
                    self.view_loading()
                }
                }
        }
    }
}

impl CodeNowPage {
    fn view_code_now_users(&self, code_now_users: &CodeNowUsers) -> Html {
        html! {
            <table class="code_now_users">
            { for code_now_users.0.iter().rev().map(|x| self.view_user_profile(x)) }
            </table>
        }
    }

    fn view_user_profile(&self, user: &PublicUserProfile) -> Html {
        html! {
            <tr class="user_profile">
                <td class="username">{ &user.username }</td>
                <td class="visible_name">{ &user.visible_name }</td>
            </tr>
        }
    }

    fn view_loading(&self) -> Html {
        html! {
            <p>{ "Loading..."}</p>
        }
    }
}
