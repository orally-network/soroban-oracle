use alloy_core::hex;
use soroban_sdk::{Bytes, BytesN, Env, String};

const UTF8_0X_PREFIX: [u8; 2] = [0x30, 0x78];

// Recover the public key from a message and a signature
pub fn recover(env: &Env, message: &[u8], signature: &[u8]) -> String {
    let message_digest = env.crypto().keccak256(&Bytes::from_slice(&env, message));

    let recovery_id = (signature[64] % 27).into();

    let signature = BytesN::from_array(&env, signature[0..64].try_into().unwrap());

    let encoded_point_uncompressed: Bytes = env
        .crypto()
        .secp256k1_recover(&message_digest, &signature, recovery_id)
        .into();

    let pubkey_hash: Bytes = env
        .crypto()
        .keccak256(&encoded_point_uncompressed.slice(1..))
        .into();

    // Address is 20 bytes, but utf8 is 2 bytes per character,
    // thus we need not 20 but 40 bytes
    // But in order to add the prefix "0x" we need 42 bytes
    let mut address_str_bytes = [0u8; 42];
    address_str_bytes[..2].copy_from_slice(&UTF8_0X_PREFIX);

    let mut pubkey_buf = [0u8; 20];
    pubkey_hash.slice(12..).copy_into_slice(&mut pubkey_buf);

    hex::encode_to_slice(&pubkey_buf, &mut address_str_bytes[2..]).unwrap();

    String::from_bytes(env, &address_str_bytes)
}
