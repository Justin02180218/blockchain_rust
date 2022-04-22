use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Txoutput {
    value: i32,
    to_addr: String,
}

impl Txoutput {
    pub fn new(value: i32, to_addr: &str) -> Self {
        Self {
            value,
            to_addr: to_addr.into(),
        }
    }

    pub fn is_locked(&self, address: &str) -> bool {
        self.to_addr.eq(address)
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}