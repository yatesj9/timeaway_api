use actix_web::web;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, middleware::Logger, patch, post, App, HttpResponse, HttpServer};

use crate::mongo::db::MongoRepo;

pub async fn init_actix() -> mongodb::error::Result<()> {
    let db = MongoRepo::init_db().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(Logger::default())
            .service(allrequests)
            .service(singlerequest)
            .service(insertrequest)
            .service(updaterequest)
            .service(deleterequest)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await?;

    Ok(())
}

// GET/Return all requests
#[get("/api/requests")]
pub async fn allrequests(db: Data<MongoRepo>) -> HttpResponse {
    db.get_all_requests().await
}

// GET/Return single request by ID
#[get("/api/requests/{id}")]
pub async fn singlerequest(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    db.get_request(path.into_inner()).await
}

// POST/Insert request via JSON body
#[post("/api/requests")]
async fn insertrequest(
    db: Data<MongoRepo>,
    request: Json<crate::mongo::db::Request>,
) -> HttpResponse {
    db.insert_request(request).await
}

#[patch("/api/requests/{id}")]
async fn updaterequest(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_request: web::Json<crate::mongo::db::Request>,
) -> HttpResponse {
    db.update_request(path.into_inner(), new_request).await
}

// DELETE/Delete request by ID
#[delete("/api/requests/{id}")]
async fn deleterequest(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    db.delete_request(path.into_inner()).await
}
