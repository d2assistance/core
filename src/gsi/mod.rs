use std::net::SocketAddr;

use axum::{routing::post, Router};
use serde_json::Value;

use crate::logger;

pub struct GSIServer {
    uri: String,
    state: Option<String>,
}

impl Default for GSIServer {
    fn default() -> Self {
        GSIServer {
            uri: "127.0.0.1:3000".to_owned(),
            state: None,
        }
    }
}

// #[derive]
impl GSIServer {
    #[allow(dead_code)]
    pub fn new(uri: String) -> Self {
        GSIServer { uri, state: None }
    }
}

pub async fn run(uri: String) {
    let app = Router::new().route("/", post(handle_request));

    println!("Running on http://{0}", uri);

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_request(body: String) -> () {
    let val: Value = serde_json::from_str(body.as_str()).unwrap();
    let val = serde_json::to_string_pretty(&val).unwrap();

    println!("log");

    logger::Logger::log(val);
}
