use std::sync::{Arc, Mutex};
use rocket::serde::{Deserialize, Serialize};

pub type SharedWorker = Arc<Mutex<Vec<Worker>>>;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct WorkerInfo {
    pub(crate) master_name: String,
    pub return_status: String,
}

#[derive(FromForm, Deserialize)]
pub struct Worker {
    pub(crate) worker_name: String,
    pub(crate) key: String,
}