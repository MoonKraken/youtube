use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use common::model::task::Task;

async fn get_task(id: &String) -> Task {
    let url = format!("/api/task/{}", id);
    return Request::get(&url).send().await.unwrap().json().await.unwrap();
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/task/:task_global_id")]
    TaskView { task_global_id: String }
}

#[derive(Properties, Clone, PartialEq)]
pub struct TaskViewProps {
    pub task_global_id: String
}

#[function_component(TaskView)]
fn task_view(props: &TaskViewProps) -> Html {

    let task_global_id = props.task_global_id.clone();
    let task = use_state(|| None);
    {
        let task = task.clone();
        use_effect_with_deps(move |_| {
            let task = task.clone();
            wasm_bindgen_futures::spawn_local( async move {
                let fetched_task = get_task(&task_global_id).await;
                task.set(Some(fetched_task));
            });
            || ()
        }, ());
    }

    if let Some(task) = &*task {
        let result_file = match &task.result_file {
            Some(result_file) => result_file,
            None => "None"
        };

        return html! {
            <>
                <table>
                    <tr>
                        <td>{"User ID"}</td>
                        <td>{task.user_uuid.clone()}</td>
                    </tr>
                    <tr>
                        <td>{"Task ID"}</td>
                        <td>{task.task_uuid.clone()}</td>
                    </tr>
                    <tr>
                        <td>{"Task Type"}</td>
                        <td>{task.task_type.clone()}</td>
                    </tr>
                    <tr>
                        <td>{"State"}</td>
                        <td>{task.state.clone()}</td>
                    </tr>
                    <tr>
                        <td>{"Source File"}</td>
                        <td>{task.source_file.clone()}</td>
                    </tr>
                    <tr>
                        <td>{"Result File"}</td>
                        <td>{result_file}</td>
                    </tr>
                </table>
            </>
        }
    } else {
        return html! { <div>{"Loading..."}</div> }
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::TaskView { task_global_id } => html! {
            <TaskView task_global_id={task_global_id.to_owned()} />
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