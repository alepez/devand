use yew::prelude::*;
use yewtil::{Pure, PureComponent};

pub type Alert = Pure<PureAlert>;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum AlertLevel {
    Success,
    Info,
    Warning,
    Danger,
}

impl Into<&'static str> for AlertLevel {
    fn into(self) -> &'static str {
        match self {
            AlertLevel::Success => "alert-success",
            AlertLevel::Info => "alert-info",
            AlertLevel::Warning => "alert-warning",
            AlertLevel::Danger => "alert-danger",
        }
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct PureAlert {
    pub children: yew::html::Children,
    #[prop_or(AlertLevel::Warning)]
    pub level: AlertLevel,
    #[prop_or_default]
    pub class: &'static str,
}

impl PureComponent for PureAlert {
    fn render(&self) -> Html {
        let level_str: &'static str = self.level.into();
        html! {
        <div class=classes!("alert", level_str, self.class)>
            { self.children.clone() }
        </div>
        }
    }
}
