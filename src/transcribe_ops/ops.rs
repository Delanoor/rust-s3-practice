use aws_config::BehaviorVersion;
use aws_sdk_transcribe as transcribe;

use tokio::time;
use uuid::Uuid;

use std::sync::Arc;
use transcribe::types::{LanguageCode, Media, TranscriptionJobStatus};

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
    pub async fn get_transcription(
        &self,
        file_uri: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // const FILE_URI: &str = "https://hwn-rust-test.s3.ap-northeast-2.amazonaws.com/ed0cef11-4493-4bba-87d4-5dffbf1725f1.webm";
        let media = Some(Media::builder().media_file_uri(file_uri).build());

        //let time: DateTime<Utc> = std::time::SystemTime::now().into();
        //let job_name = format!("test_{time}");

        let job_name = Uuid::new_v4().to_string();
        self
            .client
            .start_transcription_job()
            .transcription_job_name(&job_name)
            .set_media(media)
            .set_language_code(Some(LanguageCode::EnUs))
            .send()
            .await?;
        //println!("result: {:?}", transcription_result);

        let mut transcription_res = self
            .client
            .get_transcription_job()
            .transcription_job_name(&job_name)
            .send()
            .await?;

        let mut in_progress = transcription_res
            .clone()
            .transcription_job
            .unwrap()
            .transcription_job_status
            .unwrap()
            == TranscriptionJobStatus::InProgress;

        while in_progress {
            std::thread::sleep(time::Duration::from_secs(2));

            let new_res = self
                .client
                .get_transcription_job()
                .transcription_job_name(&job_name)
                .send()
                .await?;


            in_progress = new_res.clone()
                .transcription_job
                .unwrap()
                .transcription_job_status
                .unwrap()   == TranscriptionJobStatus::InProgress;
              


                println!("Currently: {:?}", new_res.clone());
                transcription_res = new_res;
        }

        // println!("Currently: {:?}", transcription_res);

        let content = transcription_res.transcription_job.unwrap().transcript.unwrap().transcript_file_uri;

        if let Some(res) = content {
            Ok(res)
        } else {
            Err("Error occured".into())
        }
    }
}
