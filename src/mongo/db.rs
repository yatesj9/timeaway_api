use actix_web::web;
use actix_web::{web::Json, HttpResponse};
use bson::oid::ObjectId;
use chrono::{NaiveDate, Utc};
use dotenv::dotenv;
use futures::TryStreamExt;
use log::info;
use mongodb::{bson::doc, Client, Collection};
use serde_json::json;
use std::{env, sync::Arc};
use tokio::sync::Mutex;

use crate::mongo::models;

#[derive(Debug)]
pub struct MongoRepo {
    request_col: Collection<models::Request>,
}

impl MongoRepo {
    pub async fn init_db() -> Arc<Mutex<Self>> {
        dotenv().ok();
        dotenv::dotenv().expect("Failed to load .env file");

        // MongoDB URL, DB & Collection
        let uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let db_collection =
            env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION must be set");

        // Create a new client and connect to the server
        let client = Client::with_uri_str(uri).await.expect("error");
        let db = client.database(&db_name);

        // Create collections using struct for specified database and collection name
        let request_collection: Collection<models::Request> = db.collection(&db_collection);

        // MongoRepo {
        //     request_col: request_collection,
        // }
        Arc::new(Mutex::new(MongoRepo {
            request_col: request_collection,
        }))
    }

    pub async fn check_and_update_request(&self) {
        info!("Checked");
        let today = Utc::now().date_naive();

        let mut cursor = self
            .request_col
            .find(doc! {"status":"Processed"})
            .await
            .expect("Error getting processed requests");

        let mut requests: Vec<models::Request> = Vec::new();

        while let Some(request) = cursor.try_next().await.expect("Error") {
            requests.push(request)
        }

        for request in requests {
            // Assuming 'request._id' is the identifier for the request
            let stored_date = NaiveDate::parse_from_str(&request.end_date, "%m/%d/%Y").unwrap();

            if today > stored_date {
                let update_result = self
                    .request_col
                    .update_one(
                        doc! { "_id": request.id },                 // Filter to match the request
                        doc! { "$set": { "status": "Completed" } }, // Update operation
                    )
                    .await;
                match update_result {
                    Ok(_) => info!(
                        "Updated request with ID {:?} to status 'Completed'",
                        request.id
                    ),
                    Err(e) => eprintln!("Failed to update request with ID {:?}: {}", request.id, e),
                }
            } else {
                info!(
                    "Request with ID {:?} has not yet reached the end date.",
                    request.id
                );
            }
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
        info!("Object created at -> {}", object_id.timestamp());

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

        let mut requests: Vec<models::Request> = Vec::new();

        while let Some(request) = cursor.try_next().await.expect("Error") {
            requests.push(request)
        }
        {
            info!("ALL Requests -> {:?}", &requests);
            HttpResponse::Ok().json(requests)
        }
    }

    pub async fn insert_request(&self, new_request: Json<models::Request>) -> HttpResponse {
        let request = new_request.clone();

        match self.request_col.insert_one(request).await {
            Ok(some) => {
                info!("Request {:?} data => {:?}", some, &new_request);
                HttpResponse::Ok().json(json!({"Record Inserted":some}))
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    fn insert_if_some<T>(doc: &mut bson::Document, key: &str, value: Option<T>)
    where
        T: Into<mongodb::bson::Bson>,
    {
        if let Some(value) = value {
            doc.insert(key, value.into());
        }
    }

    pub async fn update_request(
        &self,
        id: String,
        new_request: web::Json<models::UpdateRequest>,
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

        let mut update_doc = doc! {};

        if let Some(charge_against) = &new_request.charge_against {
            let charge_against_str = format!("{:?}", charge_against);
            Self::insert_if_some(&mut update_doc, "charge_against", Some(charge_against_str));
        }

        if let Some(status) = &new_request.status {
            let status_str = format!("{:?}", status);
            Self::insert_if_some(&mut update_doc, "status", Some(status_str));
        }

        Self::insert_if_some(&mut update_doc, "name", new_request.name.clone());
        Self::insert_if_some(&mut update_doc, "email", new_request.email.clone());
        Self::insert_if_some(
            &mut update_doc,
            "start_date",
            new_request.start_date.clone(),
        );
        Self::insert_if_some(&mut update_doc, "end_date", new_request.end_date.clone());
        Self::insert_if_some(
            &mut update_doc,
            "start_time",
            new_request.start_time.clone(),
        );
        Self::insert_if_some(&mut update_doc, "end_time", new_request.end_time.clone());
        Self::insert_if_some(&mut update_doc, "manager", new_request.manager.clone());

        match self
            .request_col
            .update_one(filter, doc! {"$set":update_doc})
            .await
        {
            Ok(result) => {
                if result.modified_count > 0 {
                    // Successfully updated the document
                    HttpResponse::Ok().json(
                        json!({"status": "success", "message": "Request updated successfully"}),
                    )
                } else {
                    // Document not found or no fields were updated
                    HttpResponse::NotFound().json(json!({"status": "error", "message": "Request not found or no changes applied"}))
                }
            }
            Err(err) => {
                // Handle any errors that occurred during the update
                info!("Error updating request: {}", err);
                HttpResponse::InternalServerError()
                    .json(json!({"status": "error", "message": "Internal server error"}))
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
