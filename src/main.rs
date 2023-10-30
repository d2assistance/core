mod gsi;
mod setup;
mod logger;

use gsi::GSIServer;
use setup::Setup;

#[tokio::main]
async fn main() {
    Setup::run().unwrap();

    let gsi: GSIServer = Default::default();
    gsi::run("127.0.0.1:3000".to_string()).await;
}
