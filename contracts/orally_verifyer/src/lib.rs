#![no_std]

use alloy_sol_types::SolValue;
use ethabi::{ParamType, Token};
use interface::OrallyVerifyerInterface;
use sol_types::Meta;
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env, String};
use utils::{check_owner, check_reporter, has_owner, set_owner, to_lowercase};

mod ecdsa;
pub mod interface;
mod sol_types;
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
impl OrallyVerifyerInterface for OrallyVerifyer {
    fn init(env: Env, owner: Address) {
        if has_owner(&env) {
            panic!("Owner already exists")
        }

        set_owner(&env, &owner);
    }

    fn verify_generic(env: &Env, data: Bytes) -> Bytes {
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

    fn is_reporter(env: Env, address: String) -> bool {
        utils::get_instance(&env, &DataKey::Reporters(to_lowercase(address))).unwrap_or(false)
    }

    fn add_reporter(env: Env, address: String) {
        check_owner(&env);
        utils::set_instance(&env, &DataKey::Reporters(to_lowercase(address)), &true);
    }

    fn remove_reporter(env: Env, address: String) {
        check_owner(&env);
        utils::remove_instance(&env, &DataKey::Reporters(to_lowercase(address)));
    }

    fn upgrade(env: Env, wasm_hash: BytesN<32>) {
        check_owner(&env);

        env.deployer().update_current_contract_wasm(wasm_hash);
    }
}

impl OrallyVerifyer {
    fn verify(env: &Env, generic_bytes: Token, meta_bytes: Token, signature_bytes: Token) -> Bytes {
        let generic_bytes = generic_bytes.into_bytes().unwrap();
        let meta_bytes = meta_bytes.into_bytes().unwrap();
        let signature_bytes = signature_bytes.into_bytes().unwrap();

        let message = (generic_bytes.clone(), meta_bytes.clone());

        let meta = Meta::abi_decode(&meta_bytes, false).unwrap();

        if meta.fee > alloy_sol_types::private::u256(0) {
            panic!("Fee is not zero")
        }

        let encode_packed = message.abi_encode_packed();

        let reporter_address = ecdsa::recover(env, &encode_packed, &signature_bytes);

        check_reporter(&env, reporter_address);

        Bytes::from_slice(env, &generic_bytes)
    }
}

mod test;
