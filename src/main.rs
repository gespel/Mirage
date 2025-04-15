mod plugin_host;
mod status;
mod worker_port;

#[macro_use] extern crate rocket;
use std::collections::HashMap;
use rocket_dyn_templates::{context, Template};
use plugin_host::plugin_host::MiragePluginHost;
use env_logger::Builder;
use chrono::Local;
use std::io::Write;
use std::sync::{Arc, Mutex};
use rocket::fs::{relative, FileServer};
use crate::worker_port::worker_auth::WorkerAuth;
use crate::worker_port::worker_info::{SharedWorker};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            name: "Sten"
        },
    )
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
    build_logger();

    let workers: SharedWorker = Arc::new(Mutex::new(Vec::new()));
    let worker_auth: WorkerAuth = Arc::new(Mutex::new(HashMap::new()));
    worker_auth.lock().unwrap().insert("worker1".to_string(), "asdasd".to_string());

    let mut ph = MiragePluginHost::new("plugins");
    ph.run_plugins();

    log::info!("Loaded {} plugins from {:?}!", ph.num_active_plugins, ph.plugin_dir_path);
    rocket::build()
        .attach(Template::fairing())
        .manage(workers)
        .manage(worker_auth)
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/status", routes![status::mirage_status::status_page])
        .mount("/workerport", routes![worker_port::worker_port::worker_port_handler])
}
