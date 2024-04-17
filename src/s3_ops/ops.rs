use std::{str::from_utf8, sync::Arc};

use aws_config::BehaviorVersion;
use aws_sdk_s3 as s3;
use axum::{ http::StatusCode, response::IntoResponse, Extension, Json};
use s3::{
    operation::{get_object::GetObjectOutput, put_object::PutObjectOutput},
    primitives::ByteStream,
    types::{BucketLocationConstraint, CreateBucketConfiguration},
};



#[derive(Clone)]
pub struct S3Client {
    pub s3_client: Arc<s3::Client>,
}
impl S3Client {
    pub async fn new() -> Self {
        // let config = load_defaults(BehaviorVersion::latest()).await;
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    
        let s3_client = s3::Client::new(&config);
        Self {
            s3_client: Arc::new(s3_client),
        }
    }

    pub async fn get_buckets(&self) -> Vec<String> {
        let list_buckets = self.s3_client.list_buckets().send().await;
        let buckets = list_buckets.unwrap().buckets.unwrap();

        let mut bucket_names = vec![];
        for bucket in buckets {
            bucket_names.push(bucket.name.unwrap());
        }

        bucket_names
    }
 
    pub async fn create_bucket(&self, name: &str) -> Result<impl IntoResponse, StatusCode> {
        let cfg = CreateBucketConfiguration::builder()
            .location_constraint(BucketLocationConstraint::from(
                BucketLocationConstraint::ApNortheast2,
            ))
            .build();
        let create_result = self
            .s3_client
            .create_bucket()
            .create_bucket_configuration(cfg)
            .bucket(name)
            .send()
            .await;

        if create_result.is_ok() {
            return Ok(StatusCode::OK);
        } else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    pub async fn put_object(
        &self,
        name: &str,
        bucket_name: &str,
    ) -> Result<PutObjectOutput, Box<dyn std::error::Error>> {
        let content = ByteStream::from("Hello, from the rust app".as_bytes().to_vec());
        let put_result = self
            .s3_client
            .put_object()
            .bucket(bucket_name)
            .key(name)
            .body(content)
            .send()
            .await?;

        Ok(put_result)
    }

    pub async fn get_obj(
        &self,
        bucket_name: &str,
        file_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let get_result = self
            .s3_client
            .get_object()
            .key(file_name)
            .bucket(bucket_name)
            .send()
            .await?;

            let bytes = get_result.body.collect().await?.into_bytes();
            let text_content = from_utf8(&bytes)?;
            println!("{:?}", text_content);
        Ok(text_content.to_string())
    }
}

