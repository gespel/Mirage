use rocket::serde::json::Json;
use rocket::State;
use crate::worker_port::worker_info::{SharedWorker, Worker, WorkerInfo};

#[post("/<request_type>", data = "<worker_request>")]
pub fn worker_port_handler(request_type: &str, worker_request: Json<Worker>, workers: &State<SharedWorker>) -> Json<WorkerInfo> {
    log::debug!("Got a worker port handler type: {} for {} with key {}", request_type, worker_request.worker_name, worker_request.key);

    match request_type {
        "hi" => {
            let mut worker_list = workers.lock().expect("worker lock poisoned");
            let mut contains_worker = false;
            for worker in worker_list.iter() {
                if worker.worker_name == worker_request.worker_name {
                    contains_worker = true
                }
            }

            if !contains_worker {
                worker_list.push(
                    Worker {
                        worker_name: worker_request.worker_name.clone(),
                        key: worker_request.key.clone()
                    }
                );
            }
            for worker in worker_list.iter() {
                log::info!("{}", worker.worker_name);
            }
        }
        _ => {

        }
    }


    let wi = WorkerInfo {
        master_name: "Mirage".to_string(),
    };
    Json(wi)
}