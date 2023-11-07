use std::sync::{Arc, RwLock};

use axum::{routing::{post, get}, Router, extract::State};

#[derive(Default, Clone)]
pub struct GSIServer {
    state: String,
}

type SharedState = Arc<RwLock<GSIServer>>;

pub async fn run(uri: String) {
    let shared_state = SharedState::default();

    let app =
        Router::new()
            .route("/", post(handle_set_state))
            .route("/state", get(handle_get_state))
            .with_state(Arc::clone(&shared_state));

    println!("Running on http://{0}", uri);

    axum::Server::bind(&uri.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_set_state(State(state): State<SharedState>, body: String) {
    println!("handle_set_state");
    state.write().unwrap().state = body;
}

async fn handle_get_state(State(state): State<SharedState>) -> String {
    println!("handle_get_state");

    return state.read().unwrap().state.clone();
}
