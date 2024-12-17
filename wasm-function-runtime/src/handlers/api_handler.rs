use axum::{extract::State, response::IntoResponse, routing::post};

pub(crate) fn router<TState>() -> axum::routing::Router<TState>
where
    TState: Clone + Send + Sync + 'static,
{
    axum::Router::<TState>::new().route("/deployV2", post(deploy_function_with_manifest))
}

async fn deploy_function_with_manifest(
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.expect("Failed to read file") {
        let name = field.name().expect("Failed to get field name");
        let file_name = field.file_name().expect("Failed to get file name");
        let content_type = field.content_type().expect("Failed to get content type");
        dbg!(name, file_name, content_type);

        let data = field.bytes().await.expect("Failed to read field");
    }

    todo!("Implement function");
}
