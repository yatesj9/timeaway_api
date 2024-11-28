use crate::mongo::models;
use actix_cors::Cors;
use actix_web::web;
use actix_web::web::{Data, Json, Path};
use actix_web::{
    delete, get, http, middleware::Logger, patch, post, App, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use log::info;
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::mongo::db::MongoRepo;

pub async fn init_actix(db: Arc<Mutex<MongoRepo>>) -> mongodb::error::Result<()> {
    dotenv().ok();
    dotenv::dotenv().expect("Failed to load .env file");

    let port: u16 = env::var("ACTIX_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("ACTIX_PORT must be a valid integer");

    // let db = MongoRepo::init_db().await;
    let db_data = Data::new(db);

    // Initialize the cron job for processed to completed

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "PATCH", "DELETE", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .app_data(db_data.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .service(allrequests)
            .service(singlerequest)
            .service(insertrequest)
            .service(updaterequest)
            .service(deleterequest)
            .service(requestparams)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct GetParams {
    status: String,
    limit: i32,
}

// GET/Return with /?status=#######&limit=##
#[get("/api/requests/")]
pub async fn requestparams(
    db: web::Data<Arc<Mutex<MongoRepo>>>,
    info: web::Query<GetParams>,
) -> HttpResponse {
    let db = db.lock().await;
    db.get_request_status(info.status.clone(), info.limit).await
}

// GET/Return all requests
#[get("/api/requests")]
pub async fn allrequests(db: web::Data<Arc<Mutex<MongoRepo>>>) -> HttpResponse {
    let db = db.lock().await;
    db.get_all_requests().await
}

// GET/Return single request by ID
#[get("/api/requests/{id}")]
pub async fn singlerequest(
    db: web::Data<Arc<Mutex<MongoRepo>>>,
    path: Path<String>,
) -> HttpResponse {
    let db = db.lock().await;
    db.get_request(path.into_inner()).await
}

// POST/Insert request via JSON body
#[post("/api/requests")]
async fn insertrequest(
    db: web::Data<Arc<Mutex<MongoRepo>>>,
    request: Json<models::Request>,
) -> HttpResponse {
    let db = db.lock().await;
    db.insert_request(request).await
}

// Patch/Update a request via JSON body
#[patch("/api/requests/{id}")]
async fn updaterequest(
    db: web::Data<Arc<Mutex<MongoRepo>>>,
    path: Path<String>,
    new_request: web::Json<models::UpdateRequest>,
) -> HttpResponse {
    let db = db.lock().await;
    db.update_request(path.into_inner(), new_request).await
}

// DELETE/Delete request by ID
#[delete("/api/requests/{id}")]
async fn deleterequest(db: web::Data<Arc<Mutex<MongoRepo>>>, path: Path<String>) -> HttpResponse {
    let db = db.lock().await;
    db.delete_request(path.into_inner()).await
}
