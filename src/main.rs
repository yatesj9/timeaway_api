mod actix;
mod mongo;

use crate::actix::routes;

// Logging
use log::{error, info};
use log4rs::{self};
use std::process;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Enable the log4rs logging
    match log4rs::init_file("log4rs.yaml", Default::default()) {
        Ok(_) => info!("Logger initialized successfully"),
        Err(e) => {
            println!("Failed to initialize logger: {}", e);
            process::exit(1)
        }
    };

    // Initialize Actix
    info!("Initializing Actix...");
    let actix_init = routes::init_actix().await;
    match actix_init {
        Ok(init) => init,
        Err(err) => error!("Actix inialization error -> {}", err),
    }

    Ok(())
}
