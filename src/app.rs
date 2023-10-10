use crate::components;
use crate::layout;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <layout::Container>
                <components::About/>
            </layout::Container>
        </main>
    }
}
