use std::{borrow::Borrow, sync::Arc};

use anyhow::Context;
use askama::Template;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post, put},
    Form, Router,
};
use cypher_character_model::Character;
use cypher_character_model::Sentence;
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug)]
struct AppState {
    character: Mutex<Character>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cypher_character_frontend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = Arc::new(AppState {
        character: Mutex::new(Character {
            name: "Tacos".to_string(),
            pronouns: "yum/my".to_string(),
            sentence: Sentence {
                descriptor: "Delicious".to_string(),
                character_type: "Avocado".to_string(),
                focus: "Satiates the Hungry".to_string(),
                flavor: Some("Spicy".to_string()),
            },
        }),
    });

    info!("initializing router...");
    let api_router = Router::new().route("/v1/character", put(update_character));

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(character_sheet))
        .route("/character", get(character_sheet))
        .route("/character/edit", get(character_edit))
        .route("/character/view", get(character_view))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .context("error creating listener")?;

    info!("router initialized, now listening on port 8000");
    axum::serve(listener, router)
        .await
        .context("error while starting server")?;

    Ok(())
}

#[derive(Template)]
#[template(path = "character/sheet.html")]
struct CharacterSheetTemplate {
    character: Character,
}

async fn character_sheet(State(state): State<Arc<AppState>>) -> CharacterSheetTemplate {
    CharacterSheetTemplate {
        character: state.character.lock().await.clone(),
    }
}

#[derive(Template)]
#[template(path = "character/edit.html")]
struct CharacterEditTemplate {
    character: Character,
}

async fn character_edit(State(state): State<Arc<AppState>>) -> CharacterEditTemplate {
    CharacterEditTemplate {
        character: state.character.lock().await.clone(),
    }
}

#[derive(Template)]
#[template(path = "character/view.html")]
struct CharacterViewTemplate {
    character: Character,
}

async fn character_view(State(state): State<Arc<AppState>>) -> CharacterViewTemplate {
    CharacterViewTemplate {
        character: state.character.lock().await.clone(),
    }
}

#[derive(Deserialize)]
struct CharacterRequest {
    name: Option<String>,
    pronouns: Option<String>,
    descriptor: Option<String>,
    character_type: Option<String>,
    flavor: Option<String>,
    focus: Option<String>,
}

async fn update_character(
    State(state): State<Arc<AppState>>,
    Form(form): Form<CharacterRequest>,
) -> impl IntoResponse {
    let mut lock = state.character.lock().await;

    if let Some(name) = form.name {
        lock.name = name;
    }

    if let Some(pronouns) = form.pronouns {
        lock.pronouns = pronouns;
    }

    if let Some(descriptor) = form.descriptor {
        lock.sentence.descriptor = descriptor;
    }

    if let Some(character_type) = form.character_type {
        lock.sentence.character_type = character_type;
    }

    if let Some(flavor) = form.flavor {
        lock.sentence.flavor = Some(flavor);
    }

    if let Some(focus) = form.focus {
        lock.sentence.focus = focus;
    }

    [("HX-Trigger", "updatedCharacter")]
}
