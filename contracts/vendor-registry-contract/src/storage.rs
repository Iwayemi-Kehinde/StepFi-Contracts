use crate::{
    errors::Error,
    types::{DataKey, VendorInfo},
};
use soroban_sdk::{Address, Env};

pub const PERSISTENT_TTL_THRESHOLD: u32 = 1_036_800;
pub const PERSISTENT_TTL_EXTEND_TO: u32 = 2_073_600;
// Version stored in instance storage
use soroban_sdk::symbol_short;
pub const VERSION_KEY: soroban_sdk::Symbol = symbol_short!("VERSION");

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn get_admin(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn has_vendor(env: &Env, vendor: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Vendor(vendor.clone()))
}

pub fn get_vendor(env: &Env, vendor: &Address) -> Result<VendorInfo, Error> {
    env.storage()
        .persistent()
        .get(&DataKey::Vendor(vendor.clone()))
        .ok_or(Error::VendorNotFound)
}

pub fn set_vendor(env: &Env, vendor: &Address, info: &VendorInfo) {
    let key = DataKey::Vendor(vendor.clone());
    env.storage().persistent().set(&key, info);
    extend_persistent_ttl(env, &key);
}

pub fn get_vendor_count(env: &Env) -> Result<u64, Error> {
    Ok(env
        .storage()
        .persistent()
        .get(&DataKey::VendorCount)
        .unwrap_or(0))
}

pub fn increment_vendor_count(env: &Env) -> Result<(), Error> {
    let count = get_vendor_count(env)?;
    let next = count.checked_add(1).ok_or(Error::Overflow)?;
    let key = DataKey::VendorCount;
    env.storage().persistent().set(&key, &next);
    extend_persistent_ttl(env, &key);
    Ok(())
}

pub fn is_reentrancy_locked(env: &Env) -> Result<bool, Error> {
    Ok(env
        .storage()
        .instance()
        .get(&DataKey::Locked)
        .unwrap_or(false))
}

pub fn set_reentrancy_locked(env: &Env, locked: bool) {
    env.storage().instance().set(&DataKey::Locked, &locked);
}

fn extend_persistent_ttl(env: &Env, key: &DataKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, PERSISTENT_TTL_THRESHOLD, PERSISTENT_TTL_EXTEND_TO);
}

pub fn get_version(env: &Env) -> Result<u32, Error> {
    Ok(env.storage().instance().get(&VERSION_KEY).unwrap_or(1u32))
}

pub fn set_version(env: &Env, v: u32) {
    env.storage().instance().set(&VERSION_KEY, &v);
}

pub const SCHEMA_VERSION_KEY: soroban_sdk::Symbol = symbol_short!("SCH_VER");

/// Get the contract schema version (persistent storage). Defaults to 0 when not set.
pub fn get_schema_version(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&SCHEMA_VERSION_KEY)
        .unwrap_or(0u32)
}

/// Set the contract schema version in persistent storage.
pub fn set_schema_version(env: &Env, version: u32) {
    env.storage().persistent().set(&SCHEMA_VERSION_KEY, &version);
    env.storage().persistent().extend_ttl(
        &SCHEMA_VERSION_KEY,
        PERSISTENT_TTL_THRESHOLD,
        PERSISTENT_TTL_EXTEND_TO,
    );
}

