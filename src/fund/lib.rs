#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, token, Env, Symbol};



// pub fn add_token(env: Env, Address: token) {
//     // require(token == address(0));
//     // token = _token;

//     // emit FundTokenAdded(address(token));
//     env.events()
//         .publish((symbol_short!("FundTokenAdded")), token);
// }

// pub fn withdraw_to(env: Env, _to: Address, _value: u128) {
//     if address(token) != address(0) {
//         panic;
//     }
//     if currentPrice.numerator == 0 || currentPrice.denominator == 0 {
//         panic;
//     }

//     let converted_value = current_price.denominator * _value / current_price.numerator;
//     let withdraw_value = converted_value * WITHDRAW_FEE / 100;

//     let result = contract_call(env, other_contract_id, method, args);

//     // uint256 convertedValue = currentPrice.denominator.mul(_value).div(currentPrice.numerator);
//     // uint256 withdrawValue = convertedValue.mul(WITHDRAW_FEE).div(100);
//     // if (address(this).balance >= withdrawValue) {
//     //     token.burn(requestor, _value);
//     //     _to.transfer(withdrawValue);
//     //     emit Withdrawal(requestor, _to, _value, withdrawValue);
//     env.events()
//         .publish((symbol_short!("Withdrawal")), (requestor, _to, _value, withdrawValue));
//     // } else {
//     //     emit FailedWithdrawal(requestor, _to, _value, withdrawValue);
//     env.events()
//     .publish((symbol_short!("FailedWithdrawal")), (requestor, _to, _value, withdrawValue));
//     // }
// }
