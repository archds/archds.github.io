use crate::layout::{Column, Row};
use crate::types::Experience;
use yew::{classes, function_component, html, AttrValue, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ExperiencesProps {
    pub experiences: Vec<Experience>,
}

#[function_component]
pub fn ExperiencesSection(props: &ExperiencesProps) -> Html {
    let ExperiencesProps { experiences } = props;
    let experiences_html = experiences
        .into_iter()
        .map(|exp| {
            let tools_html = exp.tools.join(", ");
            let job_duties_html = exp.job_duties.clone().into_iter().map(|duty| format!("<p>{duty}</p>")).collect::<String>();

            html! {
                <div>
                    <Row>
                        <Column><h3>{format!("{}. {}-{}", exp.employer, exp.from_age, exp.to_age)}</h3></Column>
                        <Column class={"is-right"}><h4>{format!("{}", exp.job_title)}</h4></Column>
                    </Row>
                    <p>{"Tools: "}<span class={classes!("text-grey")}>{tools_html}</span></p>
                    {Html::from_html_unchecked(AttrValue::from(job_duties_html))}
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <section>
            <h1>{"Experience"}</h1>
            {experiences_html}
        </section>
    }
}
