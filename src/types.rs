use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize, Debug)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "textType", content = "content")]
pub enum TextValue {
    Text(String),
    Link(Link),
}

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize, Debug)]
pub struct ContactInfo {
    pub name: String,
    pub link: String,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Contact {
    Email(ContactInfo),
    Phone(ContactInfo),
    Other(ContactInfo),
}

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize, Debug)]
pub struct AboutItem {
    pub title: String,
    pub text: String,
}
#[derive(PartialEq, Properties, Clone, Serialize, Deserialize, Debug)]
pub struct Skill {
    pub title: String,
    pub items: Vec<TextValue>,
}

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize, Debug)]
pub struct Language {
    pub logo: Option<String>,
    pub title: String,
    pub skills: Vec<Skill>,
    pub description: Vec<String>,
}

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize, Debug)]
pub struct Experience {
    pub employer: String,
    pub from_age: String,
    pub to_age: String,
    pub job_duties: String,
    pub skills: Vec<String>,
}

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationData {
    pub about: Vec<AboutItem>,
    pub contacts: Vec<Contact>,
    pub skills: Vec<Language>,
    pub experiences: Vec<Experience>,
}
