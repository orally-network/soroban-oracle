use soroban_sdk::{contractclient, Address, Bytes, BytesN, Env, String};

#[contractclient(name = "OrallyVerifyerClient")]
pub trait OrallyVerifyerInterface {
    fn init(env: Env, owner: Address);

    fn verify_generic(env: &Env, data: Bytes) -> Bytes;

    fn is_reporter(env: Env, address: String) -> bool;

    fn add_reporter(env: Env, address: String);

    fn remove_reporter(env: Env, address: String);

    fn upgrade(env: Env, wasm_hash: BytesN<32>);
}
