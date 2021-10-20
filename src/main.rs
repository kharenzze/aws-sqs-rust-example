use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), sqs::Error> {
    println!("Hello, world!");
    return Ok(())
}
