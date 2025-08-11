use crate::thread_ctrl::ThreadCtrl;
use rand::Rng;
use serde::Deserialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct Config {
    wait_time_ms: u64,
}
pub struct Thread1 {
    pub config: Config,
}

impl ThreadCtrl for Thread1 {
    fn init(&self) -> Result<(), Box<dyn Error>> {
        let i = rand::rng().random_range(0..=1);
        if i == 0 {
            thread::sleep(Duration::from_millis(self.config.wait_time_ms));
            println!("スレッド1: 初期化完了");
            Ok(())
        } else {
            thread::sleep(Duration::from_millis(self.config.wait_time_ms * 10));
            Err("Error) Setup Failed".into())
        }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        for i in 1..=5 {
            println!("スレッド1: {}", i);
            thread::sleep(Duration::from_millis(200));
        }
        Ok(())
    }
}
