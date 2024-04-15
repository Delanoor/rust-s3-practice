use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use crate::s3_ops::ops::S3Client;

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
    let list_buckets = s3_client.s3_client.list_buckets().send().await;
    let buckets = list_buckets.unwrap().buckets.unwrap();

    let mut bucket_names = vec![];
    for bucket in buckets {
        bucket_names.push(bucket.name.unwrap());
    }

    Ok(Json(bucket_names))
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
    payload: Json<FilePayload>
) -> Result<impl IntoResponse, StatusCode> {
    let get_result = s3_client.get_obj(&bucket_name, &payload.file_name).await;

    if get_result.is_ok() {
        Ok(get_result.unwrap())
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn get_routes() -> Router {
    let s3_client = S3Client::new().await;
    Router::new()
        .route("/s3/buckets", get(list_buckets))
        // .route("/s3/:id", get())
        .route("/s3/buckets/:bucket_name", post(create_bucket))
        .route("/s3/buckets/:bucket_name/file", get(get_obj))
        .layer(Extension(s3_client))
}
