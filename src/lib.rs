use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use std::collections::HashMap;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Supplies {
    data: HashMap<String, Supply>,
    hospitals: HashMap<String, Hospitals>,
    suppliers: HashMap<String, Supplier>,
    items: HashMap<String, Item>,
    ids: i32,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Supply {
    hospital: Hospitals,
    supplier: Supplier,
    supplies: HashMap<String, Item>,
    sponsor: String,
    fund: f32,
    misc: HashMap<String, f32>,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Hospitals {
    hospital_level: String,
    hospital_county: String,
    supplies: HashMap<String, Item>,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Item {
    cost: f32,
    manufacturer: String,
    types: String,
    date_of_manufacture: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Supplier {
    types: String,
    supply_runs: u32,
    supply_worth: f32,
}

#[near_bindgen]
impl Supplies {
    #[private]
    pub fn add_hospital(&mut self, name: String, level: String, county: String) {
        let new_hospital = Hospitals {
            hospital_level: level,
            hospital_county: county,
            supplies: HashMap::new(),
        };
        self.hospitals.insert(name, new_hospital);
    }

    #[private]
    pub fn add_item(
        &mut self,
        name: String,
        cost: f32,
        manufacturer: String,
        types: String,
        date: String,
    ) {
        let new_item = Item {
            cost: cost,
            manufacturer: manufacturer,
            types: types,
            date_of_manufacture: date,
        };
        self.items.insert(name, new_item);
    }

    #[private]
    pub fn add_supplier(&mut self, name: String, types: String) {
        let new_supplier = Supplier {
            types: types,
            supply_runs: 0,
            supply_worth: 0.0,
        };
        self.suppliers.insert(name, new_supplier);
    }
}
// 2. Default Implementation

// 3. Core Logic

// 4. Testsn
