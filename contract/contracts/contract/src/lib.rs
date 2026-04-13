#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

#[contract]
pub struct SimpleStorage;

#[contractimpl]
impl SimpleStorage {

    // Store a number
    pub fn set(env: Env, value: i32) {
        let key = symbol_short!("VALUE");
        env.storage().instance().set(&key, &value);
    }

    // Get the stored number
    pub fn get(env: Env) -> i32 {
        let key = symbol_short!("VALUE");
        env.storage().instance().get(&key).unwrap_or(0)
    }
}
 