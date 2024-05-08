use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::AmapMetrics;
use rand::Rng;

const M: usize = 2;
const N: usize = 4;
fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
    ]);

    println!("{}", metrics);
    for idx in 0..M {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..N {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: AmapMetrics) -> Result<()> {
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

fn request_worker(metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        let mut rng = rand::thread_rng();

        loop {
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..4);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
