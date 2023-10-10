use yew::{function_component, html, Children, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ContainerProps {
    pub children: Children,
}

#[function_component]
pub fn Container(props: &ContainerProps) -> Html {
    let ContainerProps { children } = props;
    html! {
        <div class="container">{ for children.iter() }</div>
    }
}
