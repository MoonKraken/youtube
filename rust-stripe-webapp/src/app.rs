use std::{str::FromStr, sync::LazyLock};

use eyre::Result;
use leptos::{
    either::{Either, EitherOf3},
    prelude::*,
};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[cfg(feature = "ssr")]
use stripe::{
    CheckoutSession, CheckoutSessionMode, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    Customer, CustomerId, ListSubscriptions, Subscription,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-stripe-webapp.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let count = RwSignal::new(0);
    // let on_click = move |_| *count.write() += 1;

    let subscription_status_resource = Resource::new(|| {}, |()| is_subscribed());

    let subscribe = Action::new(|_: &()| checkout());
    let unsubscribe = Action::new(|_: &()| cancel_subscription());

    let product = Resource::new(|| {}, |()| get_product());

    view! {
        <h1>"This is a paid product!"</h1>
        <Suspense>
            {move || {
                    match subscription_status_resource.get() {
                        Some(Ok(false)) => EitherOf3::A(
                            view! { <button on:click=move |_| {subscribe.dispatch(());}>"Subscribe"</button> }
                        ),
                        Some(Ok(true)) => EitherOf3::B(
                            view! { <button on:click=move |_| {unsubscribe.dispatch(());}>"Unsubscribe"</button> }
                        ),
                        _ => EitherOf3::C(view! { <h2>"Issue getting subscription status"</h2>}),
                    };
                }
            }
        </Suspense>

        <Suspense>
            <h1>{move || product.get()}</h1>
        </Suspense>
    }
}

#[cfg(feature = "ssr")]
static STRIPE_CLIENT: LazyLock<stripe::Client> = LazyLock::new(|| {
    let stripe_api_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY not set");
    stripe::Client::new(stripe_api_key)
});

#[cfg(feature = "ssr")]
const ONLY_CUSTOMER_ID: &'static str = "cus_RN0Ye4SHf9K8k1";
#[cfg(feature = "ssr")]
const ONLY_PRODUCT_ID: &'static str = "prod_RN0bO73isshroF";
#[cfg(feature = "ssr")]
const ONLY_PRICE_ID: &'static str = "price_1QUGZyJVYHxEbIII76keKhMi";

#[server]
async fn is_subscribed() -> Result<bool, ServerFnError> {
    let mut params = ListSubscriptions::new();
    params.customer = Some(CustomerId::from_str(ONLY_CUSTOMER_ID)?);
    let subscriptions = Subscription::list(&STRIPE_CLIENT, &params).await?.data;
    // TODO check for product_id?
    Ok(!subscriptions.is_empty())
}

#[server]
async fn checkout() -> Result<(), ServerFnError> {
    // first get the price id of the product
    let checkout_session = {
        let mut params = CreateCheckoutSession::new();
        params.cancel_url = Some("http://localhost:3000");
        params.success_url = Some("http://localhost:3000");
        params.customer = Some(CustomerId::from_str(ONLY_CUSTOMER_ID)?);
        params.mode = Some(CheckoutSessionMode::Subscription);
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            quantity: Some(1),
            price: Some(ONLY_PRICE_ID.to_string()),
            ..Default::default()
        }]);
        params.expand = &["line_items", "line_items.data.price.product"];

        CheckoutSession::create(&STRIPE_CLIENT, params)
            .await
            .unwrap()
    };

    let url = match checkout_session.url {
        Some(url) => leptos_axum::redirect(&url),
        _ => {
            return Err(ServerFnError::ServerError(
                "couldnt get session url".to_string(),
            ))
        }
    };
    Ok(())
}

#[server]
async fn cancel_subscription() -> Result<(), ServerFnError> {
    Ok(())
}

#[server]
async fn get_product() -> Result<String, ServerFnError> {
    match is_subscribed().await {
        Ok(true) => Ok("💎".to_string()),  // paid version
        Ok(false) => Ok("🪨".to_string()), // free version
        Err(_) => Err(ServerFnError::ServerError(
            "Issue getting product".to_string(),
        )),
    }
}
