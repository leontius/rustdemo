use tokio::sync::mpsc;

static NTHREADS: i32 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(32);

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();

        tokio::spawn(async move {
            thread_tx.send(id).await;

            println!("thread {} sent", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv().await);
    }

    println!("{:?}", ids);

    Ok(())
}
