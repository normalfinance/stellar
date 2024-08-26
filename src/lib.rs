#![no_std]

use soroban_sdk::{contractimpl, Env, Symbol};

pub struct MyContract;

#[contractimpl]
impl MyContract {
    // Store data
    pub fn write_data(env: Env, key: Symbol, value: Symbol) {
        env.storage().set(&key, &value);
    }

    // Read data
    pub fn read_data(env: Env, key: Symbol) -> Option<Symbol> {
        env.storage().get(&key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::EnvExt, Env, Symbol};

    #[test]
    fn test_write_and_read() {
        let env = Env::default();
        let contract_id = env.register_contract(None, MyContract);

        let key = Symbol::new(&env, "exampleKey");
        let value = Symbol::new(&env, "exampleValue");

        // Write the data
        MyContract::write_data(env.clone(), key.clone(), value.clone());

        // Read the data
        let result = MyContract::read_data(env.clone(), key.clone());

        assert_eq!(result, Some(value));
    }
}
