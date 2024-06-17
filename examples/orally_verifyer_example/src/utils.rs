use soroban_sdk::{Address, Env, IntoVal, TryFromVal, Val};

use crate::DataKey;

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

pub fn has_owner(env: &Env) -> bool {
    is_exist_instance(env, &DataKey::Owner)
}

pub fn set_owner(env: &Env, owner: &Address) {
    set_instance(env, &DataKey::Owner, owner)
}
