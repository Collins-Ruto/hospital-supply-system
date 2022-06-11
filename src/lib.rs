use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, json_types::U128, log, near_bindgen, AccountId, Promise};
use std::collections::HashMap;
use std::convert::TryInto;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Supplies {
    data: HashMap<String, Supply>,
    hospitals: HashMap<String, Hospitals>,
    suppliers: HashMap<String, Supplier>,
    items: HashMap<String, Item>,
    ids: i32,
    funds: HashMap<AccountId, f32>,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Supply {
    hospital: Hospitals,
    supplier: Supplier,
    supplies: HashMap<String, f32>,
    sponsor: String,
    supply_cost: f32,
    fund: f32,
    misc: HashMap<String, f32>,
    misc_cost: f32,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Hospitals {
    hospital_name: String,
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
    name: String,
    types: String,
    supply_runs: u32,
    supply_worth: f32,
}

#[near_bindgen]
impl Supplies {
    #[payable]
    pub fn deposit(&mut self) {
        let token = to_near(env::attached_deposit());
        let funder = env::predecessor_account_id();
        self.funds.insert(funder, token);
    }

    #[private]
    pub fn add_hospital(&mut self, name: String, level: String, county: String) {
        let new_hospital = Hospitals {
            hospital_name: name,
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
    pub fn add_supplier(&mut self, types: String) {
        let name = env::predecessor_account_id().to_string();
        let new_supplier = Supplier {
            name: name.clone(),
            types: types,
            supply_runs: 0,
            supply_worth: 0.0,
        };
        self.suppliers.insert(name, new_supplier);
    }

    pub fn new_supply(&mut self, sponsor: AccountId, hospital: String, fund: f32) -> String {
        let id = (self.ids + 1).to_string();
        let supplier_acc_id = env::predecessor_account_id().to_string();
        let hospital_name = self.hospitals[&hospital].clone();
        let supplier = self.suppliers[&supplier_acc_id].clone();
        let new_supply = Supply {
            hospital: hospital_name,
            supplier: supplier,
            supplies: HashMap::new(),
            sponsor: sponsor.to_string(),
            fund: fund,
            supply_cost: 0.0,
            misc: HashMap::new(),
            misc_cost: 0.0,
        };
        self.data.insert(id.clone(), new_supply);
        log!(
            "Note your supply id is {}, You'll need it to feed your supply data",
            id
        );
        id.to_string()
    }

    pub fn add_supplies(&mut self, id: String, supplies: String, costs: String) -> String {
        let supplies_vec: Vec<&str> = supplies.split(", ").collect();
        let supplies_cost: Vec<&str> = costs.split(", ").collect();
        let mut costs_vec: Vec<f32> = vec![];
        for s in &supplies_cost {
            costs_vec.push(s.parse().unwrap())
        }
        let supply_cost: f32 = costs_vec.iter().sum();
        if self.data.contains_key(&id) {
            for supply in supplies_vec.clone() {
                if !self.items.contains_key(&supply.to_string()) {
                    return "unsucessful".to_string();
                };
                let index = supplies_vec.iter().position(|&r| r == supply).unwrap();
                let cost = costs_vec[index];
                if let Some(supply_struct) = self.data.get_mut(&id) {
                    supply_struct.supplies.insert(supply.to_string(), cost);
                    supply_struct.supply_cost = supply_cost;
                    supply_struct.supplier.supply_worth += cost;
                    supply_struct.supplier.supply_runs += 1;
                }
            }
            return "successful".to_string();
        } else {
            log!("inexistent id")
        }
    }

    pub fn add_miscs(&mut self, id: String, misc: String, costs: String) {
        let miscs_vec: Vec<&str> = misc.split(", ").collect();
        let miscs_cost: Vec<&str> = costs.split(", ").collect();
        let mut costs_vec: Vec<f32> = vec![];
        for s in &miscs_cost {
            costs_vec.push(s.parse().unwrap())
        }
        let supply_cost: f32 = costs_vec.iter().sum();
        for supply in miscs_vec.clone() {
            let index = miscs_vec.iter().position(|&r| r == supply).unwrap();
            let cost = costs_vec[index];
            if let Some(supply_struct) = self.data.get_mut(&id) {
                supply_struct.misc.insert(supply.to_string(), cost);
                supply_struct.misc_cost = supply_cost;
                supply_struct.supplier.supply_worth += cost;
            }
        }
        log!("The public will judge your miscellaneous costs")
    }

    
}

impl Hospitals {
    pub fn clone(&self) -> Self {
        Self {
            hospital_name: self.hospital_name.clone(),
            hospital_county: self.hospital_county.clone(),
            supplies: HashMap::new(),
            hospital_level: self.hospital_level.clone(),
        }
    }
}

impl Supplier {
    pub fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            types: self.types.clone(),
            supply_runs: self.supply_runs.clone(),
            supply_worth: self.supply_worth.clone(),
        }
    }
}

fn to_near(yocto: u128) -> f32 {
    (yocto as f32) / 1_000_000_000_000_000_000_000_000.0
}
// 2. Default Implementation

// 3. Core Logic

// 4. Testsn
