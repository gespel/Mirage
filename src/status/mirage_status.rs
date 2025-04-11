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