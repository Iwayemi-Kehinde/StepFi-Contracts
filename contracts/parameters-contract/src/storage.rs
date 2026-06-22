use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::errors::ParametersError;
use crate::types::ProtocolParameters;

pub const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
pub const PARAMS_KEY: Symbol = symbol_short!("PARAMS");
pub const REENTRANCY_LOCK: Symbol = symbol_short!("LOCKED");
pub const VERSION_KEY: Symbol = symbol_short!("VERSION");

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&ADMIN_KEY)
}

pub fn get_admin(env: &Env) -> Result<Address, ParametersError> {
    env.storage()
        .instance()
        .get(&ADMIN_KEY)
        .ok_or(ParametersError::NotInitialized)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN_KEY, admin);
}

pub fn get_parameters(env: &Env) -> Result<ProtocolParameters, ParametersError> {
    env.storage()
        .instance()
        .get(&PARAMS_KEY)
        .ok_or(ParametersError::NotInitialized)
}

pub fn set_parameters(env: &Env, params: &ProtocolParameters) {
    env.storage().instance().set(&PARAMS_KEY, params);
}

pub fn is_reentrancy_locked(env: &Env) -> Result<bool, ParametersError> {
    Ok(env
        .storage()
        .instance()
        .get(&REENTRANCY_LOCK)
        .unwrap_or(false))
}

pub fn set_reentrancy_locked(env: &Env, locked: bool) {
    env.storage().instance().set(&REENTRANCY_LOCK, &locked);
}

pub fn get_version(env: &Env) -> Result<u32, ParametersError> {
    Ok(env.storage().instance().get(&VERSION_KEY).unwrap_or(1u32))
}

pub fn set_version(env: &Env, v: u32) {
    env.storage().instance().set(&VERSION_KEY, &v);
}

pub const SCHEMA_VERSION_KEY: Symbol = symbol_short!("SCH_VER");
pub const PERSISTENT_TTL_THRESHOLD: u32 = 1_036_800;
pub const PERSISTENT_TTL_EXTEND_TO: u32 = 2_073_600;

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
    env.storage()
        .persistent()
        .extend_ttl(&SCHEMA_VERSION_KEY, PERSISTENT_TTL_THRESHOLD, PERSISTENT_TTL_EXTEND_TO);
}

