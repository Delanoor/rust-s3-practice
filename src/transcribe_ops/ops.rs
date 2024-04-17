use std::{str::from_utf8, sync::Arc};

use aws_config::BehaviorVersion;
use aws_sdk_transcribe as transcribe;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

#[derive(Clone)]
pub struct TranscribeClient {
    pub client: Arc<transcribe::Client>,
}
impl S3Client {
    pub async fn new() -> Self {
        // let config = load_defaults(BehaviorVersion::latest()).await;
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

        let client = aws_sdk_transcribe::Client::new(&config);
        Self {
            client: Arc::new(client),
        }
    }
}
