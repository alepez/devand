use devand_core::{Language, LanguagePreference, Level, Priority};
use yew::{prelude::*, Properties};
use yewtil::NeqAssign;

pub struct LanguageTag {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub lang: Language,
    pub pref: LanguagePreference,
}

impl Component for LanguageTag {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let level_class = format!("devand-language-level-{}", self.props.pref.level).to_lowercase();

        let priority_class =
            format!("devand-language-priority-{}", self.props.pref.priority).to_lowercase();

        let class = vec!["devand-language-tag", &level_class, &priority_class];

        html! {
        <span class=class>
            <span>{ self.props.lang }</span>
            { view_language_level(self.props.pref.level) }
            { view_language_priority(self.props.pref.priority) }
        </span>
        }
    }
}

pub fn view_language_priority(priority: Priority) -> Html {
    let icon = match priority {
        Priority::No => "🚫",
        Priority::Low => "😐",
        Priority::High => "😍",
    };
    let title = format!("{}", priority);

    html! {
    <span class="devand-language-priority-tag"  title=title>{ icon }</span>
    }
}

pub fn view_language_level(level: Level) -> Html {
    let stars = (1..=3).map(|x| x <= level.as_number());
    let icon = |on| if on { "★" } else { "☆" };
    let title = format!("{}", level);

    html! {
    <span class="devand-language-level-tag" title=title>
        { for stars.map(|on| { html! { <span>{ icon(on) }</span> } }) }
    </span>
    }
}
