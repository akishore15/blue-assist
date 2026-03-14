use serde::Serialize;

#[derive(Serialize)]
pub struct InstanceInfo {
    pub id: String,
    pub image: String,
    pub status: String,
}
