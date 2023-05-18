use records::recorder_client::RecorderClient;
use records::RecordRequest;
use tonic::Request;

pub mod records {
    tonic::include_proto!("records");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RecorderClient::connect("http://[::1]:50051").await?;
    let request = Request::new(RecordRequest {
        user_name: "test".to_string(),
        user_age: 10,
    });

    let responce = client.send_message(request).await?;
    println!("{:#?}", responce);
    println!("Meta：{:#?}", &responce.metadata());
    println!("Meta：{:#?}", &responce.get_ref());

    Ok(())
}
