
// use crate::gsi::GSIServer;

mod gsi;
mod setup;

use gsi::GSIServer;
use setup::Setup;

#[tokio::main]
async fn main() {
    Setup::run().unwrap();

    let gsi: GSIServer = Default::default();
    gsi.run().await;
}
