mod plugin_host;
mod status;
mod worker_port;

#[macro_use]
extern crate rocket_include_static_resources;
#[macro_use] extern crate rocket;

use std::convert::Infallible;
use rocket::serde::{json::Json};
use rocket_dyn_templates::{context, Template};

use status::mirage_status::MirageStatus;
use plugin_host::plugin_host::MiragePluginHost;
use worker_port::worker_info::WorkerInfo;

use env_logger::Builder;
use chrono::Local;
use std::io::Write;
use std::sync::{Arc, Mutex};
use rocket::fs::{relative, FileServer};
use rocket::{request, Request, State};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::Responder;
use crate::worker_port::worker_info::{SharedWorker, Worker};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            name: "Sten"
        },
    )
}

#[get("/<worker_name>/<key>/<request_type>")]
fn worker_port_handler(key: &str, worker_name: &str, request_type: &str, workers: &State<SharedWorker>) -> Json<WorkerInfo> {
    log::debug!("Got a worker port handler for key: {}, name: {}, type: {}", key, worker_name, request_type);
    match request_type {
        "hi" => {

            let mut client_list = workers.lock().expect("client list poisoned");

            let mut contains_worker = false;
            for c in client_list.iter() {
                if c.worker_name == worker_name {
                    contains_worker = true;
                }
            }
            if !contains_worker {
                log::info!("New worker said hi!");
                client_list.push(Worker {worker_name: worker_name.to_string(), key: key.to_string()})
            }
            else {
                log::info!("Worker is already known...");
            }

        }
        _ => {
            log::warn!("Worker request unknown!");
        }
    }

    let mut client_list = workers.lock().expect("client list poisoned");
    let mut nr = 0;
    for client in client_list.iter() {
        log::info!("{} -> {:?}", nr, client);
        nr = nr + 1;
    }

    let wi = WorkerInfo {
        master_name: "Mirage".to_string(),
    };
    Json(wi)
}

#[get("/")]
fn status_page() -> Json<MirageStatus> {
    let j = MirageStatus::new(
        "TestServer".to_string(),
        "healthy".to_string(),
        true
    );
    Json(j)
}

fn build_logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}[{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        }).filter(None, log::LevelFilter::Info).init();
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    let workers: SharedWorker = Arc::new(Mutex::new(Vec::new()));
    build_logger();
    let mut ph = MiragePluginHost::new("plugins");
    ph.run_plugins();
    log::info!("Loaded {} plugins from {:?}!", ph.num_active_plugins, ph.plugin_dir_path);
    rocket::build()
        .manage(workers)
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .mount("/status", routes![status_page])
        .mount("/workerport", routes![worker_port_handler])
}
