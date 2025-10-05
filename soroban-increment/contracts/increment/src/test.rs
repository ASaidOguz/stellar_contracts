#![cfg(test)]

use crate::{IncrementContract, IncrementContractClient};
use soroban_sdk::{symbol_short, testutils::Events, vec, Env, IntoVal};

#[test]
fn test_increment(){
    // create the contract environment
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    
    // test the increment function.
    assert_eq!(client.increment(), 1);
    assert_eq!(
        env.events().all(),
        vec![ &env,
            (
            contract_id.clone(),
            (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
            1u32.into_val(&env)
        )]
    );
    assert_eq!(client.increment(), 2);
       assert_eq!(
        env.events().all(),
        vec![ &env,
            (
            contract_id.clone(),
            (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
            2u32.into_val(&env)
        )]
    );
    assert_eq!(client.increment(), 3);
       assert_eq!(
        env.events().all(),
        vec![ &env,
            (
            contract_id.clone(),
            (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
            3u32.into_val(&env)
        )]
    );
    
}

#[test]
fn test_decrement(){
    // create the contract environment
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    
    // test the increment function
    assert_eq!(client.decrement(), 2);
     // test the events being published.
    assert_eq!(env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("decrement")).into_val(&env),
                2u32.into_val(&env)
            )    
        ]
    );

    assert_eq!(client.decrement(), 1);
     // test the events being published.
    assert_eq!(env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("decrement")).into_val(&env),
                1u32.into_val(&env)
            )    
        ]
    );

    assert_eq!(client.decrement(), 0);
     // test the events being published.
    assert_eq!(env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("decrement")).into_val(&env),
                0u32.into_val(&env)
            )    
        ]
    );
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
    // test the events being published.
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("reset")).into_val(&env),
                0u32.into_val(&env)
            ),
        ]
    );
    // assert again to see the special log
    assert_eq!(client.reset_count(),0);

    // only qualified as reset will publish event 
    //-> if the count already 0 it will just return the value
    
 
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


