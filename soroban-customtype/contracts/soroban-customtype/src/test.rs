use crate::{SorobanCustomType, SorobanCustomTypeClient,State};
use soroban_sdk::{Env};
#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(SorobanCustomType, ());
    let client = SorobanCustomTypeClient::new(&env, &contract_id);

    assert_eq!(client.increment(&1), 1);
    assert_eq!(client.increment(&10), 11);
    assert_eq!(
        client.get_state(),
        State {
            count: 11,
            last_incr: 10
        }
    );
}

/* ✅ TL;DR Summary
Concept	Explanation
Why &1?	
The Soroban client’s generated methods expect arguments implementing IntoVal, 
and &u32 implements that — not u32. */