use axum::{
    Form, Router,
    response::{Html, IntoResponse},
    routing::{get, put},
};
use serde::Deserialize;
use std::{net::SocketAddr, sync::LazyLock};
use tokio::sync::RwLock;
use tracing::debug;

use tower_http::trace::TraceLayer;

static PERSON: LazyLock<RwLock<PersonInfo>> = LazyLock::new(|| {
    RwLock::new(PersonInfo {
        first_name: "John".to_string(),
        last_name: "Smith".to_string(),
        email: "john@companyco.com".to_string(),
    })
});

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        Router::new()
            .route("/", get(index))
            .route("/contact/1", put(contact_put))
            .route("/contact/1", get(cancel_edit))
            .route("/contact/1/edit", get(contact_edit))
            .layer(TraceLayer::new_for_http()),
    )
    .await
    .unwrap();
}

#[derive(Debug, Deserialize, Clone)]
struct PersonInfo {
    first_name: String,
    last_name: String,
    email: String,
}

fn view_only_person(person: &PersonInfo) -> String {
    format!(
        r#"
        <div hx-target="this" hx-swap="outerHTML">
            <div><label>First Name</label>: {}</div>
            <div><label>Last Name</label>: {}</div>
            <div><label>Email</label>: {}</div>
            <button hx-get="/contact/1/edit" class="btn primary">
            Click To Edit
            </button>
        </div>
        "#,
        person.first_name, person.last_name, person.email,
    )
}

async fn index() -> impl IntoResponse {
    let person = PERSON.read().await;
    Html(format!(
        r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Axum + HTMX</title>
                <script src="https://unpkg.com/htmx.org"></script>
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css">
            </head>
            <body>
            {}
            </body>
            </html>
        "#,
        view_only_person(&person)
    ))
}

async fn contact_edit() -> impl IntoResponse {
    let person = PERSON.read().await;
    Html(format!(
        r#"<form hx-put="/contact/1" hx-target="this" hx-swap="outerHTML">
          <div>
            <label>First Name</label>
            <input type="text" name="first_name" value="{}">
          </div>
          <div class="form-group">
            <label>Last Name</label>
            <input type="text" name="last_name" value="{}">
          </div>
          <div class="form-group">
            <label>Email Address</label>
            <input type="email" name="email" value="{}">
          </div>
          <button class="btn">Submit</button>
          <button class="btn" hx-get="/contact/1">Cancel</button>
        </form>"#,
        person.first_name, person.last_name, person.email,
    ))
}

// this should save the form submission
// and turn the editable form back to view-only
async fn contact_put(Form(new_person): Form<PersonInfo>) -> impl IntoResponse {
    {
        let mut person = PERSON.write().await;
        *person = new_person.clone();
    }
    Html(view_only_person(&new_person))
}

async fn cancel_edit() -> impl IntoResponse {
    let person = PERSON.read().await;
    Html(view_only_person(&person))
}
