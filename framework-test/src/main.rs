mod thread1;
mod thread2;
mod thread3;
mod thread_ctrl;

use std::thread;
use thread_ctrl::ThreadCtrl;
use thread1::Thread1;
use thread2::Thread2;
use thread3::Thread3;

fn main() {
    let runners: Vec<Box<dyn ThreadCtrl>> =
        vec![Box::new(Thread1), Box::new(Thread2), Box::new(Thread3)];

    let handles: Vec<_> = runners
        .into_iter()
        .map(|runner| {
            thread::spawn(move || {
                runner.init();
                runner.run();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("すべてのスレッドが終了しました。");
}
