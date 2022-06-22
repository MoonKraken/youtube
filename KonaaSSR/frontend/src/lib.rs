use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;
use component::blog::Blog;

pub mod component;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/:blog_id")]
    Blog { blog_id: String }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Blog { blog_id } => html! {
            <Blog blog_id={blog_id.to_owned()} />
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    return html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.url);

    return html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    };
}
