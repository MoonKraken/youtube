use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostProps {
    pub title: String,
    pub datetime: String,
    pub content: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let title_html = if let Some(title) = &props.title {
        html!{<h1>{title}</h1>}
    } else {
        html!{}
    };

    html! {
        <>
            {title_html}
            <h5>{props.post_id.clone()}</h5>
            <p>{props.content.clone()}</p>
        </>
    }
}