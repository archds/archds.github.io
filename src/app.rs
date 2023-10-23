use crate::components;
use crate::data::get_data;
use crate::layout;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let data = get_data().expect("Failed to load application data!");

    html! {
        <main>
            <layout::Container>
                <components::Contacts contacts={data.contacts} />
                <components::About items={data.about} />
                <components::Languages languages={data.skills} />
                <components::ExperiencesSection experiences={data.experiences} />
            </layout::Container>
        </main>
    }
}
