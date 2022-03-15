use yew::prelude::*;
use serde_derive::Deserialize;
use reqwasm::http::Request;

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Task {
    task_uuid: String
}

async fn task_list() -> Task {
    return Request::get("api/task/22ddc70e-0743-4bcb-a23f-4d0805a27edb_c5d30858-2460-4801-b1bb-d7ab2ecae8de").send().await.unwrap().json().await.unwrap();
}

#[function_component(App)]
fn app() -> Html {

    let task = use_state(|| Task { task_uuid: "asdf".to_string() });
    {
        let task = task.clone();
        use_effect_with_deps(move |_| {
            let task = task.clone();
            wasm_bindgen_futures::spawn_local( async move {
                let fetched_task = task_list().await;
                task.set(fetched_task);
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <ul><li> {task.task_uuid.clone()} </li></ul>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}