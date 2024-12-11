use std::{str::FromStr, sync::LazyLock};

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[cfg(feature = "ssr")]
use stripe::{
    CancelSubscription, CheckoutSession, CheckoutSessionMode, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CustomerId, ListSubscriptions, Subscription, SubscriptionId,
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
        <Title text="Moonmine"/>

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
    let subscribe = Action::new(|_| checkout());
    let unsubscribe = Action::new(|_| unsubscribe());

    let product = Resource::new(|| {}, |_| get_product());

    let button = move || match product.get() {
        Some(Ok((false, _))) => {
            view! { <button on:click=move |_| {subscribe.dispatch(());}>"Subscribe"</button> }
                .into_any()
        }
        Some(Ok((true, _))) => {
            view! { <button on:click=move |_| {unsubscribe.dispatch(());}>"Unsubscribe"</button> }
                .into_any()
        }
        _ => view! { <h2>"Issue getting subscription status"</h2>}.into_any(),
    };

    let product_display = move || match product.get() {
        Some(Ok((_, product))) => product,
        _ => "Issue retrieving product".to_string(),
    };

    view! {
        <h1>"Moonmine"</h1>
        // <button on:click=move |_| {subscribe.dispatch(());}>"Subscribe"</button>
        <Suspense>
            {button}
        </Suspense>

        <Suspense>
            <h1>{product_display}</h1>
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
async fn checkout() -> Result<(), ServerFnError> {
    let params = CreateCheckoutSession {
        cancel_url: Some("http://localhost:3000"),
        success_url: Some("http://localhost:3000"),
        customer: Some(CustomerId::from_str(ONLY_CUSTOMER_ID)?),
        mode: Some(CheckoutSessionMode::Subscription),
        line_items: Some(vec![CreateCheckoutSessionLineItems {
            quantity: Some(1),
            price: Some(ONLY_PRICE_ID.to_string()),
            ..Default::default()
        }]),
        ..Default::default()
    };

    let checkout_session = CheckoutSession::create(&STRIPE_CLIENT, params).await?;

    if let Some(url) = checkout_session.url {
        leptos_axum::redirect(&url);
        Ok(())
    } else {
        Err(ServerFnError::ServerError(
            "couldnt get session url".to_string(),
        ))
    }
}

#[server]
async fn unsubscribe() -> Result<(), ServerFnError> {
    // get the subscription id for the user
    let params = ListSubscriptions {
        customer: Some(CustomerId::from_str(ONLY_CUSTOMER_ID)?),
        ..Default::default()
    };

    let subscription_list = Subscription::list(&STRIPE_CLIENT, &params).await?;
    let subscription_id = match subscription_list.data.get(0) {
        Some(subscription) => &subscription.id,
        None => {
            return Err(ServerFnError::ServerError(
                "Couldn't find subscription to cancel".to_string(),
            ))
        }
    };

    let params = CancelSubscription {
        invoice_now: Some(false),
        prorate: Some(false),
        cancellation_details: None,
    };

    let _ = Subscription::cancel(
        &STRIPE_CLIENT,
        &SubscriptionId::from_str(subscription_id)?,
        params,
    )
    .await?;

    Ok(())
}

#[server]
async fn get_product() -> Result<(bool, String), ServerFnError> {
    let params = ListSubscriptions {
        customer: Some(CustomerId::from_str(ONLY_CUSTOMER_ID)?),
        ..Default::default()
    };

    let subscriptions = Subscription::list(&STRIPE_CLIENT, &params).await?.data;
    if subscriptions.is_empty() {
        // not subscribed
        Ok((false, "🪨".to_string())) // free version
    } else {
        // subscribed
        Ok((true, "💎".to_string())) // paid version
    }
}
