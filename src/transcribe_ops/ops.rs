use std::{str::from_utf8, sync::Arc};

use aws_config::BehaviorVersion;
use aws_sdk_transcribe as transcribe;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use transcribe::types::{LanguageCode, Media, Transcript, TranscriptionJob};

#[derive(Clone)]
pub struct TranscribeClient {
    pub client: Arc<transcribe::Client>,
}

impl TranscribeClient {
    pub async fn new() -> Self {
        // let config = load_defaults(BehaviorVersion::latest()).await;
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

        let client = aws_sdk_transcribe::Client::new(&config);
        Self {
            client: Arc::new(client),
        }
    }
    pub async fn get_transcription(&self) -> Result<String, Box<dyn std::error::Error>> {
        const FILE_URI: &str = "https://hwn-rust-test.s3.ap-northeast-2.amazonaws.com/ed0cef11-4493-4bba-87d4-5dffbf1725f1.webm";
        let media = Some(Media::builder().media_file_uri(FILE_URI).build());

        let transcription_result = self
            .client
            .start_transcription_job()
            .transcription_job_name("testing")
            .set_media(media)
            .set_language_code(Some(LanguageCode::EnUs))
            .send()
            .await?;
        println!("result: {:?}", transcription_result);

        let content = transcription_result.transcription_job.unwrap().media;

        println!("{:?}", content);
        if let Some(res) = content {
            Ok(res.media_file_uri.unwrap())
        } else {
            Err("Error occured".into())
        }
    }
}
