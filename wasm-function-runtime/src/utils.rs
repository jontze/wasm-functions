use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ErrorResponse<'a> {
    pub message: &'a str,
}
