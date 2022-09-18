use tonic::{transport::Server, Request, Response, Status};
use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{GreetRequest, GreetResponse };

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[derive(Debug, Default)]
pub struct GreeterService {}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn greet(
        &self, 
        req: Request<GreetRequest>
    ) -> Result<Response<GreetResponse>, Status> {
        println!("Got a request: {:?}", req);
        let r = req.into_inner();
        match r.name.as_str() {
            "Vince" => Ok(Response::new(greeter::GreetResponse {
                greeting: format!("Fuck you {}", r.name),
            })),
            _ => Ok(Response::new(greeter::GreetResponse {
                greeting: format!("Hello there {}", r.name),
            }))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let greeter_service = GreeterService::default();
    let server = GreeterServer::new(greeter_service);

    Server::builder()
        .add_service(server)
        .serve(address)
        .await?;

    Ok(())
}