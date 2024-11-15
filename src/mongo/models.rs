use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReqCharge {
    Vacation,
    BankedTime,
    BankedStatTime,
    UnPaidTime,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReqStatus {
    Pending,
    Approved,
    Processed,
    Completed,
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
    charge_against: ReqCharge,
    manager: String,
    status: ReqStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub charge_against: Option<ReqCharge>,
    pub manager: Option<String>,
    pub status: Option<ReqStatus>,
}
