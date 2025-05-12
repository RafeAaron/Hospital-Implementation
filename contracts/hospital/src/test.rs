#![cfg(test)]

use crate::{HospitalContract, HospitalContractClient};
use soroban_sdk::{Env, String, Vec};

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

    env.mock_all_auths();
    let contract_id = env.register(HospitalContract, ());
    let client = HospitalContractClient::new(&env, &contract_id);
    let user = client.initialize(&contract_id);
    client.check_admin();
    assert_eq!(contract_id, user);
    
}

#[test]
fn add_patient()
{
    let env = Env::default();
    let contract_id = env.register(HospitalContract, ());

    env.mock_all_auths();

    let client = HospitalContractClient::new(&env, &contract_id);

    client.initialize(&contract_id);

    let mut allergies:Vec<_> = Vec::new(&env);
    allergies.push_back(String::from_str(&env, "Cancer"));
    allergies.push_back(String::from_str(&env, "Waking up"));

    let date_of_birth = 20304005;

    let patient_id:u64 = client.register_patient(&String::from_str(&env, "Rafe Aaron"), &date_of_birth, &String::from_str(&env, "O+"), &allergies , &String::from_str(&env, "65ydg97r5"));
    assert_eq!(patient_id, 1);
}

#[test]
fn update_patient_details()
{
    let env = Env::default();

    env.mock_all_auths();
    let contract_id = env.register(HospitalContract, ());
    let client = HospitalContractClient::new(&env, &contract_id);

    client.initialize(&contract_id);

    let name = String::from_str(&env, "Rafe Aaron");
    let mut date_of_birth = 20102003;
    let blood_type = String::from_str(&env, "A+");
    let mut allergies = Vec::new(&env);
    let insurance_id = String::from_str(&env, "56789hhgfvb");

    allergies.push_back(String::from_str(&env, "Cancer"));
    allergies.push_back(String::from_str(&env, "Capricorns"));

    let patient_id = client.register_patient(&name, &date_of_birth, &blood_type, &allergies, &insurance_id);

    date_of_birth = 10102005;

    let patient = client.update_patient(&patient_id, &name, &date_of_birth, &blood_type, &allergies, &insurance_id);

    assert_eq!(patient.date_of_birth, date_of_birth);
    
}

#[test]
fn get_patient_details()
{
    let env = Env::default();

    env.mock_all_auths();

    let contract_id = env.register(HospitalContract, ());

    let client = HospitalContractClient::new(&env, &contract_id);
    client.initialize(&contract_id);

    let name = String::from_str(&env, "Rafe Aaron");
    let date_of_birth = 20102003;
    let blood_type = String::from_str(&env, "B+");
    let mut allergies = Vec::new(&env);
    allergies.push_back(String::from_str(&env, "Cancer"));

    let insurance_id = String::from_str(&env, "6yy987yv");

    let patient_id = client.register_patient(&name, &date_of_birth, &blood_type, &allergies, &insurance_id);

    let patient = client.get_patient(&patient_id);

    assert_eq!(patient.date_of_birth, date_of_birth);
}

#[test]
fn set_paient_active()
{
    let env = Env::default();

    env.mock_all_auths();

    let contract_id = env.register(HospitalContract, ());

    let client = HospitalContractClient::new(&env, &contract_id);
    client.initialize(&contract_id);

    let name = String::from_str(&env, "Rafe Aaron");
    let date_of_birth = 20102003;
    let blood_type = String::from_str(&env, "B+");
    let mut allergies = Vec::new(&env);
    allergies.push_back(String::from_str(&env, "Cancer"));

    let insurance_id = String::from_str(&env, "6yy987yv");

    let patient_id = client.register_patient(&name, &date_of_birth, &blood_type, &allergies, &insurance_id);

    let patient = client.set_patient_active(&patient_id, &true);

    assert_eq!(patient.active, true);
}

#[test]
fn list_all_patients()
{
    let env = Env::default();

    env.mock_all_auths();

    let contract_id = env.register(HospitalContract, ());

    let client = HospitalContractClient::new(&env, &contract_id);
    client.initialize(&contract_id);

    let name = String::from_str(&env, "Rafe Aaron");
    let date_of_birth = 20102003;
    let blood_type = String::from_str(&env, "B+");
    let mut allergies = Vec::new(&env);
    allergies.push_back(String::from_str(&env, "Cancer"));

    let insurance_id = String::from_str(&env, "6yy987yv");

    client.register_patient(&name, &date_of_birth, &blood_type, &allergies, &insurance_id);
    client.register_patient(&name, &date_of_birth, &blood_type, &allergies, &insurance_id);
    client.register_patient(&name, &date_of_birth, &blood_type, &allergies, &insurance_id);
    client.register_patient(&name, &date_of_birth, &blood_type, &allergies, &insurance_id);

    let patients = client.list_patients();

    assert_eq!(patients.len(), 4);

    
}