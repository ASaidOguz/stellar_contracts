#![no_std]
use soroban_sdk::{contract, contractimpl,contracterror,contracttype, vec, Env, String, Vec};

// error 
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    CallerNotAdmin = 1,
    ContractUninitialize =2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitState {
    pub admin: Address,
    pub is_init: bool,
    pub init_block:u32,//// ledger sequence of init
}

const INITSTATE: Symbol = symbol_short!("INITSTATE");
const COUNTER:Symbol = symbol_short!("COUNTER");

#[contract]
pub struct AuthContract;


#[contractimpl]
impl AuthContract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
