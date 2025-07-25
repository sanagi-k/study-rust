use crate::thread_ctrl::ThreadCtrl;
use std::thread;
use std::time::Duration;

pub struct Thread3;

impl ThreadCtrl for Thread3 {
    fn init(&self) {
        println!("スレッド3: 初期化中...");
        thread::sleep(Duration::from_millis(400));
        println!("スレッド3: 初期化完了");
    }

    fn run(&self) {
        for i in 1..=5 {
            println!("スレッド3: {}", i);
            thread::sleep(Duration::from_millis(400));
        }
    }
}
