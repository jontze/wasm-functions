use uuid::Uuid;

pub(crate) struct Service {
    pub uuid: Uuid,
    pub address: String,
    pub status: String,
    pub last_heartbeat: String,
    joined_at: String,
}
