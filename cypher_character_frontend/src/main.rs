use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cypher_character_frontend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");
    let api_router = Router::new().route("/hello", get(hello_from_the_server));

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(hello))
        .route("/another-page", get(another_page))
        .route("/character-sheet", get(character_sheet));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .context("error creating listener")?;

    info!("router initialized, now listening on port 8000");
    axum::serve(listener, router)
        .await
        .context("error while starting server")?;

    Ok(())
}

async fn character_sheet() -> CharacterSheetTemplate {
    CharacterSheetTemplate {
        name: "Tacos".to_string(),
        pronouns: "sal/sa".to_string(),
        descriptor: "Delicious".to_string(),
        cypher_type: "Avocado".to_string(),
        focus: "Satiates the Hungry".to_string(),
        flavor: "Spicy".to_string(),
    }
}

#[derive(Template)]
#[template(path = "character-sheet.html")]
struct CharacterSheetTemplate {
    name: String,
    pronouns: String,
    descriptor: String,
    cypher_type: String,
    focus: String,
    flavor: String,
}

async fn hello_from_the_server() -> &'static str {
    "Hello!"
}

async fn another_page() -> impl IntoResponse {
    AnotherPageTemplate {}
}

#[derive(Template)]
#[template(path = "another-page.html")]
struct AnotherPageTemplate;

async fn hello() -> impl IntoResponse {
    HelloTemplate {}
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;
