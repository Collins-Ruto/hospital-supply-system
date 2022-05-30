use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

// 1. Main Struct
pub Struct Hospitals {
    hospital_name: String,
    hospital_level: String,
    hospital_county: String,
}

impl Hospitals {
    pub fn new(&mut self, name: String) {
        println!("{}", name);
    }
}
// 2. Default Implementation

// 3. Core Logic

// 4. Testsn
