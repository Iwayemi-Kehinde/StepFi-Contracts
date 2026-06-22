use soroban_sdk::{symbol_short, Address, Env, Map, Symbol};

use crate::errors::ReputationError;

// Storage keys for the reputation contract
pub const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
pub const UPDATERS_MAP: Symbol = symbol_short!("UPDATERS");
pub const SCORES_MAP: Symbol = symbol_short!("SCORES");
pub const REENTRANCY_LOCK: Symbol = symbol_short!("LOCKED");
pub const VERSION_KEY: Symbol = symbol_short!("VERSION");

/// Get the admin address from storage
pub fn get_admin(env: &Env) -> Result<Address, ReputationError> {
    env.storage()
        .instance()
        .get(&ADMIN_KEY)
        .ok_or(ReputationError::NotInitialized)
}

/// Set the admin address in storage
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN_KEY, admin);
}

/// Read a user's reputation score from storage
pub fn read_score(env: &Env, user: &Address) -> Result<u32, ReputationError> {
    let scores: Map<Address, u32> = env
        .storage()
        .instance()
        .get(&SCORES_MAP)
        .unwrap_or_else(|| Map::new(env));

    Ok(scores.get(user.clone()).unwrap_or(0))
}

/// Write a user's reputation score to storage
pub fn write_score(env: &Env, user: &Address, score: u32) {
    let mut scores: Map<Address, u32> = env
        .storage()
        .instance()
        .get(&SCORES_MAP)
        .unwrap_or_else(|| Map::new(env));

    scores.set(user.clone(), score);
    env.storage().instance().set(&SCORES_MAP, &scores);
}

/// Check if an address is an authorized updater
pub fn is_updater(env: &Env, addr: &Address) -> Result<bool, ReputationError> {
    let updaters: Map<Address, bool> = env
        .storage()
        .instance()
        .get(&UPDATERS_MAP)
        .unwrap_or_else(|| Map::new(env));

    Ok(updaters.get(addr.clone()).unwrap_or(false))
}

/// Set an address as an authorized updater
pub fn set_updater(env: &Env, updater: &Address, allowed: bool) {
    let mut updaters: Map<Address, bool> = env
        .storage()
        .instance()
        .get(&UPDATERS_MAP)
        .unwrap_or_else(|| Map::new(env));

    if allowed {
        updaters.set(updater.clone(), true);
    } else {
        updaters.remove(updater.clone());
    }

    env.storage().instance().set(&UPDATERS_MAP, &updaters);
}

pub fn is_reentrancy_locked(env: &Env) -> Result<bool, ReputationError> {
    Ok(env
        .storage()
        .instance()
        .get(&REENTRANCY_LOCK)
        .unwrap_or(false))
}

pub fn set_reentrancy_locked(env: &Env, locked: bool) {
    env.storage().instance().set(&REENTRANCY_LOCK, &locked);
}

pub fn get_version(env: &Env) -> Result<u32, ReputationError> {
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

