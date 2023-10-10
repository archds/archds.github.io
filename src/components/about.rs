use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AboutProps {}

#[function_component]
pub fn About(props: &AboutProps) -> Html {
    let AboutProps {} = props;
    html! {
        <p>{"Hello world"}</p>
    }
}
