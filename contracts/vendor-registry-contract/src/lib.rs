#![no_std]

mod access;
mod errors;
mod events;
mod storage;
mod types;

#[cfg(test)]
mod tests;

use errors::Error;
use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal, String, Symbol, Val};
use types::VendorInfo;

// Export Error type for external use
pub use errors::Error as VendorRegistryError;

#[contract]
pub struct VendorRegistryContract;

#[contractimpl]
impl VendorRegistryContract {
    /// Initializes the contract with an admin
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if storage::has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }

        storage::set_admin(&env, &admin);

        Ok(())
    }

    /// Registers a new vendor
    pub fn register_vendor(
        env: Env,
        admin: Address,
        vendor: Address,
        name: String,
    ) -> Result<(), Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized);
        }

        access::require_admin(&env, &admin)?;

        if storage::has_vendor(&env, &vendor) {
            return Err(Error::VendorAlreadyRegistered);
        }

        if name.is_empty() || name.len() > 64 {
            return Err(Error::InvalidName);
        }

        let info = VendorInfo {
            name: name.clone(),
            registration_date: env.ledger().timestamp(),
            active: true,
            total_sales: 0,
        };

        storage::set_vendor(&env, &vendor, &info);
        storage::increment_vendor_count(&env)?;
        events::publish_vendor_registered(&env, vendor, name);

        Ok(())
    }

    /// Deactivates an existing vendor
    pub fn deactivate_vendor(env: Env, admin: Address, vendor: Address) -> Result<(), Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized);
        }

        access::require_admin(&env, &admin)?;
        let mut info = storage::get_vendor(&env, &vendor)?;
        info.active = false;
        storage::set_vendor(&env, &vendor, &info);
        events::publish_vendor_status(&env, vendor, false);

        Ok(())
    }

    /// Activates an existing vendor
    pub fn activate_vendor(env: Env, admin: Address, vendor: Address) -> Result<(), Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized);
        }

        access::require_admin(&env, &admin)?;
        let mut info = storage::get_vendor(&env, &vendor)?;
        info.active = true;
        storage::set_vendor(&env, &vendor, &info);
        events::publish_vendor_status(&env, vendor, true);

        Ok(())
    }

    /// Sets a vendor's active status (admin only).
    /// Pass `active = true` to activate, `active = false` to deactivate.
    pub fn set_vendor_status(
        env: Env,
        admin: Address,
        vendor: Address,
        active: bool,
    ) -> Result<(), Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized);
        }

        access::require_admin(&env, &admin)?;
        let mut info = storage::get_vendor(&env, &vendor)?;
        info.active = active;
        storage::set_vendor(&env, &vendor, &info);
        events::publish_vendor_status(&env, vendor, active);

        Ok(())
    }

    pub fn is_active(env: Env, vendor: Address) -> bool {
        storage::get_vendor(&env, &vendor)
            .map(|info| info.active)
            .unwrap_or(false)
    }

    pub fn get_vendor_info(env: Env, vendor: Address) -> Result<VendorInfo, Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized);
        }

        storage::get_vendor(&env, &vendor)
    }

    pub fn get_vendor_count(env: Env) -> Result<u64, Error> {
        if !storage::has_admin(&env) {
            return Err(Error::NotInitialized);
        }

        storage::get_vendor_count(&env)
    }

    /// Set the admin address for this contract.
    /// Requires authorization from the current admin.
    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        let old_admin = storage::get_admin(&env)?;
        old_admin.require_auth();
        access::require_admin(&env, &old_admin)?;

        storage::set_admin(&env, &new_admin);
        Ok(())
    }

    /// Migrate contract storage to the latest schema version.
    /// Called automatically during upgrade() to handle any data migrations.
    /// This is idempotent and safe to call multiple times.
    pub fn migrate(env: Env) -> Result<(), Error> {
        let stored = storage::get_schema_version(&env);
        let current = storage::CURRENT_SCHEMA_VERSION;
        if stored >= current {
            return Ok(());
        }
        // Future: add per-version data migration steps here
        storage::set_schema_version(&env, current);
        Ok(())
    }

    /// Upgrade the contract WASM — admin only.
    /// After replacing the WASM, automatically runs migrate() to ensure
    /// contract storage is up to date with the new code version.
    pub fn upgrade(env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) -> Result<(), Error> {
        let admin = storage::get_admin(&env)?;
        admin.require_auth();
        access::require_admin(&env, &admin)?;
        env.deployer().update_current_contract_wasm(new_wasm_hash);
        // Self-invoke migrate to run post-upgrade migration logic
        env.invoke_contract::<Val>(&env.current_contract_address(), &Symbol::new(&env, "migrate"), ().into_val(&env));
        Ok(())
    }
}
