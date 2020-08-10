use crate::app::components::user_affinity_bubble;
use crate::app::components::LanguageTag;
use devand_core::UserAffinity;
use yew::prelude::*;

pub fn view_affinities_table(affinities: &[UserAffinity]) -> Html {
    let affinities = affinities.iter().rev().map(|a| view_affinity(a));
    html! {
    <table class="user-affinities pure-table-striped">
    { for affinities}
    </table>
    }
}

fn view_affinity(user_affinity: &UserAffinity) -> Html {
    let languages = user_affinity.user.languages.clone().into_sorted_vec();

    let languages_tags = languages.iter().map(|(lang, pref)| {
        html! { <LanguageTag lang=lang pref=pref /> }
    });

    html! {
    <tr class=("user-affinity")>
        <td>{ user_affinity_bubble(user_affinity) }</td>
        <td class="languages"> { for languages_tags } </td>
    </tr>
    }
}
