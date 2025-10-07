#![no_std]


use soroban_sdk::{contract, contracterror, contractimpl, log, symbol_short, Env, Symbol};

#[contracterror]
#[derive(Copy,Clone,Debug,Eq,PartialEq,PartialOrd,Ord)]
#[repr(u32)]
pub enum Error{
    LimitReached=1,
}

const COUNTER: Symbol = symbol_short!("COUNTER");
const MAX: u32 = 5;

#[contract]
pub struct CustomErrorContract;

#[contractimpl]
impl CustomErrorContract{
    pub fn increment(env:Env) ->Result<u32,Error>{
        let mut count:u32 = env
        .storage()
        .instance()
        .get(&COUNTER).unwrap_or(0);
    
        log!(&env, "count: {}", count);

        count+=1;

        if count<=MAX {
            env.storage().instance().set(&COUNTER,&count);

            Ok(count)
        } else{
         Err(Error::LimitReached)
        }
    }
}









mod test;
