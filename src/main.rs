use tokio;

mod s3_ops;
mod server;
use server::run_server;

#[tokio::main]
async fn main() {
    run_server().await;

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

    // let get_result = get_obj(s3_client, "hwn-rust-test", "text_1.txt").await;
    // if let Ok(get_output) = get_result {
    //     let bytes_stream = get_output.body;

    //     let bytes =bytes_stream.collect().await;

    //     match bytes {
    //         Ok(b) => {
    //             let inner_bytes = b.to_vec();
    //             match String::from_utf8(inner_bytes) {
    //                 Ok(text) => println!("Successfully retrieved content:\n\t{}", text),
    //                 Err(e) => println!("Failed to convert bytes to string: {:?}", e),
    //             }
    //         },
    //         Err(e) => println!("Error collecting bytes from stream: {:?}", e),
    //     }

    // } else {
    //     println!("Error: {:?}", get_result);
    // }
}
