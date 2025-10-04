#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // retrieve the current count, defaulting to 0 if not set 
        let mut count: u32 = env
        .storage()
        .instance()
        .get(&COUNTER)
        .unwrap_or(0);
        // firing an event maybe ? 
        log!(&env, "count: {}", count);
        // increment the count;
        count += 1;
        // store the new count back in storage(overwriting the existing value);
        env
        .storage()
        .instance()
        .set(&COUNTER, &count);
        // ???
        env
        .storage()
        .instance()
        .extend_ttl(50, 100);
        // return the new count value;
        count
    }
}

mod test;