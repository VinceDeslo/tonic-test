use greeter::greeter_client::GreeterClient;
use greeter::GreetRequest;

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "http://[::1]:8080";
    let mut client = GreeterClient::connect(address).await?;

    let request = tonic::Request::new(GreetRequest {
        name: "Vince".into(),
    });

    let response = client.greet(request).await?;

    println!("RESPONSE = {:?}", response);

    Ok(())
}
