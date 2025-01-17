use appbase::app;

use crate::plugin::tendermint::TendermintPlugin;

mod plugin;
mod types;
mod validation;
mod libs;

#[tokio::main]
async fn main() {
    env_logger::init();
    app::register_plugin::<TendermintPlugin>();
    app::initialize();
    app::startup();
    app::execute().await; // XXX: a better way for graceful shutdown?
}
