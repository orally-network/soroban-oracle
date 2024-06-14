#![cfg(test)]

use super::*;
use ethabi::ParamType;
use ethers_core::utils::hex;
use soroban_sdk::{
    testutils::{Address as _, Logs},
    Env,
};

extern crate std;

const TEST_REPORTER: &'static str = "0x61E9658dFE7c620E96ae41f97b89B079Ef7ECd1A";
const TEST_GET_XRC_DATA_BYTES: &'static str = "0x00000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000160000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000033362743b800000000000000000000000000000000000000000000000000000000000000009000000000000000000000000000000000000000000000000000000006669527400000000000000000000000000000000000000000000000000000000000000074554482f5553440000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000006669528c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000074554482f555344000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000041b5349cd9807da634851ff0cf7883098087d6b7a48b9f92295df1d84ef40b5031446eee4c459f0d553283781f70b33d2aafa3ed8bb3fb4e8393877e69a47754351b00000000000000000000000000000000000000000000000000000000000000";
const TEST_GET_XRC_DATA_BYTES_NOT_A_REPORTER: &'static str = "0x0000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000016000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000003b9aca00000000000000000000000000000000000000000000000000000000000000000900000000000000000000000000000000000000000000000000000000666af05c0000000000000000000000000000000000000000000000000000000000000008555344542f55534400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000666af07900000000000000000000000000000000000000000000000000003a9060faa5df00000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000008555344542f55534400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000034554480000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004155fd3c6a8626ffc48cf8765c61b74b64254a9ab5284498a568b2660aeb2a3f271b587058952b85b50154fdf7467246177e9cee8dcbda588c02f0c956de96e0771c00000000000000000000000000000000000000000000000000000000000000";

fn init_contract<'a>(env: &'a Env, owner: &'a Address) -> OrallyVerifyerClient<'a> {
    let contract_id = env.register_contract(None, OrallyVerifyer);
    let client = OrallyVerifyerClient::new(&env, &contract_id);
    client.init(&owner);

    client
}

#[test]
fn test_verify_generic() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let test_reporter = String::from_str(&env, TEST_REPORTER);

    let client = init_contract(&env, &owner);
    client.add_reporter(&test_reporter);

    let hex = hex::decode(&TEST_GET_XRC_DATA_BYTES[2..]).unwrap();

    let bytes = Bytes::from_slice(&env, &hex);

    let mut vec = ethabi::decode(
        &[ParamType::Bytes, ParamType::Bytes, ParamType::Bytes],
        &bytes.to_alloc_vec(),
    )
    .unwrap();

    let _signature_bytes = vec.pop().unwrap().into_bytes().unwrap();
    let _meta_bytes = vec.pop().unwrap().into_bytes().unwrap();
    let generic_bytes = vec.pop().unwrap().into_bytes().unwrap();

    std::println!("{}", hex::encode(&generic_bytes));
    std::println!("{}", hex::encode(&_meta_bytes));
    std::println!("{}", hex::encode(&_signature_bytes));

    let verify_generic_result = client.verify_generic(&bytes);
    assert_eq!(
        verify_generic_result,
        Bytes::from_slice(&env, &generic_bytes)
    );

    let logs = env.logs().all();
    std::println!("{}", logs.join("\n"));
}

#[test]
#[should_panic(expected = "Owner already exists")]
fn test_init_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let owner = Address::generate(&env);
    let client = init_contract(&env, &owner);
    client.init(&owner);
}

#[test]
fn test_reporters() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let test_reporter = String::from_str(&env, TEST_REPORTER);

    let client = init_contract(&env, &owner);
    client.add_reporter(&test_reporter);

    let is_reporter_result = client.is_reporter(&test_reporter);
    assert!(is_reporter_result);

    client.remove_reporter(&test_reporter);

    let is_reporter_result = client.is_reporter(&test_reporter);
    assert!(!is_reporter_result);
}

#[test]
fn test_reporters_lowercase() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let test_reporter = String::from_str(&env, TEST_REPORTER);
    let test_reporter_lowercase = to_lowercase(String::from_str(&env, TEST_REPORTER));

    let client = init_contract(&env, &owner);
    client.add_reporter(&test_reporter);

    let is_reporter_result = client.is_reporter(&test_reporter);
    assert!(is_reporter_result);

    let is_reporter_result = client.is_reporter(&test_reporter_lowercase);
    assert!(is_reporter_result);
}

#[test]
#[should_panic]
fn test_owner_not_authorized() {
    let env = Env::default();
    let owner = Address::generate(&env);

    let test_reporter = String::from_str(&env, TEST_REPORTER);

    let client = init_contract(&env, &owner);

    // Next line will panic because the owner is not authorized
    client.add_reporter(&test_reporter);
}

#[test]
#[should_panic(expected = "Not a reporter")]
fn test_not_a_reporter() {
    let env = Env::default();
    env.mock_all_auths();
    let owner = Address::generate(&env);

    let client = init_contract(&env, &owner);

    let hex = hex::decode(&TEST_GET_XRC_DATA_BYTES_NOT_A_REPORTER[2..]).unwrap();

    let bytes = Bytes::from_slice(&env, &hex);

    client.verify_generic(&bytes);
}
