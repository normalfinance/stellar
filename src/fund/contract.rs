use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::price::{read_price, write_price};
#[cfg(test)]
use crate::storage_types::{DataKey, Error};
use crate::storage_types::{DEPOSIT_FEE, WITHDRAW_FEE};
use crate::token::{read_token, write_token};
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, token, Env, Symbol};

mod token {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32-unknown-unknown/release/soroban_cross_token_contract.wasm"
    );
}

#[contract]
pub struct Fund;

#[contractimpl]
impl Fund {
    pub fn initialize(
        e: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
        fee: u64,
    ) {
        // Save admin
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);

        // Deploy new Token instance
        let token = TokenClient::new(e, &e.register_contract(None, Token {}));
        token.initialize(admin, &decimal, &name, &symbol);

        // Save token
        write_token(&e, &token);

        write_fee(&e, &fee);
    }

    // @dev Purchase function which is used to buy shares/tokens in exchange for Ether
    // The amount of tokens received in exchange for Ether is calculated based on the current price and purchase fee
    // @param _to Address to which the purchased tokens will be credited to.
    pub fn invest(e: Env, _spender: Address, _to: Address, _value: u64) {
        _spender.require_auth();

        let current_price = read_price(&e);

        let converted_value = _value * current_price.numerator / current_price.denominator;
        let purchase_value = converted_value * PURCHASE_FEE / 100;

        let key = DataKey::Token(addr);
        token.mint(&e, &_to, purchase_value);
        // let share_contract_id = get_token_share(e);
        // token::Client::new(e, &share_contract_id).mint(&to, &amount);

        env.events()
            .publish((symbol_short!("Invest")), (_spender, _to, purchase_value));
    }

    // @dev Withdraw function which is used to sell your shares/tokens of the fund in exchange for Ether
    // The amount of Ether received in exchange for tokens is calculated based on the current price and withdraw fee
    // In the case of a successful withdrawal the tokens received are burned.
    // Can fail if the fund does no have enough Ether
    // Purposely no withdrawal pattern was implemented because the use case is simple enough here IMO
    // @param _to Address to which the Ether received in exchange for the tokens is sent to
    // @param _value Amount of tokens to withdrawn/sold
    pub fn divest(e: Env, _owner: Address, _to: Address, _value: u64) {
        _owner.require_auth();

        let current_price = read_price(&e);

        let converted_value = current_price.denominator * _value / current_price.numerator;
        let withdraw_value = converted_value * WITHDRAW_FEE / 100;

        let balance = get_balance(e.current_contract_address());

        if balance >= withdraw_value {
            let key = DataKey::Token(addr);
            token.burn(&e, _owner, _value);
            token.transfer(&e, _owner, &_to, &withdraw_value);
            env.events().publish(
                (symbol_short!("Divest")),
                (_owner, _to, _value, withdraw_value),
            );
        } else {
            Err(Error::InsufficientFunds)
        }
    }

    // @dev Simple function which updates the current price of the tokens/shares
    // @param _numerator Numerator of the currentPrice
    // @param _denominator Denominator of the currentPrice
    pub fn update_price(e: Env, _admin: Address, _numerator: u64, _denominator: u64) {
        _admin.require_auth();

        if _numerator = 0 || _denominator = 0 {
            Err(Error::InvalidParams)
        }

        write_price(
            &e,
            Price {
                _numerator,
                _denominator,
            },
        );
        env.events()
            .publish((symbol_short!("PriceUpdate")), (_numerator, _denominator));
    }

    pub fn update_fee(e: Env, _admin: Address, _fee: u64) {
        _admin.require_auth();

        write_fee(&e, &_fee);
        env.events().publish((symbol_short!("FeeUpdate")), (_fee));
    }
}
