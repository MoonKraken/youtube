use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::journal::{CreateEntry, GetEntry, GetEntries};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let get_entry = create_server_action::<GetEntry>(cx);
    let create_entry = create_server_action::<CreateEntry>(cx);
    let get_entries = create_server_action::<GetEntries>(cx);

    view! { cx,
        <ActionForm action=create_entry>
            <h1>"Put Entry"</h1>
            <label>
                "UserID"
                <input type="text" name="userid"/>
            </label>
            <label>
                "Content"
                <input type="text" name="content"/>
            </label>
            <label>
                "Date"
                <input type="text" name="date"/>
            </label>
            <input type="submit" value="Set Entry"/>
        </ActionForm>
        <hr/>
        <ActionForm action=get_entry>
            <h1>"Get Entry"</h1>
            <label>
                "UserID"
                <input type="text" name="userid"/>
            </label>
            <label>
                "Get Date"
                <input type="text" name="date"/>
            </label>
            <input type="submit" value="Get Entry"/>
        </ActionForm>
        <hr/>
        <ActionForm action=get_entries>
            <h1>"Get Entries"</h1>
            <label>
                "UserID"
                <input type="text" name="userid"/>
            </label>
            <label>
                "Start Date"
                <input type="text" name="start_date"/>
            </label>
            <label>
                "End Date"
                <input type="text" name="end_date"/>
            </label>
            <input type="submit" value="Get Entries"/>
        </ActionForm>
    }
}
