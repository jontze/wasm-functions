use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Manifest {
    pub(crate) name: String,
    pub(crate) method: String,
    pub(crate) path: String,
}
