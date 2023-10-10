use crate::components;
use crate::data::get_contacts;
use crate::layout;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <layout::Container>
                <components::Contacts contacts={get_contacts()} />
                <components::About/>
            </layout::Container>
        </main>
    }
}
