use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;

const M: usize = 2;
const N: usize = 4;
fn main() -> Result<()> {
    let metrics = Metrics::new();

    println!("{:?}", metrics.snapshot());
    for idx in 0..M {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..N {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{:?}", metrics.snapshot());
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            thread::sleep(Duration::from_millis(rng.gen_range(500..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        let mut rng = rand::thread_rng();

        loop {
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..256);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
