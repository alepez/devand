use yew::{prelude::*, Properties};

pub struct SchedulePage {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {}

impl Component for SchedulePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Schedule" }</h1>
                <p>{ "WIP - schedule will be here" }</p>
            </>
        }
    }
}
