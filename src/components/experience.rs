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
            let skills_html = exp.skills.join(", ");

            html! {
                <div>
                    <h3>{format!("{}. {}-{}.", exp.employer, exp.from_age, exp.to_age)}</h3>
                    <p>{"Tools: "}<span class={classes!("text-grey")}>{skills_html}</span></p>
                    <p>{format!("{}", exp.job_duties)}</p>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <section class={classes!("page-break")}>
            <h1>{"Experience"}</h1>
            {experiences_html}
        </section>
    }
}
