use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn read_fee(e: &Env) -> Fee {
    e.storage().instance().get(&DataKey::Fee).unwrap()
}

pub fn write_fee(e: &Env, fee: &Fee) {
    e.storage().instance().set(&DataKey::Fee, fee);
}
