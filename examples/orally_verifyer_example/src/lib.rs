#![no_std]

use soroban_sdk::{
    contract, contractclient, contractimpl, contracttype, Address, Bytes, BytesN, Env, String,
};
use utils::{get_instance, has_owner, set_instance, set_owner};

mod utils;

// Interface for the OrallyVerifyer contract.
// OrallyVerifyerClient is constructed using the trait.
#[contractclient(name = "OrallyVerifyerClient")]
pub trait OrallyVerifyerInterface {
    fn init(env: Env, owner: Address);

    fn verify_generic(env: &Env, data: Bytes) -> Bytes;

    fn is_reporter(env: Env, address: String) -> bool;

    fn add_reporter(env: Env, address: String);

    fn remove_reporter(env: Env, address: String);

    fn upgrade(env: Env, wasm_hash: BytesN<32>);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Owner,
    OrallyVerifyer,
}

#[contract]
pub struct OrallyVerifyerExample;

#[contractimpl]
impl OrallyVerifyerExample {
    pub fn init(env: Env, owner: Address, orally_verifyer_address: Address) {
        if has_owner(&env) {
            panic!("Owner already exists")
        }

        set_owner(&env, &owner);

        set_instance(&env, &DataKey::OrallyVerifyer, &orally_verifyer_address)
    }

    pub fn update_feed(env: Env, feed: Bytes) -> Bytes {
        let orally_verifyer_address = get_instance(&env, &DataKey::OrallyVerifyer).unwrap();

        // Create a client for the OrallyVerifyer contract, that was constructed using
        // the trait.
        let client = OrallyVerifyerClient::new(&env, &orally_verifyer_address);

        let general_bytes = client.verify_generic(&feed);

        general_bytes
    }
}

mod test;
