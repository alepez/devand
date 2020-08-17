use yew::prelude::*;
use yewtil::{Pure, PureComponent};

pub type BusyIndicator = Pure<PureBusyIndicator>;

#[derive(PartialEq, Properties, Clone)]
pub struct PureBusyIndicator {
    #[prop_or(true)]
    pub running: bool,
}

impl PureComponent for PureBusyIndicator {
    fn render(&self) -> Html {
        let mut class = vec!["devand-busy-indicator"];

        if self.running {
            class.push("devand-running");
        };

        html! { <div class=class>{ "Loading..." }</div>}
    }
}
