use crate::layout::{Column, Row};
use crate::types::Contact;
use yew::{classes, function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ContactsProps {
    pub contacts: Vec<Contact>,
}

#[function_component]
pub fn Contacts(props: &ContactsProps) -> Html {
    let ContactsProps { contacts } = props;

    let contacts_html = contacts
        .into_iter()
        .map(|contact| match contact {
            Contact::Email(ct) => html! {
            <p>
                <b>{&ct.name}</b>
                {" - "}
                <a target={"blank"} href={format!("mailto: {}", ct.link)}>{&ct.link}</a>
            </p>
            },
            Contact::Phone(ct) => html! {
            <p>
                <b>{&ct.name}</b>
                {" - "}
                <a target={"blank"} href={format!("tel: {}", ct.link)}>{&ct.link}</a>
            </p>
            },
            Contact::Other(ct) => html! {
            <p>
                <b>{&ct.name}</b>
                {" - "}
                <a target={"blank"} href={format!("{}", ct.link)}>{&ct.link}</a>
            </p>
            },
        })
        .collect::<Html>();

    html! {
        <Row>
            <Column size={"4"}>
                <div class="is-horizontal-align">
                    <img class={classes!("avatar")} src="assets/avatar.jpg" alt="avatar"/>
                </div>
            </Column>
            <Column size={"8"}>
                <h1><b>{"Denis Alekseev"}</b></h1>
                <p>{contacts_html}</p>
            </Column>
        </Row>
    }
}
