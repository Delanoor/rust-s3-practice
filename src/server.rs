use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use axum::{
    extract::Path,
    handler::Handler,
    http::{Extensions, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use crate::s3_ops::ops::S3Client;
use crate::transcribe_ops::ops::TranscribeClient;

pub async fn run_server(addr: &str) {
    let routes_all = get_routes().await;
    let addr = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("-->> Listening on {:?}", &addr);

    axum::serve(addr, routes_all.into_make_service())
        .await
        .unwrap();
}

async fn list_buckets(
    Extension(s3_client): Extension<S3Client>,
) -> Result<impl IntoResponse, StatusCode> {
    let list_buckets = s3_client.get_buckets().await;
    Ok(Json(list_buckets))
}

async fn create_bucket(
    Extension(s3_client): Extension<S3Client>,
    Path(bucket_name): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let _cfg = CreateBucketConfiguration::builder()
        .location_constraint(BucketLocationConstraint::from(
            BucketLocationConstraint::ApNortheast2,
        ))
        .build();
    let create_result = s3_client.create_bucket(&bucket_name).await;

    if create_result.is_ok() {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
#[derive(Debug, serde::Deserialize)]
struct FilePayload {
    file_name: String,
}

async fn get_obj(
    Extension(s3_client): Extension<S3Client>,
    Path(bucket_name): Path<String>,
    payload: Json<FilePayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let get_result = s3_client.get_obj(&bucket_name, &payload.file_name).await;

    if get_result.is_ok() {
        Ok(get_result.unwrap())
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn get_transcription(
    Extension(transcribe_client): Extension<TranscribeClient>,
    payload: Json<FilePayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = transcribe_client.get_transcription().await;
    if result.is_ok() {
        println!("{:?}", result);
        Ok(result.unwrap())
    } else {
        println!("{:?}", result);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn get_routes() -> Router {
    let s3_client = S3Client::new().await;
    let transcribe_client = TranscribeClient::new().await;
    Router::new()
        // .route("/s3/buckets", get(list_buckets))
        .route("/s3/buckets", get(list_buckets))
        // .route("/s3/:id", get())
        .route("/s3/buckets/:bucket_name", post(create_bucket))
        .route("/s3/buckets/:bucket_name/file", get(get_obj))
        .route("/transcribe", post(get_transcription))
        .layer(Extension(s3_client))
        .layer(Extension(transcribe_client))
}
