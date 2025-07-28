use std::error::Error;

pub trait ThreadCtrl: Send + Sync {
    fn init(&self) -> Result<(), Box<dyn Error>>;
    fn run(&self) -> Result<(), Box<dyn Error>>;
}
