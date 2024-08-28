use soroban_sdk::{contracttype, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Fund {
    pub name: Symbol,
}

#[contracttype]
#[derive(Clone)]
pub struct Price {
    pub numerator: u64,
    pub denominator: u64,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidParams = 1,
    InsufficientFunds = 2,
    InvalidAddress = 3,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Fund(Address),
    Price,
    Fee,
    Token,
    Admin,
}
