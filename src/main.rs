#[macro_use] extern crate rocket;
use rocket::http::{ContentType, Status};
use rocket::serde::{Serialize, json::Json};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct MirageStatus {
    name: String,
    status: String,
    online: bool
}
#[get("/")]
fn status() -> Json<MirageStatus> {
    let j = MirageStatus {
        name: "TestMirage".to_string(),
        status: "healthy".to_string(),
        online: true
    };
    Json(j)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/status", routes![status])
}
