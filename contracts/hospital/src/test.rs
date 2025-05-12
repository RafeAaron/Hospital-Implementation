#![cfg(test)]
use crate::{HospitalContract, HospitalContractClient};
use soroban_sdk::{Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(HospitalContract, ());
    let client = HospitalContractClient::new(&env, &contract_id);

    client.initialize( &contract_id);
}

#[test]
fn check_if_admin()
{

    let env = Env::default();
    let contract_id = env.register(HospitalContract, ());
    let client = HospitalContractClient::new(&env, &contract_id);

    assert_eq!(contract_id, client.initialize(&contract_id));
    
}