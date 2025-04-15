use std::sync::{Arc, Mutex};
use rocket::serde::Serialize;

pub type SharedWorker = Arc<Mutex<Vec<Worker>>>;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct WorkerInfo {
    pub(crate) master_name: String,
}

#[derive(Debug)]
pub struct Worker {
    pub(crate) worker_name: String,
    pub(crate) key: String,
}