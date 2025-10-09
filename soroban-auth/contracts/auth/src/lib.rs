#![no_std]
use soroban_sdk::{contract, contracterror, 
                  contractimpl, contracttype, 
                  symbol_short, Address, 
                  Env, Symbol};

// error 
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
     CallerNotAdmin = 1,
     ContractUninitialized =2,
     ContractAlreadyInitialized=3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitState {
    pub admin: Address,
    pub init_block:u32,//// ledger sequence of init
}

const INITSTATE: Symbol = symbol_short!("INITSTATE");
const COUNTER:Symbol = symbol_short!("COUNTER");

#[contract]
pub struct AuthContract;


#[contractimpl]
impl AuthContract {
   pub fn __constructor(env: Env,admin_address:Address)  {
  
    let state = InitState {
        admin:admin_address,
        init_block: env.ledger().sequence(),
        };

    env.storage().instance().set(&INITSTATE, &state);
    env.storage().instance().set(&COUNTER, &0u32);
  
    }

    // only admin can use increment;
    pub fn increment(env: Env, value: u32) -> Result<u32, Error> {
      let init_state = Self::get_initstate(env.clone()).ok_or(Error::ContractUninitialized)?;
      init_state.admin.require_auth();
      // u can safely use unwrap cause it will initalized with value;
      let mut counter :u32 = env.storage()
                                .instance()
                                .get(&COUNTER).unwrap();
      counter+=value;
      
      env.storage()
         .instance()
         .set(&COUNTER,&counter);
      Ok(counter)
    }

    /// Return the init-state.
    pub fn get_initstate(env: Env) -> Option<InitState> {
        env.storage().instance().get(&INITSTATE)
    }
}

mod test;
