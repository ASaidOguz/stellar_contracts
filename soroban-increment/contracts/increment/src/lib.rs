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
        
        // increment the count;
        count += 1;
        // fire event for updated count
        log!(&env, "increased count: {}", count);
        // store the new count back in storage(overwriting the existing value);
        env
        .storage()
        .instance()
        .set(&COUNTER, &count);
        // some kind of storage time to live method
        env
        .storage()
        .instance()
        .extend_ttl(50, 100);
        // return the new count value;
        count
    }
    /// Decrement decreases an internal counter, and returns the value.
    pub fn decrement(env:Env) ->u32{
        // retrieve the current count, defaulting to 0 if not set;
        let mut count: u32 = env
        .storage()
        .instance()
        .get(&COUNTER)
        .unwrap_or(3);

        if count==0 {
            env
            .storage()
            .instance()
            .extend_ttl(50, 100);
            log!(&env, "count-already-zero: {}", count);
            return count;
        }
        // update count;  
        count -=1;
        // firing event ;
        log!(&env, "decreased count: {}", count);
        // store the new count back in storage(overwriting the existing value);
        env
        .storage()
        .instance()
        .set(&COUNTER, &count);

        // some kind of storage time to live method
        env
        .storage()
        .instance()
        .extend_ttl(50, 100);
        // return the new count value;
        count

    }
    /// Reset resets an internal counter, and returns the value.
    pub fn reset_count(env:Env) ->u32{
        // retrieve the current count, defaulting to 0 if not set;
        let mut count: u32 = env
        .storage()
        .instance()
        .get(&COUNTER)
        .unwrap_or(0);
       
       if count==0 {
            env
            .storage()
            .instance()
            .extend_ttl(50, 100);
            log!(&env, "count-already-zero: {}", count);
            return count;
        }
             // update count;  
        count =0;
        // firing event ;
        log!(&env, "reset count: {}", count);
        // store the new count back in storage(overwriting the existing value);
        env
        .storage()
        .instance()
        .set(&COUNTER, &count);

     // some kind of storage time to live method
        env
        .storage()
        .instance()
        .extend_ttl(50, 100);
        // return the new count value;
        count

    }

    pub fn get_current_count(env:Env) ->u32{
        // retrieve the current count, defaulting to 0 if not set -> keep read-only;
        let count: u32 = env
        .storage()
        .instance()
        .get(&COUNTER)
        .unwrap_or(0);

        count
    }
}

mod test;