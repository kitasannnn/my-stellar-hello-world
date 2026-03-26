#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, String, Symbol};

const GREETING_KEY: Symbol = symbol_short!("GREET");

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn set_greeting(env: Env, new_greeting: String) {
        env.storage().instance().set(&GREETING_KEY, &new_greeting);
    }

    pub fn get_greeting(env: Env) -> String {
        env.storage()
            .instance()
            .get(&GREETING_KEY)
            .unwrap_or(String::from_str(&env, "No greeting yet!"))
    }
}