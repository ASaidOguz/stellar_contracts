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

#[test]
fn test_decrement(){
    // create the contract environment
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    
    // test the increment function
    assert_eq!(client.decrement(), 2);
    assert_eq!(client.decrement(), 1);
    assert_eq!(client.decrement(), 0);
    assert_eq!(client.decrement(), 0);

}

#[test]
fn test_reset(){
    // create the contract environment
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    // increase internal count so we can set 
    client.increment();
    // test the increment function
    assert_eq!(client.reset_count(), 0);
    // assert again to see the special event
    assert_eq!(client.reset_count(),0);
}

#[test]
fn test_get_current_count(){
    // create the contract environment
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    // increase internal count so we can set 
    client.increment();
    assert_eq!(client.get_current_count(), 1);
    // cli simulation can understand if its read-only or mutating function and act according to.
}


