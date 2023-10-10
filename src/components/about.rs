use crate::data::ABOUT_ME;
use crate::layout::{Column, Row};
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AboutProps {}

#[function_component]
pub fn About(props: &AboutProps) -> Html {
    let AboutProps {} = props;
    let about_me_html = ABOUT_ME
        .into_iter()
        .map(|(head, text)| {
            html! {
                <p>
                    <b>{head}</b>
                    <p>{text}</p>
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
