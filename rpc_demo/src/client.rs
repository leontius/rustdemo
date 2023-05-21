use records::recorder_client::RecorderClient;
use records::{RecordRequest, SendStreamReq};
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

    let send_stream_req = Request::new(SendStreamReq {
        user_name: "test".to_string(),
        user_age: 10,
    });
    let mut stream = client.send_stream(send_stream_req).await?.into_inner();

    while let Some(res) = stream.message().await? {
        println!("{:#?}", res);
    }

    Ok(())
}
