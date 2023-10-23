use crate::layout::{Column, Row};
use crate::types::{Contact, LinkType};
use yew::{classes, function_component, html, Html, Properties};

fn fmt_href(contact: &Contact) -> String {
    match &contact._type {
        Some(_type) => match _type {
            LinkType::Email => format!("mailto: {}", contact.link),
            LinkType::Phone => format!("tel: {}", contact.link),
        },
        None => contact.link.to_owned(),
    }
}

#[derive(PartialEq, Properties)]
pub struct ContactsProps {
    pub contacts: Vec<Contact>,
}

#[function_component]
pub fn Contacts(props: &ContactsProps) -> Html {
    let ContactsProps { contacts } = props;

    let contacts_html = contacts
        .into_iter()
        .map(|contact| {
            html! {
            <p>
                <b>{&contact.name}</b>
                {" - "}
                <a href={fmt_href(contact)}>{&contact.link}</a>
            </p>
            }
        })
        .collect::<Html>();

    html! {
        <Row>
            <Column size={"4"}>
                <div class="is-horizontal-align">
                    <img class={classes!("avatar")} src="assets/200x200.svg" alt="avatar"/>
                </div>
            </Column>
            <Column size={"8"}>
                <h1><b>{"Denis Alekseev"}</b></h1>
                <p>{contacts_html}</p>
            </Column>
        </Row>
    }
}
