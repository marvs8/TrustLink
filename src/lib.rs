#![no_std]

//! # TrustLink
//!
//! An on-chain attestation and verification system for the Stellar blockchain.
//!
//! Trusted issuers register with an admin, then create signed attestations about
//! wallet addresses. Any contract or dApp can query TrustLink to verify claims
//! before executing financial operations.

mod storage;
pub mod types;
mod validation;
mod events;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Vec};
use types::{Attestation, AttestationStatus, ClaimTypeInfo, ContractMetadata, Error, IssuerMetadata};
use storage::Storage;
use validation::Validation;
use events::Events;

/// The TrustLink smart contract.
#[contract]
pub struct TrustLinkContract;

#[contractimpl]
impl TrustLinkContract {
    /// Initialize the contract and set the administrator.
    ///
    /// Must be called exactly once after deployment.
    ///
    /// # Errors
    /// - [`Error::AlreadyInitialized`] — contract has already been initialized.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if Storage::has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        Storage::set_admin(&env, &admin);
        Storage::set_version(&env, &String::from_str(&env, "1.0.0"));
        Ok(())
    }

    /// Register an address as an authorized attestation issuer.
    ///
    /// # Errors
    /// - [`Error::NotInitialized`] — contract has not been initialized.
    /// - [`Error::Unauthorized`] — `admin` is not the registered administrator.
    pub fn register_issuer(env: Env, admin: Address, issuer: Address) -> Result<(), Error> {
        admin.require_auth();
        Validation::require_admin(&env, &admin)?;
        Storage::add_issuer(&env, &issuer);
        Events::issuer_registered(&env, &issuer, &admin);
        Ok(())
    }

    /// Remove an address from the authorized issuer registry.
    ///
    /// # Errors
    /// - [`Error::NotInitialized`] — contract has not been initialized.
    /// - [`Error::Unauthorized`] — `admin` is not the registered administrator.
    pub fn remove_issuer(env: Env, admin: Address, issuer: Address) -> Result<(), Error> {
        admin.require_auth();
        Validation::require_admin(&env, &admin)?;
        Storage::remove_issuer(&env, &issuer);
        Events::issuer_removed(&env, &issuer, &admin);
        Ok(())
    }

    /// Create a new attestation about a subject address.
    ///
    /// # Parameters
    /// - `issuer` — authorized issuer (must authorize).
    /// - `subject` — address the attestation is about.
    /// - `claim_type` — free-form claim label, e.g. `"KYC_PASSED"`.
    /// - `expiration` — optional Unix timestamp after which the attestation expires.
    /// - `metadata` — optional free-form string (max 256 chars).
    ///
    /// # Errors
    /// - [`Error::Unauthorized`] — `issuer` is not a registered issuer.
    /// - [`Error::DuplicateAttestation`] — same ID already exists.
    /// - [`Error::MetadataTooLong`] — metadata exceeds 256 characters.
    pub fn create_attestation(
        env: Env,
        issuer: Address,
        subject: Address,
        claim_type: String,
        expiration: Option<u64>,
        metadata: Option<String>,
    ) -> Result<String, Error> {
        issuer.require_auth();
        Validation::require_issuer(&env, &issuer)?;

        // Enforce 256-character limit on metadata
        if let Some(ref m) = metadata {
            if m.len() > 256 {
                return Err(Error::MetadataTooLong);
            }
        }

        let timestamp = env.ledger().timestamp();

        let attestation_id = Attestation::generate_id(
            &env,
            &issuer,
            &subject,
            &claim_type,
            timestamp,
        );

        if Storage::has_attestation(&env, &attestation_id) {
            return Err(Error::DuplicateAttestation);
        }

        let attestation = Attestation {
            id: attestation_id.clone(),
            issuer: issuer.clone(),
            subject: subject.clone(),
            claim_type: claim_type.clone(),
            timestamp,
            expiration,
            revoked: false,
            metadata,
        };

        Storage::set_attestation(&env, &attestation);
        Storage::add_subject_attestation(&env, &subject, &attestation_id);
        Storage::add_issuer_attestation(&env, &issuer, &attestation_id);

        Events::attestation_created(&env, &attestation);

        Ok(attestation_id)
    }

    /// Revoke an existing attestation.
    ///
    /// # Errors
    /// - [`Error::NotFound`] — no attestation with the given ID.
    /// - [`Error::Unauthorized`] — caller is not the original issuer.
    /// - [`Error::AlreadyRevoked`] — attestation already revoked.
    pub fn revoke_attestation(
        env: Env,
        issuer: Address,
        attestation_id: String,
    ) -> Result<(), Error> {
        issuer.require_auth();

        let mut attestation = Storage::get_attestation(&env, &attestation_id)?;

        if attestation.issuer != issuer {
            return Err(Error::Unauthorized);
        }

        if attestation.revoked {
            return Err(Error::AlreadyRevoked);
        }

        attestation.revoked = true;
        Storage::set_attestation(&env, &attestation);

        Events::attestation_revoked(&env, &attestation_id, &issuer);

        Ok(())
    }

    /// Revoke multiple attestations in a single call (issuer only).
    ///
    /// Auth is checked once for the issuer. Returns the count of revoked attestations.
    ///
    /// # Errors
    /// - [`Error::Unauthorized`] — issuer is not registered or doesn't own an attestation.
    /// - [`Error::AlreadyRevoked`] — an attestation in the batch is already revoked.
    pub fn revoke_attestations_batch(
        env: Env,
        issuer: Address,
        attestation_ids: Vec<String>,
    ) -> Result<u32, Error> {
        issuer.require_auth();
        Validation::require_issuer(&env, &issuer)?;

        let mut count: u32 = 0;

        for id in attestation_ids.iter() {
            let mut attestation = Storage::get_attestation(&env, &id)?;

            if attestation.issuer != issuer {
                return Err(Error::Unauthorized);
            }

            if attestation.revoked {
                return Err(Error::AlreadyRevoked);
            }

            attestation.revoked = true;
            Storage::set_attestation(&env, &attestation);
            Events::attestation_revoked(&env, &id, &issuer);

            count += 1;
        }

        Ok(count)
    }

    /// Renew an existing attestation with a new expiration (issuer only).
    ///
    /// # Errors
    /// - [`Error::NotFound`] — no attestation with the given ID.
    /// - [`Error::Unauthorized`] — caller is not the original issuer or not registered.
    /// - [`Error::AlreadyRevoked`] — attestation has been revoked.
    /// - [`Error::InvalidExpiration`] — new expiration is in the past.
    pub fn renew_attestation(
        env: Env,
        issuer: Address,
        attestation_id: String,
        new_expiration: Option<u64>,
    ) -> Result<(), Error> {
        issuer.require_auth();

        let mut attestation = Storage::get_attestation(&env, &attestation_id)?;

        if attestation.issuer != issuer {
            return Err(Error::Unauthorized);
        }

        Validation::require_issuer(&env, &issuer)?;

        if attestation.revoked {
            return Err(Error::AlreadyRevoked);
        }

        if let Some(t) = new_expiration {
            if t <= env.ledger().timestamp() {
                return Err(Error::InvalidExpiration);
            }
        }

        attestation.expiration = new_expiration;
        Storage::set_attestation(&env, &attestation);
        Events::attestation_renewed(&env, &attestation_id, &issuer, new_expiration);

        Ok(())
    }

    /// Update the expiration of an existing attestation.
    ///
    /// # Errors
    /// - [`Error::NotFound`] — no attestation with the given ID.
    /// - [`Error::Unauthorized`] — caller is not the original issuer.
    /// - [`Error::AlreadyRevoked`] — attestation has been revoked.
    pub fn update_expiration(
        env: Env,
        issuer: Address,
        attestation_id: String,
        new_expiration: Option<u64>,
    ) -> Result<(), Error> {
        issuer.require_auth();

        let mut attestation = Storage::get_attestation(&env, &attestation_id)?;

        if attestation.issuer != issuer {
            return Err(Error::Unauthorized);
        }

        if attestation.revoked {
            return Err(Error::AlreadyRevoked);
        }

        attestation.expiration = new_expiration;
        Storage::set_attestation(&env, &attestation);

        Events::attestation_updated(&env, &attestation_id, &issuer, new_expiration);

        Ok(())
    }

    /// Check if an address has a valid attestation of a given type.
    ///
    /// Emits an `expired` event for any expired (non-revoked) attestation encountered.
    pub fn has_valid_claim(
        env: Env,
        subject: Address,
        claim_type: String,
    ) -> bool {
        let attestation_ids = Storage::get_subject_attestations(&env, &subject);
        let current_time = env.ledger().timestamp();

        for id in attestation_ids.iter() {
            if let Ok(attestation) = Storage::get_attestation(&env, &id) {
                if attestation.claim_type == claim_type {
                    match attestation.get_status(current_time) {
                        AttestationStatus::Valid => return true,
                        AttestationStatus::Expired => {
                            Events::attestation_expired(&env, &id, &subject);
                        }
                        AttestationStatus::Revoked => {}
                    }
                }
            }
        }

        false
    }

    /// Check if an address has a valid attestation for any of the given claim types.
    ///
    /// Returns `false` if `claim_types` is empty.
    pub fn has_any_claim(env: Env, subject: Address, claim_types: Vec<String>) -> bool {
        if claim_types.is_empty() {
            return false;
        }
        let attestation_ids = Storage::get_subject_attestations(&env, &subject);
        let current_time = env.ledger().timestamp();
        for claim_type in claim_types.iter() {
            for id in attestation_ids.iter() {
                if let Ok(attestation) = Storage::get_attestation(&env, &id) {
                    if attestation.claim_type == claim_type
                        && attestation.get_status(current_time) == AttestationStatus::Valid
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if an address has valid attestations for ALL of the given claim types.
    ///
    /// Returns `true` if `claim_types` is empty (vacuous truth).
    /// Short-circuits and returns `false` as soon as any claim is missing/invalid.
    pub fn has_all_claims(env: Env, subject: Address, claim_types: Vec<String>) -> bool {
        if claim_types.is_empty() {
            return true;
        }
        let attestation_ids = Storage::get_subject_attestations(&env, &subject);
        let current_time = env.ledger().timestamp();

        'outer: for claim_type in claim_types.iter() {
            for id in attestation_ids.iter() {
                if let Ok(attestation) = Storage::get_attestation(&env, &id) {
                    if attestation.claim_type == claim_type
                        && attestation.get_status(current_time) == AttestationStatus::Valid
                    {
                        continue 'outer;
                    }
                }
            }
            // No valid attestation found for this claim type
            return false;
        }

        true
    }

    /// Fetch the full attestation record by ID.
    ///
    /// # Errors
    /// - [`Error::NotFound`] — no attestation with the given ID.
    pub fn get_attestation(
        env: Env,
        attestation_id: String,
    ) -> Result<Attestation, Error> {
        Storage::get_attestation(&env, &attestation_id)
    }

    /// Return the current status of an attestation.
    ///
    /// Emits an `expired` event when the status is [`AttestationStatus::Expired`].
    ///
    /// # Errors
    /// - [`Error::NotFound`] — no attestation with the given ID.
    pub fn get_attestation_status(
        env: Env,
        attestation_id: String,
    ) -> Result<AttestationStatus, Error> {
        let attestation = Storage::get_attestation(&env, &attestation_id)?;
        let current_time = env.ledger().timestamp();
        let status = attestation.get_status(current_time);
        if status == AttestationStatus::Expired {
            Events::attestation_expired(&env, &attestation_id, &attestation.subject);
        }
        Ok(status)
    }

    /// Return a paginated list of attestation IDs for a subject.
    pub fn get_subject_attestations(
        env: Env,
        subject: Address,
        start: u32,
        limit: u32,
    ) -> Vec<String> {
        let all_ids = Storage::get_subject_attestations(&env, &subject);
        let total = all_ids.len();
        let mut result = Vec::new(&env);
        let end = (start + limit).min(total);
        for i in start..end {
            if let Some(id) = all_ids.get(i) {
                result.push_back(id);
            }
        }
        result
    }

    /// Return a paginated list of attestation IDs created by an issuer.
    pub fn get_issuer_attestations(
        env: Env,
        issuer: Address,
        start: u32,
        limit: u32,
    ) -> Vec<String> {
        let all_ids = Storage::get_issuer_attestations(&env, &issuer);
        let total = all_ids.len();
        let mut result = Vec::new(&env);
        let end = (start + limit).min(total);
        for i in start..end {
            if let Some(id) = all_ids.get(i) {
                result.push_back(id);
            }
        }
        result
    }

    /// Return a deduplicated list of valid claim types for a subject.
    pub fn get_valid_claims(env: Env, subject: Address) -> Vec<String> {
        let attestation_ids = Storage::get_subject_attestations(&env, &subject);
        let current_time = env.ledger().timestamp();
        let mut result: Vec<String> = Vec::new(&env);

        for id in attestation_ids.iter() {
            if let Ok(attestation) = Storage::get_attestation(&env, &id) {
                if attestation.get_status(current_time) == AttestationStatus::Valid {
                    let mut already_present = false;
                    for existing in result.iter() {
                        if existing == attestation.claim_type {
                            already_present = true;
                            break;
                        }
                    }
                    if !already_present {
                        result.push_back(attestation.claim_type);
                    }
                }
            }
        }

        result
    }

    /// Find the most recent valid attestation for a subject by claim type.
    ///
    /// # Errors
    /// - [`Error::NotFound`] — no valid attestation of that type exists.
    pub fn get_attestation_by_type(
        env: Env,
        subject: Address,
        claim_type: String,
    ) -> Result<Attestation, Error> {
        let attestation_ids = Storage::get_subject_attestations(&env, &subject);
        let current_time = env.ledger().timestamp();
        let len = attestation_ids.len();

        let mut i = len;
        while i > 0 {
            i -= 1;
            if let Some(id) = attestation_ids.get(i) {
                if let Ok(attestation) = Storage::get_attestation(&env, &id) {
                    if attestation.claim_type == claim_type
                        && attestation.get_status(current_time) == AttestationStatus::Valid
                    {
                        return Ok(attestation);
                    }
                }
            }
        }

        Err(Error::NotFound)
    }

    /// Check whether an address is a registered issuer.
    pub fn is_issuer(env: Env, address: Address) -> bool {
        Storage::is_issuer(&env, &address)
    }

    /// Set metadata for the calling issuer.
    ///
    /// # Errors
    /// - [`Error::Unauthorized`] — `issuer` is not a registered issuer.
    pub fn set_issuer_metadata(
        env: Env,
        issuer: Address,
        metadata: IssuerMetadata,
    ) -> Result<(), Error> {
        issuer.require_auth();
        Validation::require_issuer(&env, &issuer)?;
        Storage::set_issuer_metadata(&env, &issuer, &metadata);
        Ok(())
    }

    /// Retrieve metadata for an issuer.
    pub fn get_issuer_metadata(env: Env, issuer: Address) -> Option<IssuerMetadata> {
        Storage::get_issuer_metadata(&env, &issuer)
    }

    /// Return the current administrator address.
    ///
    /// # Errors
    /// - [`Error::NotInitialized`] — contract has not been initialized.
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        Storage::get_admin(&env)
    }

    /// Register a known claim type with a human-readable description (admin only).
    ///
    /// # Errors
    /// - [`Error::NotInitialized`] — contract has not been initialized.
    /// - [`Error::Unauthorized`] — `admin` is not the registered administrator.
    pub fn register_claim_type(
        env: Env,
        admin: Address,
        claim_type: String,
        description: String,
    ) -> Result<(), Error> {
        admin.require_auth();
        Validation::require_admin(&env, &admin)?;

        let info = ClaimTypeInfo { claim_type: claim_type.clone(), description: description.clone() };
        Storage::set_claim_type(&env, &info);
        env.events().publish(
            (symbol_short!("clmtype"), claim_type.clone()),
            description.clone(),
        );
        Ok(())
    }

    /// Return the description for a registered claim type, or `None` if unknown.
    pub fn get_claim_type_description(env: Env, claim_type: String) -> Option<String> {
        Storage::get_claim_type(&env, &claim_type).map(|info| info.description)
    }

    /// Return a paginated list of registered claim type identifiers.
    pub fn list_claim_types(env: Env, start: u32, limit: u32) -> Vec<String> {
        let all = Storage::get_claim_type_list(&env);
        let total = all.len();
        let mut result = Vec::new(&env);
        let end = (start + limit).min(total);
        for i in start..end {
            if let Some(ct) = all.get(i) {
                result.push_back(ct);
            }
        }
        result
    }

    /// Return the semver version string set at initialization.
    ///
    /// # Errors
    /// - [`Error::NotInitialized`] — contract has not been initialized.
    pub fn get_version(env: Env) -> Result<String, Error> {
        Storage::get_version(&env).ok_or(Error::NotInitialized)
    }

    /// Return static metadata about this contract.
    ///
    /// # Errors
    /// - [`Error::NotInitialized`] — contract has not been initialized.
    pub fn get_contract_metadata(env: Env) -> Result<ContractMetadata, Error> {
        let version = Storage::get_version(&env).ok_or(Error::NotInitialized)?;
        Ok(ContractMetadata {
            name: String::from_str(&env, "TrustLink"),
            version,
            description: String::from_str(
                &env,
                "On-chain attestation and verification system for the Stellar blockchain.",
            ),
        })
    }
}
