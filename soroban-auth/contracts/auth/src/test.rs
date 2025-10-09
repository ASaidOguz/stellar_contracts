#![cfg(test)]

use crate::{AuthContract, AuthContractClient};
use soroban_sdk::{testutils::{Address as _, MockAuth, MockAuthInvoke}, Address, Env, IntoVal};


#[test]
fn test_admin_increment() {
    let env = Env::default();
    let admin = Address::generate(&env);                   
    //Single-element tuple! as constructor arg
    let contract_id = env.register(AuthContract, (&admin,));
    let client = AuthContractClient::new(&env, &contract_id);
    
    // ✅ Test increment WITH authentication (admin required)
    let count =client
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "increment",
                args: (5u32,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .increment(&5);
        // assert returned value;
        assert_eq!(&count,&5);        
}

#[test]
#[should_panic]
fn test_nonadmin_increment(){

    let env = Env::default();
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);    

    //Single-element tuple! as constructor arg
    let contract_id = env.register(AuthContract, (&admin,));
    let client = AuthContractClient::new(&env, &contract_id);
    
    // ✅ Test increment WITH authentication (admin required)
    let _ =client
        .mock_auths(&[MockAuth {
            address: &non_admin,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "increment",
                args: (5u32,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .increment(&5);
        // assert returned value;
        //assert_eq!(&count,&5); -> no need to assert its gonna fail.      
}

// Soroban SDK create two functions 
//1- increment which returns solid u32 type
//2- try_increment which returns result type -> Result<u32,Error> 