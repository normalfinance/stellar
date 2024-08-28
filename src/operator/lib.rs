#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, U256};

#[contractype]
#[derive(Clone)]
pub enum Action {
    AddFund,
    AddToken,
    AddTrustedWallets,
    AddColdWallet,
    EtherTransfer,
    TokenTransfer,
    PriceUpdate,
    Pause,
}

#[contracttype]
pub enum isHotAccount {
    Bool(Address),
}

// Saves all trust party accounts
#[contracttype]
pub enum isTrustPartyAccount {
    Bool(Address),
}

// Saves all cold wallets used by the fund and the cold key used to protect the wallet.
// Format: (Wallet storing funds) => (cold account required for access)
#[contracttype]
pub enum coldStorage {
    Address(Address),
}

// Used for storing funds which can be quickly accessed
#[contracttype]
pub enum isHotWallet {
    Bool(Address),
}

#[contract]
pub struct Operator;

#[contractimpl]
impl Operator {
    pub fn FundOperator(
        env: Env,
        _hotThreshold: U256,
        _trustPartyThreshold: U256,
        _hotAccounts: Vec<Address>,
        _trustPartyAccounts: Vec<Address>,
    ) {
        if _hotAccounts.len() 
    }
}
