use std::net::SocketAddr;

use records::recorder_server::{Recorder, RecorderServer};
use records::{RecordRequest, RecordResponse};
use tonic::transport::Server;
use tonic::Status;
use tonic::{Request, Response};

pub mod records {
    tonic::include_proto!("records");
}

#[derive(Debug, Default)]
pub struct RecorderService {}

#[tonic::async_trait]
impl Recorder for RecorderService {
    async fn send_message(
        &self,
        request: Request<RecordRequest>,
    ) -> Result<Response<RecordResponse>, Status> {
        println!("Received request: {:#?}", request);
        let req = request.into_inner();
        let res = RecordResponse {
            success: true,
            message: format!("User {} is {} old!", req.user_name, req.user_age),
        };
        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let recorder = RecorderService::default();
    println!("Recorder listening on {}", addr);
    Server::builder()
        .add_service(RecorderServer::new(recorder))
        .serve(addr)
        .await?;
    Ok(())
}
