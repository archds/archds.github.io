use crate::layout::{Column, Row};
use crate::types::AboutItem;
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AboutProps {
    pub items: Vec<AboutItem>,
}

#[function_component]
pub fn About(props: &AboutProps) -> Html {
    let AboutProps { items } = props;
    let about_me_html = items
        .into_iter()
        .map(|item| {
            html! {
                <p>
                    <b>{item.title.clone()}</b>
                    <p>{item.text.clone()}</p>
                </p>
            }
        })
        .collect::<Html>();

    html! {
        <section>
            <Row>
                <Column>
                    <h1>{"About me"}</h1>
                    {about_me_html}
                </Column>
            </Row>
        </section>
    }
}
