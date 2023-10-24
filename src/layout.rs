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

#[derive(PartialEq)]
pub enum ColScreenSize {
    Medium,
    Large,
    Small,
}

#[derive(PartialEq, Properties)]
pub struct ColumnProps {
    pub children: Children,
    pub size: Option<String>,
    pub class: Option<String>,
    pub screen: Option<ColScreenSize>,
}

#[function_component]
pub fn Column(props: &ColumnProps) -> Html {
    let ColumnProps {
        children,
        size,
        class,
        screen,
    } = props;
    let mut col_class = size
        .clone()
        .map(|s| format!("col-{}", s))
        .unwrap_or("col".to_string());

    col_class = screen
        .as_ref()
        .map(|s| match s {
            ColScreenSize::Large => format!("{}-{}", col_class, "lg"),
            ColScreenSize::Medium => format!("{}-{}", col_class, "md"),
            ColScreenSize::Small => format!("{}", col_class),
        })
        .unwrap_or(col_class);

    html! {
        <div class={classes!(col_class, class)}>{ for children.iter() }</div>
    }
}

#[derive(PartialEq, Properties)]
pub struct RowProps {
    pub children: Children,
    pub class: Option<String>,
}

#[function_component]
pub fn Row(props: &RowProps) -> Html {
    let RowProps { children, class } = props;
    html! {
        <div class={classes!("row", class)}>{ for children.iter() }</div>
    }
}
