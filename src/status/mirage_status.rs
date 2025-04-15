use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MirageStatus {
    name: String,
    status: String,
    online: bool
}

impl MirageStatus {
    pub fn new(name: String, status: String, online: bool) -> Self {
        MirageStatus {
            name,
            status,
            online
        }
    }
}

#[get("/")]
pub fn status_page() -> Json<MirageStatus> {
    let j = MirageStatus::new(
        "TestServer".to_string(),
        "healthy".to_string(),
        true
    );
    Json(j)
}