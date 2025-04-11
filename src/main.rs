mod plugin_host;
mod status;

#[macro_use] extern crate rocket;
use rocket::http::{ContentType, Status};
use rocket::serde::{Serialize, json::Json};
use rocket_dyn_templates::{context, Template};
use status::mirage_status::MirageStatus;
use plugin_host::plugin_host::MiragePluginHost;
use env_logger::Builder;
use chrono::Local;
use log::LevelFilter;
use std::io::Write;

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            name: "Sten"
        },
    )
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
    let mut builder = Builder::new()
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
    let mut ph = MiragePluginHost::new("plugins");
    ph.run_plugins();
    log::info!("Loaded {} plugins!", ph.num_active_plugins);
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .mount("/status", routes![status_page])
}
