use crate::app::{AppRoute, RouterAnchor, RouterButton};
use devand_core::{AffinityLevel, UserAffinity};
use devand_text::Text;
use yew::prelude::*;

fn affinity_title(affinity_level: AffinityLevel) -> Text<'static> {
    match affinity_level {
        AffinityLevel::High => Text::HighAffinity,
        AffinityLevel::Medium => Text::MediumAffinity,
        AffinityLevel::Low => Text::LowAffinity,
    }
}

fn affinity_class(affinity_level: AffinityLevel) -> &'static str {
    match affinity_level {
        AffinityLevel::High => "devand-affinity-high",
        AffinityLevel::Medium => "devand-affinity-medium",
        AffinityLevel::Low => "devand-affinity-low",
    }
}

pub fn user_affinity_bubble(user: &UserAffinity) -> Html {
    let UserAffinity { user, affinity } = user;
    let username = &user.username;
    let affinity_str = affinity.to_string();
    let affinity_level = AffinityLevel::from(*affinity);
    let title = affinity_title(affinity_level);
    let class = affinity_class(affinity_level);

    html! {
    <span class="devand-slot-user">
        <span class="devand-start-chat"><RouterButton route=AppRoute::Chat(username.clone())>{ "💬" }</RouterButton></span>
        <span class=("devand-affinity-tag", class) title=title>{ affinity_str }</span>
        <span class="devand-visible-name"><RouterAnchor route=AppRoute::UserProfile(username.clone()) >{ &user.visible_name }</RouterAnchor></span>
    </span>
    }
}
