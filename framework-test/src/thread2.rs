use crate::thread_ctrl::ThreadCtrl;
use rand::Rng;
use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct Thread2;

impl ThreadCtrl for Thread2 {
    fn init(&self) -> Result<(), Box<dyn Error>> {
        let wait_time_ms = 300;
        let i = rand::rng().random_range(0..=1);
        if i == 0 {
            thread::sleep(Duration::from_millis(wait_time_ms));
            println!("スレッド2: 初期化完了");
            Ok(())
        } else {
            thread::sleep(Duration::from_millis(wait_time_ms * 10));
            Err("Error) Setup Failed".into())
        }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        for i in 1..=5 {
            println!("スレッド2: {}", i);
            thread::sleep(Duration::from_millis(300));
        }
        Ok(())
    }
}
