#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(CustomErrorContract, ());
    let client = CustomErrorContractClient::new(&env, &contract_id);

    assert_eq!(client.try_increment(), Ok(Ok(1)));
    assert_eq!(client.try_increment(), Ok(Ok(2)));
    assert_eq!(client.try_increment(), Ok(Ok(3)));
    assert_eq!(client.try_increment(), Ok(Ok(4)));
    assert_eq!(client.try_increment(), Ok(Ok(5)));
    assert_eq!(client.try_increment(), Err(Ok(Error::LimitReached)));

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_panic() {
    let env = Env::default();
    let contract_id = env.register(CustomErrorContract, ());
    let client = CustomErrorContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    assert_eq!(client.increment(), 4);
    assert_eq!(client.increment(), 5);
    client.increment();
}

/* ⚙️ Why the test sees “HostError: Error(Contract, #1)”

In Soroban, the Error enum you define (via #[contracterror]) doesn’t 
literally serialize as ContractError(1) in the panic message.
Instead, when the host interprets it, 
it classifies it under the “contract error” category (Error(Contract, #1)), where:

Contract → indicates it’s a contract-defined error type

#1 → your LimitReached value

The host doesn’t know the Rust enum name Error or LimitReached — 
it only carries over the code number 1 and the type category.

So, what you expect (Error(ContractError(1))) and 
what the Soroban host emits (Error(Contract, #1)) are functionally identical, 
just different representations. */