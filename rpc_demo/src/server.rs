use records::recorder_server::{Recorder, RecorderServer};
use records::{RecordRequest, RecordResponse, SendStreamReq, SendStreamRes};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

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

    type SendStreamStream = ReceiverStream<Result<SendStreamRes, Status>>;

    async fn send_stream(
        &self,
        request: Request<SendStreamReq>,
    ) -> Result<Response<Self::SendStreamStream>, Status> {
        println!("request: {:#?}", request);
        let (tx, rx) = mpsc::channel(10);

        tokio::spawn(async move {
            for i in 0..10 {
                tx.send(Ok(SendStreamRes {
                    success: true,
                    message: format!("Message {}", i),
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let recorder = RecorderService::default();
    println!("Recorder listening on {}", addr);
    Server::builder()
        .add_service(RecorderServer::new(recorder))
        .serve(addr)
        .await?;
    Ok(())
}
