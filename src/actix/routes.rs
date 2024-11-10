use actix_web::web::{Data, Json};
use actix_web::{get,post,App, middleware::Logger,HttpResponse,HttpServer};

use crate::mongo::db::MongoRepo;

pub async fn init_actix() -> mongodb::error::Result<()> {
    let db = MongoRepo::init_db().await;
    let db_data = Data::new(db);
    
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(Logger::default())
            .service(allrequests)
            .service(insertrequest)
    })
        .bind(("127.0.0.1", 8085))?
        .run()
        .await?;

    Ok(())
}

#[get("/api/requests")]
pub async fn allrequests(db: Data<MongoRepo>) -> HttpResponse {
    db.get_all_requests().await
}

#[post("/api/requests")]
async fn insertrequest(db: Data<MongoRepo>, request: Json<crate::mongo::db::Request>) -> HttpResponse {
   db.insert_request(request).await 
}
