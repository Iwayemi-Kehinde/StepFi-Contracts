use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::errors::ParametersError;
use crate::types::ProtocolParameters;

// Schema version key
pub const SCHEMA_VERSION_KEY: Symbol = symbol_short!("SCHEMA_V");
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Get the current schema version (0 = unset)
pub fn get_schema_version(env: &Env) -> u32 {
    env.storage().instance().get(&SCHEMA_VERSION_KEY).unwrap_or(0)
}

/// Set the schema version
pub fn set_schema_version(env: &Env, version: u32) {
    env.storage().instance().set(&SCHEMA_VERSION_KEY, &version);
}

pub const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
pub const PARAMS_KEY: Symbol = symbol_short!("PARAMS");

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
