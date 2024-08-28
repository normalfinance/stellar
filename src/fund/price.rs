use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn read_price(e: &Env) -> Price {
    let key = DataKey::Price;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_price(e: &Env, price: &Price) {
    let key = DataKey::Price;
    e.storage().instance().set(&key, price);
}
