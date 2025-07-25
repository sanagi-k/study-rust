pub trait ThreadCtrl: Send + Sync {
    fn init(&self);
    fn run(&self);
}
