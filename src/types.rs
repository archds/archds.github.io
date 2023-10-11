use yew::Properties;

#[derive(PartialEq, Clone)]
pub struct Link {
    pub text: String,
    pub url: String,
    pub _type: Option<LinkType>,
}

#[derive(PartialEq, Clone)]
pub enum TextValue {
    Text(String),
    Link(Link),
}

#[derive(PartialEq, Clone)]
pub enum LinkType {
    Email,
    Phone,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Contact {
    pub name: String,
    pub link: String,
    pub _type: Option<LinkType>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct AboutItem {
    pub title: String,
    pub text: String,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Skill {
    pub title: String,
    pub items: Vec<TextValue>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Language {
    pub logo: Option<String>,
    pub title: String,
    pub skills: Vec<Skill>,
    pub description: Vec<String>,
}
