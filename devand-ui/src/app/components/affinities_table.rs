use crate::app::components::LanguageTag;
use crate::app::{AppRoute, RouterAnchor, RouterButton};
use devand_core::{PublicUserProfile, UserAffinity};
use yew::prelude::*;

pub fn view_affinities_table(affinities: &Vec<UserAffinity>) -> Html {
    let affinities = affinities.iter().rev().map(|a| view_affinity(a));
    html! {
    <table class="user-affinities pure-table-striped">
    { for affinities}
    </table>
    }
}

fn view_affinity(affinity: &UserAffinity) -> Html {
    let UserAffinity { user, affinity } = affinity;

    let PublicUserProfile {
        visible_name,
        languages,
        username,
        ..
    } = user;

    let languages = languages.clone().to_sorted_vec();

    let languages_tags = languages.iter().map(|(lang, pref)| {
        html! {
            <LanguageTag lang=lang pref=pref />
        }
    });

    html! {
        <tr class=("user-affinity")>
            <td class="start-chat"><RouterButton route=AppRoute::Chat(username.clone())>{ "💬" }</RouterButton></td>
            <td class="affinity">{ affinity.to_string() }</td>
            <td class="visible_name"><RouterAnchor route=AppRoute::UserProfile(username.clone()) >{ visible_name }</RouterAnchor></td>
            <td class="languages"> { for languages_tags } </td>
        </tr>
    }
}
