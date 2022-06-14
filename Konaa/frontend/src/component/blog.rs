use reqwasm::http::Request;
use common::model::blog::Blog as BlogModel;
use common::model::post::Post;
use yew::prelude::*;

async fn get_blog(id: &String) -> BlogModel {
    let url = format!("/api/{}", id);
    return Request::get(&url).send().await.unwrap().json().await.unwrap();
}

#[derive(Properties, Clone, PartialEq)]
pub struct BlogViewProps {
    pub blog_id: String
}

fn post_to_html(post: &Post) -> Html {
    let title_html = if let Some(title) = &post.title {
        html!{<h1>{title}</h1>}
    } else {
        html!{}
    };

    html! {
        <>
            {title_html}
            <h5>{post.post_id.clone()}</h5>
            <p>{post.content.clone()}</p>
        </>
    }
}

#[function_component(Blog)]
pub fn blog(props: &BlogViewProps) -> Html {
    let blog_id = props.blog_id.clone();
    let posts = use_state(|| vec![]);
    let title = use_state(|| String::new());
    let subtitle = use_state(|| String::new());
    {
        let posts = posts.clone();
        use_effect_with_deps(move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local( async move {
                let blog: BlogModel = get_blog(&blog_id).await;
                posts.set(blog.posts);
                blog.title.map(|blog_title| title.set(blog_title));
                blog.subtitle.map(|blog_subtitle| subtitle.set(blog_subtitle));
            });
            || ()
        },());
    }

    let posts_view =
        (*posts).iter().map(|post| post_to_html(post));

    let current_time: String = chrono::Local::now().to_string();

    if posts_view.len() > 0 {
        return html!{
            <>
                {posts_view.collect::<Html>()}
            </>
        }
    } else {
        return html! {
            <>
                <h5>{"Render Time "} {current_time}</h5>
                <div>{"Loading..."}</div>
            </>
        }
    }
}

