use crate::components;
use crate::data::{get_contacts, get_skill_info, ABOUT_ME, get_experiences};
use crate::layout;
use crate::types::AboutItem;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <layout::Container>
                <components::Contacts contacts={get_contacts()} />
                <components::About items={ABOUT_ME.into_iter().map(|(head, text)| {
                    AboutItem {
                        title: head.to_string(),
                        text: text.to_string(),
                    }
                }).collect::<Vec<_>>()} />
                <components::Languages languages={get_skill_info()} />
                <components::ExperiencesSection experiences={get_experiences()} />
            </layout::Container>
        </main>
    }
}
