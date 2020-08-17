use yew::prelude::*;
use yewtil::{Pure, PureComponent};

pub type CountTag = Pure<PureCountTag>;

#[derive(PartialEq, Properties, Clone)]
pub struct PureCountTag {
    pub count: usize,
}

impl PureComponent for PureCountTag {
    fn render(&self) -> Html {
        if self.count > 0 {
            html! { <span class="devand-count-tag">{ self.count }</span> }
        } else {
            html! {}
        }
    }
}
