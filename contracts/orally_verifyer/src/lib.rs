#![no_std]

use alloy_sol_types::SolValue;
use ethabi::{ParamType, Token};
use soroban_sdk::{contract, contractimpl, contracttype, log, Address, Bytes, BytesN, Env, String};
use utils::{check_owner, check_reporter, has_owner, set_owner, to_lowercase};

mod ecdsa;
mod utils;

#[contract]
pub struct OrallyVerifyer;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Reporters(String),
    Owner,
}

#[contractimpl]
impl OrallyVerifyer {
    pub fn init(env: Env, owner: Address) {
        if has_owner(&env) {
            panic!("Owner already exists")
        }

        set_owner(&env, &owner);
    }

    fn verify(env: &Env, generic_bytes: Token, meta_bytes: Token, signature_bytes: Token) -> Bytes {
        let message = (
            generic_bytes.clone().into_bytes().unwrap(),
            meta_bytes.into_bytes().unwrap(),
        );

        let encode_packed = message.abi_encode_packed();

        let reporter_address =
            ecdsa::recover(env, &encode_packed, &signature_bytes.into_bytes().unwrap());
        log!(&env, "Reporter address: {}", reporter_address);
        check_reporter(&env, reporter_address);

        Bytes::from_slice(env, &generic_bytes.into_bytes().unwrap())
    }

    pub fn verify_generic(env: &Env, data: Bytes) -> Bytes {
        let bytes_vec = data.to_alloc_vec();
        let mut vec = ethabi::decode(
            &[ParamType::Bytes, ParamType::Bytes, ParamType::Bytes],
            &bytes_vec,
        )
        .unwrap();

        let signature_bytes = vec.pop().unwrap();
        let meta_bytes = vec.pop().unwrap();
        let generic_bytes = vec.pop().unwrap();

        Self::verify(env, generic_bytes, meta_bytes, signature_bytes)
    }

    pub fn is_reporter(env: Env, address: String) -> bool {
        utils::get_instance(&env, &DataKey::Reporters(to_lowercase(address))).unwrap_or(false)
    }

    pub fn add_reporter(env: Env, address: String) {
        check_owner(&env);
        utils::set_instance(&env, &DataKey::Reporters(to_lowercase(address)), &true);
    }

    pub fn remove_reporter(env: Env, address: String) {
        check_owner(&env);
        utils::remove_instance(&env, &DataKey::Reporters(to_lowercase(address)));
    }

    pub fn upgrade(env: Env, wasm_hash: BytesN<32>) {
        check_owner(&env);

        env.deployer().update_current_contract_wasm(wasm_hash);
    }
}

mod test;