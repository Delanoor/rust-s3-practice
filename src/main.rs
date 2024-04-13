use aws_config::{defaults, load_defaults, BehaviorVersion};
use aws_sdk_s3 as s3;
use axum::http::StatusCode;
use axum::routing::{get, get_service, post};

use axum::extract::{Path, Query};
use axum::Json;
use axum::{response::IntoResponse, Router};
use s3::types::Bucket;
use s3::{
    config::Credentials,
    operation::{
        create_bucket::{CreateBucketError, CreateBucketOutput},
        get_object::GetObjectOutput,
        put_object::{self, PutObjectOutput},
    },
    primitives::{ByteStream, SdkBody},
    types::{BucketLocationConstraint, CreateBucketConfiguration},
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let config = load_defaults(BehaviorVersion::latest()).await;
    let s3_client = s3::Client::new(&config);



    let routes_all = Router::new().merge(get_routes());
    let addr = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("-->> Listening on {:?}", &addr);

    axum::serve(addr, routes_all.into_make_service()).await.unwrap();



    // let create_result = create_bucket(s3_client, "hwn-rust-test").await;
    // if create_result.is_ok() {
    //     println!("Bucket created successfully!")
    // } else {
    //     println!("Error occured while creating S3 bucket");
    //     println!("{:?}", create_result.err());
    // }

    // let input_string = "Hello, from the rust app".as_bytes().to_vec();
    // let content = ByteStream::from(input_string);
    // let put_object = s3_client
    //     .put_object()
    //     .bucket("saturn-blahblah")
    //     .key("ex-name-1.txt")
    //     .body(content)
    //     .send()
    //     .await;

    // let put_result = put_object(s3_client, "text_1.txt", "hwn-rust-test").await;

    // if put_result.is_ok() {
    //     println!("Successfully created S3 object!")
    // } else {
    //     println!("Failed to create S3 object")
    // }

    let get_result = get_obj(s3_client, "hwn-rust-test", "text_1.txt").await;
    if let Ok(get_output) = get_result {
        let bytes_stream = get_output.body;

        let bytes =bytes_stream.collect().await;


        match bytes {
            Ok(b) => {
                let inner_bytes = b.to_vec(); 
                match String::from_utf8(inner_bytes) {
                    Ok(text) => println!("Successfully retrieved content:\n\t{}", text),
                    Err(e) => println!("Failed to convert bytes to string: {:?}", e),
                }
            },
            Err(e) => println!("Error collecting bytes from stream: {:?}", e),
        }


    } else {
        println!("Error: {:?}", get_result);
    }
}


fn get_routes() -> Router {
    Router::new()
    .route("/s3", get(handler_s3))
    .route("/s3/buckets", get(get_buckets))
    // .route("/s3/:id", get())
    .route("/s3/:name", post(create_bucket))
 
}

#[derive(Debug, Deserialize)]
struct S3Params {
    bucket_name: Option<String>
}

async fn handler_s3(params: Query<S3Params>) -> impl IntoResponse {
    let name = params.bucket_name.as_deref().unwrap();

    println!("-->> handler s3 - {name:?}");

}

async fn get_buckets() -> impl IntoResponse {
    let config = load_defaults(BehaviorVersion::latest()).await;
    let s3_client = s3::Client::new(&config);

    let list_buckets = s3_client.list_buckets().send().await;
    let buckets = list_buckets.unwrap().buckets.unwrap();

    let mut bucket_names = vec![];
    for bucket in buckets {
        bucket_names.push(bucket.name.unwrap());
    }

    Json(bucket_names)
}

async fn create_bucket(
    Path(name): Path<String>
) -> StatusCode {
    
    let config = load_defaults(BehaviorVersion::latest()).await;
    let s3_client = s3::Client::new(&config);


    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(BucketLocationConstraint::from(
            BucketLocationConstraint::ApNortheast2,
        ))
        .build();
    let create_result = s3_client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(name)
        .send()
        .await;


        if create_result.is_ok() {
            return StatusCode::OK
        } else {
            return StatusCode::BAD_REQUEST
        }


}

async fn put_object(
    client: s3::Client,
    name: &str,
    bucket_name: &str,
) -> Result<PutObjectOutput, Box<dyn std::error::Error>> {
    let content = ByteStream::from("Hello, from the rust app".as_bytes().to_vec());
    let put_result = client
        .put_object()
        .bucket(bucket_name)
        .key(name)
        .body(content)
        .send()
        .await?;

    Ok(put_result)
}

async fn get_obj(
    client: s3::Client,
    bucket_name: &str,
    file_name: &str,
) -> Result<GetObjectOutput, Box<dyn std::error::Error>> {
    let get_result = client
        .get_object()
        .key(file_name)
        .bucket(bucket_name)
        .send()
        .await?;
    Ok(get_result)
}
