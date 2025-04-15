use std::sync::{Arc, Mutex};
use rocket::serde::{Deserialize, Serialize};

pub type SharedWorker = Arc<Mutex<Vec<Worker>>>;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct WorkerInfo {
    pub(crate) master_name: String,
}

#[derive(FromForm, Deserialize)]
pub struct Worker {
    pub(crate) worker_name: String,
    pub(crate) key: String,
}