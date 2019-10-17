// async-std = "=0.99.9"
// futures-preview = "=0.3.0-alpha.19"
// tokio = "=0.2.0-alpha.6"
// env_logger = "0.6"


use std::error::Error;
use std::future::Future;
use std::time::Duration;

fn describe_header() {
    println!("{}\t{}\t{}", "runtime", "fs", "time per");
}

fn describe_result(runtime: &str, fs: &str, time: Duration) {
    println!("{}\t{}\t{}.{:03}", runtime, fs, time.as_secs(), time.subsec_millis());
}

// Runs the future returned by 'f()' 'count' times, in parallel.
// Returns the mean time elapsed.
async fn time_many<F, Fut, E>(f: F, count: u32) -> Result<Duration, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output=Result<(), E>>,
{
    let times = futures::future::try_join_all((0..count).map(|_|
        async {
            let start = std::time::Instant::now();
            f().await?;
            Ok(start.elapsed())
        }
    )).await?;

    Ok(times.iter().sum::<Duration>().checked_div(count).expect("count > 0"))
}

const BUF_SIZE: usize = 1024;

async fn discard<T>(_x: T) {
    // TODO: this should probably be a black-box
}

async fn read_file_async_std() -> Result<(), Box<dyn Error>> {
    use async_std::prelude::*;

    let mut file = async_std::fs::File::open("file.dat").await?;
    loop {
        let mut buf = vec![0; BUF_SIZE];
        match file.read(&mut buf).await {
            Ok(n) if n == 0 => break,
            Ok(n) => {
                buf.truncate(n);
                discard(buf).await;
            }
            Err(e) => return Err(e)?,
        }
    }

    Ok(())
}

async fn read_file_tokio() -> Result<(), Box<dyn Error>> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::fs::File::open("file.dat").await?;
    loop {
        let mut buf = vec![0; BUF_SIZE];
        match file.read(&mut buf).await {
            Ok(n) if n == 0 => break,
            Ok(n) => {
                buf.truncate(n);
                discard(buf).await;
            }
            Err(e) => return Err(e)?,
        }
    }

    Ok(())
}

// Runs 'count' file-reading tasks with each fs implementation.
async fn run_benchmark(runtime_name: Option<&str>, count: u32) {
    let time_tokio = time_many(read_file_tokio, count).await.expect("failed to time tokio file");
    let time_async_std = time_many(read_file_async_std, count).await.expect("failed to time async_std file");

    if let Some(runtime) = runtime_name {
        describe_result(runtime, "tokio", time_tokio);
        describe_result(runtime, "a_std", time_async_std);
    }
}

fn main() {
    env_logger::init();

    let tokio_rt = tokio::runtime::Runtime::new().unwrap();

    // Warm up
    tokio_rt.block_on(run_benchmark(None, 100));
    async_std::task::block_on(run_benchmark(None, 100));

    describe_header();

    // Real thing
    tokio_rt.block_on(run_benchmark(Some("tokio"), 5000));
    async_std::task::block_on(run_benchmark(Some("a_std"), 5000));
}
