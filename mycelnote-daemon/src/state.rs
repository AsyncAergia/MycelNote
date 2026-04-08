mod dispatch;
pub struct State {}

impl State {
    pub fn new() -> Self {
        State {}
    }

    pub fn log_event<T: std::fmt::Debug>(&self, name: &str, event: T) {
        println!("{}: {:?}", name, event);
    }
}
