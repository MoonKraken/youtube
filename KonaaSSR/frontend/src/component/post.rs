use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostProps {
    pub title: String,
    pub datetime: String,
    pub content: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    html! {
        <>
            <h1>{props.title.clone()}</h1>
            <h5>{props.datetime.clone()}</h5>
            <p>{props.content.clone()}</p>
        </>
    }
}