#![cfg(test)]

use crate::{IncrementContract, IncrementContractClient};
use soroban_sdk::Env;

#[test]
fn test_increment(){
    // create the contract environment
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    
    // test the increment function
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

}


