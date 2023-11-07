mod gsi;
mod logger;
mod setup;

use gsi::GSIServer;
use setup::Setup;

#[tokio::main]
async fn main() {
    match Setup::run() {
        Ok(_) => println!("Setup success"),
        Err(err) => println!("Setup error: {}", err),
    }

    gsi::run("127.0.0.1:3000".to_string()).await;
}
