use crate::layout;
use yew::{classes, function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct MadeWithProps {}

#[function_component]
pub fn MadeWith(props: &MadeWithProps) -> Html {
    let MadeWithProps {} = props;
    html! {
        <layout::Row>
            <layout::Column class={"text-right"}>
                <span class={classes!("text-grey")}>{"Made with: "}</span>
                <a target={"_blank"} href={"https://yew.rs/"}>{"Yew"}</a><span>{", "}</span>
                <a target={"_blank"} href={"https://jenil.github.io/chota/"}>{"chota"}</a><span>{", "}</span>
                <a target={"_blank"} href={"https://trunkrs.dev/"}>{"Trunk"}</a>
            </layout::Column>
        </layout::Row>
    }
}
