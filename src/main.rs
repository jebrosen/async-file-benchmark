use std::error::Error;
use std::future::Future;
use std::time::Duration;

fn describe_header() {
    println!("{}\t\t{}", "runtime", "time per");
}

fn describe_result(runtime: &str, time: Duration) {
    println!("{}\t{}.{:03}", runtime, time.as_secs(), time.subsec_millis());
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

// File size is verified as a sanity check
const FILE_SIZE: usize = 256 * 1024;

// This is relatively small. For comparison, the BufReader in std,
// async-std, and tokio all use a 8192-byte buffer.
const BUF_SIZE: usize = 2048;

async fn read_file_async_std() -> Result<(), Box<dyn Error>> {
    use async_std::prelude::*;

    let mut file = async_std::fs::File::open("file.dat").await?;
    let mut total_read = 0;
    let mut buf = [0; BUF_SIZE];
    loop {
        match file.read(&mut buf).await? {
            0 => break,
            n => {
                total_read += n;
            }
        }
    }

    assert!(total_read == FILE_SIZE);

    Ok(())
}

async fn read_file_tokio() -> Result<(), Box<dyn Error>> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::fs::File::open("file.dat").await?;
    let mut total_read = 0;
    let mut buf = [0; BUF_SIZE];
    loop {
        match file.read(&mut buf).await? {
            0 => break,
            n => {
                total_read += n;
            }
        }
    }

    assert!(total_read == FILE_SIZE);

    Ok(())
}

// Runs 'count' file-reading tasks with each fs implementation.
async fn run_benchmark<F, Fut>(func: F, name: Option<&str>, count: u32)
where
    F: Fn() -> Fut,
    Fut: Future<Output=Result<(), Box<dyn Error>>>,
{
    let time = time_many(func, count).await.expect("failed to time");

    if let Some(name) = name {
        describe_result(name, time);
    }
}

fn main() {
    env_logger::init();

    let mut tokio_rt = tokio::runtime::Runtime::new().unwrap();

    describe_header();

    const TASK_COUNT: u32 = 5000;

    for _ in 0..6 {
        async_std::task::block_on(run_benchmark(read_file_async_std, Some("async-std"), TASK_COUNT));
        tokio_rt.block_on(run_benchmark(read_file_tokio, Some("tokio\t"), TASK_COUNT));
    }
}
