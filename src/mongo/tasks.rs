use log::info;

use chrono::{Duration, Local, Timelike};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep_until, Instant};

use crate::mongo::db::MongoRepo;

pub async fn periodic_task(db: Arc<Mutex<MongoRepo>>) {
    loop {
        // Get the current time
        let now = Local::now().naive_local();
        let tomorrow = now + Duration::days(1);
        let midnight = tomorrow
            .with_hour(0)
            .and_then(|dt| dt.with_minute(0))
            .and_then(|dt| dt.with_second(0))
            .unwrap_or(tomorrow);
        let duration_until_midnight = (midnight - now).to_std().unwrap();
        let sleep_until_midnight = Instant::now() + duration_until_midnight;

        // info!("Now {}", now);
        // info!("tomorrow {:?}", tomorrow);
        // info!("Midnight {:?}", midnight);
        info!("Duration until Midnight {:?}", duration_until_midnight);

        sleep_until(sleep_until_midnight).await;
        // let duration = Duration::new(10,0);
        // sleep(duration).await;

        let db = db.lock().await;
        db.check_and_update_request().await;
    }
}
