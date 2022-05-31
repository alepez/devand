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
        let level_class = format!("devand-language-level-{}", self.props.pref.level).to_lowercase();

        let priority_class =
            format!("devand-language-priority-{}", self.props.pref.priority).to_lowercase();

        let class = vec![
            "devand-editable-language-tag".to_string(),
            "devand-language-tag".to_string(),
            level_class,
            priority_class,
        ];

        html! {
            <span class=class>
                <button class="pure-button" onclick=self.link.callback(move |_| Msg::Remove)>{ "✖" }</button>
                <span>{ self.props.lang }</span>
                { view_language_level(self.props.pref.level) }
                { view_language_priority(self.props.pref.priority) }
            </span>
        }
    }
}
