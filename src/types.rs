use yew::Properties;

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
