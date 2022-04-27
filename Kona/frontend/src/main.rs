use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use common::model::blog::Blog;
use common::model::post::Post;
use component::blog_view::BlogView;

pub mod component;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/:blog_id")]
    BlogView { blog_id: String }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::BlogView { blog_id } => html! {
            <BlogView blog_id={blog_id.to_owned()} />
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}