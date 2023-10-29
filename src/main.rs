
// use crate::gsi::GSIServer;

mod gsi;
mod setup;

use setup::Setup;

#[tokio::main]
async fn main() {
    Setup::create_configuration();
    // let gsi: GSIServer = Default::default();
    // gsi.run().await;
}
