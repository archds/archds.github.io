use yew::{classes, function_component, html, Children, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ContainerProps {
    pub children: Children,
}

#[function_component]
pub fn Container(props: &ContainerProps) -> Html {
    let ContainerProps { children } = props;
    html! {
        <div class="container">{ for children.iter() }</div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ColumnProps {
    pub children: Children,
    pub size: Option<String>,
    pub class: Option<String>,
}

#[function_component]
pub fn Column(props: &ColumnProps) -> Html {
    let ColumnProps {
        children,
        size,
        class,
    } = props;
    let col_class = size
        .clone()
        .map(|s| format!("col-{}", s))
        .unwrap_or("col".to_string());

    html! {
        <div class={classes!(col_class, class)}>{ for children.iter() }</div>
    }
}

#[derive(PartialEq, Properties)]
pub struct RowProps {
    pub children: Children,
}

#[function_component]
pub fn Row(props: &RowProps) -> Html {
    let RowProps { children } = props;
    html! {
        <div class={classes!("row")}>{ for children.iter() }</div>
    }
}