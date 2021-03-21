use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

pub struct Robot(String);
lazy_static! {
    static ref NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}
impl Robot {
    pub fn new() -> Self {
        loop {
            let name = Robot::next_name();
            if !NAMES.lock().unwrap().insert(name.clone()) {
                return Robot(name);
            }
        }
    }

    fn next_name() -> String {
        format!(
            "{}{}{:03}",
            rand::thread_rng().gen_range('A'..='Z'),
            rand::thread_rng().gen_range('A'..='Z'),
            rand::thread_rng().gen_range(0..=999)
        )
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        loop {
            let name = Robot::next_name();
            {
                let mut names = NAMES.lock().unwrap();
                if !names.insert(name.clone()) {
                    names.remove(&self.0);
                    self.0 = name;
                    break;
                }
            }
        }
    }
}
