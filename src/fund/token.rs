use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn read_token(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::TokenA).unwrap();
}

pub fn write_token(e: &Env, id: &Address) {
    e.storage().instance().set(&DataKey::Token, id);
}
