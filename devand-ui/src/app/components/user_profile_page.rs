use crate::app::components::LanguageTag;
use crate::app::elements::busy_indicator;
use crate::app::workers::{main_worker, main_worker::MainWorker};
use crate::app::{AppRoute, RouterButton};
use devand_core::PublicUserProfile;
use devand_core::SpokenLanguages;
use yew::{prelude::*, Properties};

pub struct UserProfilePage {
    props: Props,
    state: State,
    main_worker: Box<dyn Bridge<MainWorker>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub username: String,
}

pub enum Msg {
    MainWorkerRes(main_worker::Response),
}

#[derive(Default)]
struct State {
    other_user: Option<PublicUserProfile>,
}

impl Component for UserProfilePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut main_worker = MainWorker::bridge(link.callback(Msg::MainWorkerRes));

        main_worker.send(main_worker::Request::LoadPublicUserProfileByUsername(
            props.username.clone(),
        ));

        let state = State::default();

        Self {
            props,
            state,
            main_worker,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MainWorkerRes(res) => {
                use main_worker::Response;

                match res {
                    Response::PublicUserProfileFetched(other_user) => {
                        self.state.other_user = Some(*other_user);
                        true
                    }

                    _ => false,
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = if self.props.username != props.username {
            self.main_worker
                .send(main_worker::Request::LoadPublicUserProfileByUsername(
                    props.username.clone(),
                ));
            true
        } else {
            false
        };

        self.props = props;
        changed
    }

    fn view(&self) -> Html {
        if let Some(other_user) = &self.state.other_user {
            let PublicUserProfile {
                languages,
                spoken_languages,
                visible_name,
                ..
            } = other_user;

            let languages = languages.clone().into_sorted_vec();

            let languages_tags = languages.iter().map(|(lang, pref)| {
                html! { <LanguageTag lang=lang pref=pref /> }
            });

            html! {
            <>
                <h1><RouterButton route=AppRoute::Chat(other_user.username.clone())>{ "💬 " }</RouterButton>{ other_user.full_name() }</h1>
                <p class="devand-user-bio">{ &other_user.bio }</p>

                <h2>{"Languages"}</h2>
                <div>
                    { for languages_tags }
                </div>

                { view_spoken_languages(&visible_name, &spoken_languages) }
            </>
            }
        } else {
            busy_indicator()
        }
    }
}

fn view_spoken_languages(visible_name: &str, spoken_languages: &SpokenLanguages) -> Html {
    if spoken_languages.is_empty() {
        html! {}
    } else {
        let spoken_languages = spoken_languages.iter().map(|x| html! { <li>{ x }</li> });
        html! {
        <>
            <h2>{ format!("{} speaks:", visible_name) }</h2>
            <ul>
                <li>{ for spoken_languages }</li>
            </ul>
        </>
        }
    }
}
