use crate::app::{AppRoute, RouterAnchor, RouterButton};
use devand_core::UserAffinity;
use yew::prelude::*;

pub fn user_affinity_bubble(user: &UserAffinity) -> Html {
    let UserAffinity { user, affinity } = user;
    let username = &user.username;
    let affinity_str = affinity.to_string();

    let (affinity_class, affinity_title) = match affinity.normalize() {
        x if x >= 0.6 => ("devand-affinity-high", "High affinity"),
        x if x >= 0.3 => ("devand-affinity-medium", "Medium affinity"),
        _ => ("devand-affinity-low", "Low affinity"),
    };

    html! {
    <span class="devand-slot-user">
        <span class="devand-start-chat"><RouterButton route=AppRoute::Chat(username.clone())>{ "💬" }</RouterButton></span>
        <span class=("devand-affinity-tag", affinity_class) title=affinity_title>{ affinity_str }</span>
        <span class="devand-visible-name"><RouterAnchor route=AppRoute::UserProfile(username.clone()) >{ &user.visible_name }</RouterAnchor></span>
    </span>
    }
}
