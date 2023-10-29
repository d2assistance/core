use axum::{routing:: post, Router, body::Bytes};
use serde_json::{Result, Value};

pub struct GSIServer {
  uri: String,
}

impl Default for GSIServer {
  fn default() -> Self {
      GSIServer {
          uri: "127.0.0.1:3000".to_owned(),
      }
  }
}

impl GSIServer {
  pub fn new(uri: String) -> Self {
    GSIServer { uri }
  }

  pub async fn run(&self) {
    let app = Router::new()
        .route("/", post(Self::handle_request));

    println!("Running on http://{0}", &self.uri);

    axum::Server::bind(&self.uri.parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();

  }

  async fn handle_request(body: String) -> () {
    let val: Value = serde_json::from_str(body.as_str()).unwrap();
    let val = serde_json::to_string_pretty(&val).unwrap();

    println!("{}", val);
  }
}