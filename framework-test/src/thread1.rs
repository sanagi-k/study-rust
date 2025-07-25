use crate::thread_ctrl::ThreadCtrl;
use std::thread;
use std::time::Duration;

pub struct Thread1;

impl ThreadCtrl for Thread1 {
    fn init(&self) {
        println!("スレッド1: 初期化中...");
        thread::sleep(Duration::from_millis(500));
        println!("スレッド1: 初期化完了");
    }

    fn run(&self) {
        for i in 1..=5 {
            println!("スレッド1: {}", i);
            thread::sleep(Duration::from_millis(200));
        }
    }
}
