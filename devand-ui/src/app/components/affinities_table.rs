use crate::app::components::user_affinity_bubble;
use crate::app::components::LanguageTag;
use devand_core::UserAffinity;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct AffinitiesTable {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub affinities: Vec<UserAffinity>,
}

pub enum Msg {}

impl Component for AffinitiesTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        view_affinities_table(&self.props.affinities)
    }
}

fn view_affinities_table(affinities: &[UserAffinity]) -> Html {
    let affinities = affinities.iter().rev().map(|a| view_affinity(a));
    html! {
    <ul class="devand-user-affinities">
    { for affinities}
    </ul>
    }
}

fn view_affinity(user_affinity: &UserAffinity) -> Html {
    let languages = user_affinity.user.languages.clone().into_sorted_vec();

    let languages_tags = languages.iter().map(|(lang, pref)| {
        html! { <LanguageTag lang=lang pref=pref /> }
    });

    html! {
    <li>{ user_affinity_bubble(user_affinity) } { for languages_tags }</li>
    }
}
