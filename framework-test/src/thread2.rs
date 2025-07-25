use crate::thread_ctrl::ThreadCtrl;
use std::thread;
use std::time::Duration;

pub struct Thread2;

impl ThreadCtrl for Thread2 {
    fn init(&self) {
        println!("スレッド2: 初期化中...");
        thread::sleep(Duration::from_millis(300));
        println!("スレッド2: 初期化完了");
    }

    fn run(&self) {
        for i in 1..=5 {
            println!("スレッド2: {}", i);
            thread::sleep(Duration::from_millis(300));
        }
    }
}
