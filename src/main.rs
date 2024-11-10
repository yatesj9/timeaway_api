mod actix;
mod mongo;

use crate::actix::routes;

// Logging
use log::{error, info};
use log4rs::{self};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Enable the log4rs logging
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Initializing Logging...");

    // Initialize Actix
    info!("Initializing Actix...");
    let actix_init = routes::init_actix().await;
    match actix_init {
        Ok(init) => init,
        Err(err) => error!("Actix inialization error -> {}", err),
    }

    Ok(())
}
