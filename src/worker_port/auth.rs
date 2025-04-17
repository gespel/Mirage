use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rocket::serde::json::Json;
use rocket::State;
use crate::worker_port::structs::{Worker};

pub type WorkerAuth = Arc<Mutex<HashMap<String, String>>>;

pub fn check_auth(worker_auth: &State<WorkerAuth>, worker_request: &Json<Worker>) -> bool {
    let key_list = worker_auth.lock().expect("worker lock poisoned");
    match key_list.get(&worker_request.worker_name) {
        Some(worker_key) => {
            if worker_key != &worker_request.key {
                return false;
            }
        }
        None => {
            return false;
        }
    }
    true
}