use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use anyhow::{anyhow, Result};

const THREAD_NUMBER: usize = 4;
fn main() -> Result<()> {
    let (tx, rx) = channel();
    for i in 0..THREAD_NUMBER {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
    });
    consumer
        .join()
        .map_err(|e| anyhow!("Error to join thread: {:?}", e))?;
    Ok(())
}

fn producer(idx: usize, tx: Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;

        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time));
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Msg { idx, value }
    }
}
