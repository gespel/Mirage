mod plugin_host;
mod status;
mod worker_port;

#[macro_use]
extern crate rocket_include_static_resources;
#[macro_use] extern crate rocket;

use std::convert::Infallible;
use serde::Deserialize;
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
use rocket::form::Form;
use rocket::form::name::Key;
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

    let mut ph = MiragePluginHost::new("plugins");
    ph.run_plugins();

    log::info!("Loaded {} plugins from {:?}!", ph.num_active_plugins, ph.plugin_dir_path);
    rocket::build()
        .manage(workers)
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .mount("/status", routes![status::mirage_status::status_page])
        .mount("/workerport", routes![worker_port::worker_port::worker_port_handler])
}
