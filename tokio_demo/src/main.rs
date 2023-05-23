use std::{
    thread,
    time::{self, Duration},
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let block_task = tokio::task::spawn_blocking(|| loop {
        println!("I'm a blocking task!");
        thread::sleep(time::Duration::from_secs(1));
    });

    block_task.await?;
    Ok(())
}
