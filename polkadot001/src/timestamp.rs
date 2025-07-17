
#[derive(Default, Debug)]
pub struct Timestamp {
    pub timestamp: u64, // Unix timestamp in milliseconds
}

impl Timestamp {
    pub fn set(&mut self, moment: u64) {
        self.timestamp = moment;
        println!("Timestamp set to: {}", self.timestamp);
    }

    pub fn get(&self) -> u64 {
        self.timestamp
    }
}