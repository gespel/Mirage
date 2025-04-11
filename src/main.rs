mod plugin_host;
mod status;

#[macro_use] extern crate rocket;
use rocket::http::{ContentType, Status};
use rocket::serde::{Serialize, json::Json};
use rocket_dyn_templates::{context, Template};
use status::mirage_status::MirageStatus;
use plugin_host::plugin_host::MiragePluginHost;

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

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    let mut ph = MiragePluginHost::new();
    ph.run_plugins();

    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .mount("/status", routes![status_page])
}
