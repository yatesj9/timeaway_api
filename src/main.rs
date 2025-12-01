mod actix;
mod mongo;

use crate::actix::routes;
use crate::mongo::db::MongoRepo;
use crate::mongo::tasks::periodic_task;

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

    // Initialize DB
    info!("Initializing DB...");
    let db = MongoRepo::init_db().await;

    // Spawn task to change processed to completed
    info!("Starting Tasks...");
    tokio::spawn(periodic_task(db.clone()));

    // Initialize Actix
    info!("Initializing Actix...");
    let actix_init = routes::init_actix(db.clone()).await;
    match actix_init {
        Ok(init) => init,
        Err(err) => error!("Actix inialization error -> {}", err),
    }

    Ok(())
}
