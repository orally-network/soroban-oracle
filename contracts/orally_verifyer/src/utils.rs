use soroban_sdk::{Address, Env, IntoVal, String, TryFromVal, Val};

use crate::DataKey;

extern crate alloc;

pub fn get_instance<K, V>(env: &Env, key: &K) -> Option<V>
where
    K: IntoVal<Env, Val>,
    V: TryFromVal<Env, Val>,
{
    env.storage().instance().get(key)
}

pub fn is_exist_instance<K>(env: &Env, key: &K) -> bool
where
    K: IntoVal<Env, Val>,
{
    env.storage().instance().has(key)
}

pub fn set_instance<K, V>(env: &Env, key: &K, value: &V)
where
    K: IntoVal<Env, Val>,
    V: IntoVal<Env, Val>,
{
    env.storage().instance().set(key, value)
}

pub fn remove_instance<K>(env: &Env, key: &K)
where
    K: IntoVal<Env, Val>,
{
    env.storage().instance().remove(key)
}

pub fn has_owner(env: &Env) -> bool {
    is_exist_instance(env, &DataKey::Owner)
}

pub fn get_owner(env: &Env) -> Option<Address> {
    get_instance(env, &DataKey::Owner)
}

pub fn set_owner(env: &Env, owner: &Address) {
    set_instance(env, &DataKey::Owner, owner)
}

pub fn check_owner(env: &Env) {
    if !has_owner(env) {
        panic!("Owner doesn't exist")
    }

    let owner = get_owner(env).unwrap();
    owner.require_auth();
}

pub fn check_reporter(env: &Env, address: String) {
    if !env.storage().instance().has(&DataKey::Reporters(address)) {
        panic!("Not a reporter")
    }
}

// Used for evm addresses for now
pub fn to_lowercase(address: String) -> String {
    let env = address.env();

    // This size of the buffer is calculated based on the idea that
    // this function is used only for evm addresses.
    // Note that you need to change buf size if you want to use this function for other purposes.
    //
    // Address is 20 bytes, but utf8 is 2 bytes per character,
    // thus we need not 20 but 40 bytes
    // But in order to add the prefix "0x" we need 42 bytes
    let mut string_buf = [0u8; 42];

    address.copy_into_slice(&mut string_buf);

    String::from_str(
        env,
        &alloc::string::String::from_utf8_lossy(&string_buf[..]).to_lowercase(),
    )
}
