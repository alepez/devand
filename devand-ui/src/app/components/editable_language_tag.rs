use super::language_tag::{view_language_level, view_language_priority};
use devand_core::{Language, LanguagePreference};
use yew::{prelude::*, Properties};
use yewtil::NeqAssign;

pub struct EditableLanguageTag {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub lang: Language,
    pub pref: LanguagePreference,
    pub on_remove: Callback<()>,
}

pub enum Msg {
    Remove,
}

impl Component for EditableLanguageTag {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Remove => {
                self.props.on_remove.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <span class="devand-editable-language-tag devand-language-tag">
                <button class="pure-button" onclick=self.link.callback(move |_| Msg::Remove)>{ "✖" }</button>
                <span>{ self.props.lang }</span>
                { view_language_level(self.props.pref.level) }
                { view_language_priority(self.props.pref.priority) }
            </span>
        }
    }
}
