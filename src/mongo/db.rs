use actix_web::web;
use actix_web::{web::Json, HttpResponse};
use bson::oid::ObjectId;
use futures::TryStreamExt;
use log::info;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ReqOptions {
    Vacation,
    Banked,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    email: String,
    start_date: String,
    end_date: String,
    start_time: String,
    end_time: String,
    option: ReqOptions,
}

#[derive(Debug)]
pub struct MongoRepo {
    request_col: Collection<Request>,
}

impl MongoRepo {
    pub async fn init_db() -> Self {
        // MongoDB URL
        let uri = "mongodb://localhost:27017/";

        // Create a new client and connect to the server
        let client = Client::with_uri_str(uri).await.expect("error");
        let db = client.database("timeaway");

        // Create collections using struct for specified database and collection name
        let request_collection: Collection<Request> = db.collection("requests");

        MongoRepo {
            request_col: request_collection,
        }
    }

    pub async fn get_request(&self, id: String) -> HttpResponse {
        let object_id = match ObjectId::parse_str(&id) {
            Ok(id) => id,
            Err(_) => {
                return HttpResponse::BadRequest().json(json!({"error": "Invalid ID format"}))
            }
        };
        info!("Object id -> {}", object_id);

        let filter = doc! {"_id": object_id};

        match self.request_col.find_one(filter).await {
            Ok(Some(record)) => HttpResponse::Ok().json(record),
            Ok(None) => {
                info!("Id requested -> {:?} Not found", &id);
                HttpResponse::NotFound().json(json!({"Record Not Found": id}))
            }
            Err(err) => HttpResponse::InternalServerError().json(json!({"Error":err.to_string()})),
        }
    }

    pub async fn get_all_requests(&self) -> HttpResponse {
        let mut cursor = self.request_col.find(doc! {}).await.expect("Error getting");

        let mut requests: Vec<Request> = Vec::new();

        while let Some(request) = cursor.try_next().await.expect("Error") {
            requests.push(request)
        }
        {
            info!("ALL Requests -> {:?}", &requests);
            HttpResponse::Ok().json(requests)
        }
    }

    pub async fn insert_request(&self, new_request: Json<Request>) -> HttpResponse {
        let request = new_request.clone();

        match self.request_col.insert_one(request).await {
            Ok(some) => {
                info!("Request {:?} data => {:?}", some, &new_request);
                HttpResponse::Ok().json(json!({"Record Inserted":some}))
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    pub async fn update_request(
        &self,
        id: String,
        new_request: web::Json<Request>,
    ) -> HttpResponse {
        let object_id = match ObjectId::parse_str(&id) {
            Ok(id) => id,
            Err(_) => {
                return HttpResponse::BadRequest().json(json!({"error": "Invalid ID format"}))
            }
        };

        info!("Object id -> {}", object_id);

        let filter = doc! {"_id": object_id};
        info!("Filter -> {:?}", filter);

        let updated_request = doc! {
            "$set":
            {
                "name": new_request.name.clone(),
                "email": new_request.email.clone(),
                "start_date": new_request.start_date.clone(),
                "end_date": new_request.end_date.clone(),
                "start_time": new_request.start_date.clone(),
                "end_time": new_request.end_date.clone(),
                "option": format!("{:?}", new_request.option),
            }
        };

        match self.request_col.update_one(filter, updated_request).await {
            Ok(result) => {
                if result.modified_count > 0 {
                    // Successfully updated the document
                    HttpResponse::Ok().json(json!({"status": "success", "message": "Request updated successfully"}))
                } else {
                    // Document not found or no fields were updated
                    HttpResponse::NotFound().json(json!({"status": "error", "message": "Request not found or no changes applied"}))
                }
            },
            Err(err) => {
                // Handle any errors that occurred during the update
                info!("Error updating request: {}", err);
                HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Internal server error"}))
            }
        }
    }

    pub async fn delete_request(&self, id: String) -> HttpResponse {
        let object_id = match ObjectId::parse_str(&id) {
            Ok(id) => id,
            Err(_) => {
                return HttpResponse::BadRequest().json(json!({"error": "Invalid ID format"}))
            }
        };
        info!("Object id -> {}", object_id);

        let filter = doc! {"_id": object_id};
        let result = self.request_col.delete_one(filter.clone()).await;

        info!("Record {:?} deleted? -> {:?}", filter, result);

        match result {
            Ok(msg) => {
                info!("Record deleted for {:?}", &id);
                HttpResponse::Ok().json(msg)
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }
}
