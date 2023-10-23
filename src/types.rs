use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize, Debug)]
pub struct Link {
    pub text: String,
    pub url: String,
    pub _type: Option<LinkType>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "textType", content = "content")]
pub enum TextValue {
    Text(String),
    Link(Link),
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "linkType", content = "content")]
pub enum LinkType {
    Email,
    Phone,
}

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize, Debug)]
pub struct Contact {
    pub name: String,
    pub link: String,
    #[serde(rename = "name")]
    pub _type: Option<LinkType>,
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
