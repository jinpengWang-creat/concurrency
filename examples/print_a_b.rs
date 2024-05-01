use std::{sync::mpsc::channel, thread};

const TIMES: usize = 20;
fn main() {
    let (tx, rx) = channel();
    let (tx1, rx1) = channel();

    let join1 = thread::spawn(move || {
        let mut cur_times = 0usize;
        loop {
            if let Ok(val) = rx.recv() {
                if val == 0 {
                    break;
                }
                cur_times += 1;
                println!("{cur_times}: A");
                let _ = tx1.send(1);

                if cur_times == TIMES {
                    let _ = tx1.send(0);
                    break;
                }
            }
        }
    });

    let tx_begin = tx.clone();
    let join2 = thread::spawn(move || {
        let mut cur_times = 0usize;
        loop {
            if let Ok(val) = rx1.recv() {
                if val == 0 {
                    break;
                }
                cur_times += 1;
                println!("{cur_times}: B");
                let _ = tx.send(1);

                if cur_times == TIMES {
                    let _ = tx.send(0);
                    break;
                }
            }
        }
    });

    tx_begin.send(1).unwrap();
    join1.join().unwrap();
    join2.join().unwrap();
}
