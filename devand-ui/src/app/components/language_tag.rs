use devand_core::{Language, LanguagePreference, Level, Priority};
use yew::{prelude::*, Properties};

pub struct LanguageTag {
    props: LanguageTagProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LanguageTagProps {
    pub lang: Language,
    pub pref: LanguagePreference,
}

impl Component for LanguageTag {
    type Message = ();
    type Properties = LanguageTagProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="language-control-group ">
                <span class="language-tag">{ self.props.lang }</span>
                { view_language_level(self.props.pref.level) }
                { view_language_priority(self.props.pref.priority) }
            </div>
        }
    }
}

pub struct EditableLanguageTag {
    props: EditableLanguageTagProps,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EditableLanguageTagProps {
    pub lang: Language,
    pub pref: LanguagePreference,
    pub on_remove: Callback<()>,
}

pub enum EditableLanguageMsg {
    Remove,
}

impl Component for EditableLanguageTag {
    type Message = EditableLanguageMsg;
    type Properties = EditableLanguageTagProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            EditableLanguageMsg::Remove => {
                self.props.on_remove.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="language-control-group pure-u-1 pure-u-md-1-2 pure-u-lg-1-3">
                <span class="language-tag">
                    <button class="pure-button" onclick=self.link.callback(move |_| EditableLanguageMsg::Remove)>{ "✖" }</button>
                </span>
                <span class="language-tag">{ self.props.lang }</span>
                { view_language_level(self.props.pref.level) }
                { view_language_priority(self.props.pref.priority) }
            </div>
        }
    }
}

fn view_language_priority(priority: Priority) -> Html {
    let icon = match priority {
        Priority::No => "X",
        Priority::Low => ":|",
        Priority::High => ":)",
    };
    let title = format!("{}", priority);
    let priority_class = format!("language-priority-tag-{}", priority).to_lowercase();
    let class = vec!["language-priority-tag", &priority_class];

    html! {
        <span class=class title=title>{ icon }</span>
    }
}

fn view_language_level(level: Level) -> Html {
    let stars = (1..=3).map(|x| x <= level.as_number());
    let icon = |on| if on { "★" } else { "☆" };
    let title = format!("{}", level);
    let level_class = format!("language-level-tag-{}", level).to_lowercase();
    let class = vec!["language-level-tag", &level_class];

    html! {
        <span class=class title=title>
            { for stars.map(|on| { html! { <span>{ icon(on) }</span> } }) }
        </span>
    }
}
