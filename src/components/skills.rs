use yew::{classes, function_component, html, AttrValue, Html, Properties};

use crate::layout::{Column, Row, ColScreenSize};
use crate::types::{Language, TextValue};

#[derive(PartialEq, Properties)]
pub struct LanguageInfoProps {
    pub language: Language,
}

#[function_component]
pub fn LanguageInfo(props: &LanguageInfoProps) -> Html {
    let LanguageInfoProps { language } = props;
    let skills_html = language
        .skills
        .iter()
        .map(|skill| {
            let skill_items_html = skill
                .items
                .iter()
                .map(|item| match item {
                    TextValue::Link(link) => {
                        format!("<a target=\"_blank\" href={}>{}</a>", link.url.clone(), link.text.clone())
                    }
                    TextValue::Text(text) => format!("<span>{}</span>", text),
                })
                .collect::<Vec<String>>()
                .join("<span>, </span>");

            html! {
               <p>
                   <span class="text-grey">{skill.title.clone()}{": "}</span>
                   {Html::from_html_unchecked(AttrValue::from(skill_items_html))}
               </p>
            }
        })
        .collect::<Vec<Html>>();

    html! {
        <Row>
            <Column>
                <h2 class={classes!("is-vertical-align ")}>
                    if language.logo.is_some() {
                        <img class={classes!("skill-logo")} src={language.logo.clone().unwrap()} alt=""/>
                    }
                    <span>{language.title.clone()}</span>
                </h2>
                {skills_html}
            </Column>
        </Row>
    }
}

#[derive(PartialEq, Properties)]
pub struct LanguagesProps {
    pub languages: Vec<Language>,
}

#[function_component]
pub fn Languages(props: &LanguagesProps) -> Html {
    let LanguagesProps { languages } = props;

    let langs_html = languages.iter().map(|lang| {
        let description_html = lang.description.iter().map(|p| {
            html! { <p>{p}</p> }
        }).collect::<Html>();

        html! {
            <Row>
                <Column size={"3"} screen={ColScreenSize::Large}>
                    <LanguageInfo language={lang.clone()} />
                </Column>
                <Column class={"is-vertical-align is-horizontal-align"} size={"1"} screen={ColScreenSize::Large}>
                    <div class="vertical-line hide-xs hide-sm hide-md hide-pr"></div>
                </Column>
                <Column class={"skill-description"} size={"8"} screen={ColScreenSize::Large}>
                    {description_html}
                </Column>
            </Row>
        }
    }).collect::<Vec<Html>>();

    html! {
        <section class={classes!("page-break")}>
            <h1>{"Skills"}</h1>
            {langs_html}
        </section>
    }
}
