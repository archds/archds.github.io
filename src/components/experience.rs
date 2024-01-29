use crate::layout::{Column, Row};
use crate::types::Experience;
use yew::{classes, function_component, html, Html, Properties};

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

            html! {
                <div>
                    <Row>
                        <Column><h3>{format!("{} {}-{}", exp.employer, exp.from_age, exp.to_age)}</h3></Column>
                        <Column class={"is-right"}><h4>{format!("{}", exp.job_title)}</h4></Column>
                    </Row>
                    <p>{"Tools: "}<span class={classes!("text-grey")}>{tools_html}</span></p>
                    <p>{format!("{}", exp.job_duties)}</p>
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
