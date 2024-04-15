mod s3_ops;
mod server;
use aws_s3_practice::configuration::get_s3_configuration;
use server::run_server;

#[tokio::main]
async fn main() {
    get_s3_configuration().expect("Failed to get S3 configuration");
    run_server().await;
}
