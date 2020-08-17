use devand_text::Text;
use yew::{prelude::*, Properties};
use yewtil::NeqAssign;

pub struct NotFoundPage {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub missed_route: Option<String>,
}

impl Component for NotFoundPage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NotFoundPage { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if let Some(missed_route) = &self.props.missed_route {
            html! { <h1>{ Text::PageNotFound(missed_route)}</h1> }
        } else {
            html! { <h1>{ Text::UnknownPageNotFound }</h1> }
        }
    }
}
