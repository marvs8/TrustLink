//! Core types for TrustLink.
//!
//! Defines the [`Attestation`] struct, [`AttestationStatus`] enum, [`Error`]
//! codes, and supporting metadata types used throughout the contract.

use soroban_sdk::{contracttype, contracterror, Address, Env, String};

/// Contract metadata returned by `get_contract_metadata`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractMetadata {
    /// Contract name.
    pub name: String,
    /// Semver version string, e.g. `"1.0.0"`.
    pub version: String,
    /// Short description of the contract.
    pub description: String,
}

/// A single attestation record stored on-chain.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    /// Deterministic hash-based identifier for this attestation.
    pub id: String,
    /// Address that created the attestation.
    pub issuer: Address,
    /// Address the attestation is about.
    pub subject: Address,
    /// Free-form claim label, e.g. `"KYC_PASSED"`.
    pub claim_type: String,
    /// Ledger timestamp (seconds) when the attestation was created.
    pub timestamp: u64,
    /// Optional Unix timestamp after which the attestation is expired.
    pub expiration: Option<u64>,
    /// `true` if the issuer has explicitly revoked this attestation.
    pub revoked: bool,
    /// Optional free-form metadata string (max 256 characters).
    pub metadata: Option<String>,
}

/// Metadata an issuer can associate with their address.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuerMetadata {
    /// Human-readable display name for the issuer.
    pub name: String,
    /// URL pointing to the issuer's website or documentation.
    pub url: String,
    /// Short description of the issuer and the claims they issue.
    pub description: String,
}

/// Info stored for a registered claim type.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimTypeInfo {
    /// Claim type identifier string.
    pub claim_type: String,
    /// Human-readable description.
    pub description: String,
}

/// The current validity state of an attestation.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttestationStatus {
    /// Attestation is active and has not expired.
    Valid,
    /// Attestation has passed its expiration timestamp.
    Expired,
    /// Attestation was explicitly revoked by its issuer.
    Revoked,
}

/// Errors returned by TrustLink contract functions.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Contract has not been initialized.
    NotInitialized = 1,
    /// [`initialize`] was called more than once.
    AlreadyInitialized = 2,
    /// The caller lacks the required admin or issuer role.
    Unauthorized = 3,
    /// No attestation exists with the requested ID.
    NotFound = 4,
    /// The attestation has already been revoked.
    AlreadyRevoked = 5,
    /// An attestation with the same deterministic ID already exists.
    DuplicateAttestation = 6,
    /// The provided expiration timestamp is in the past.
    InvalidExpiration = 7,
    /// The provided metadata exceeds the maximum allowed length of 256 characters.
    MetadataTooLong = 8,
}

impl Attestation {
    /// Generate a deterministic attestation ID by SHA-256 hashing
    /// `(issuer, subject, claim_type, timestamp)`.
    pub fn generate_id(
        env: &Env,
        issuer: &Address,
        subject: &Address,
        claim_type: &String,
        timestamp: u64,
    ) -> String {
        use soroban_sdk::xdr::ToXdr;
        use soroban_sdk::Bytes;

        let mut data = Bytes::new(env);

        let issuer_xdr = issuer.clone().to_xdr(env);
        data.append(&issuer_xdr);

        let subject_xdr = subject.clone().to_xdr(env);
        data.append(&subject_xdr);

        let claim_bytes = claim_type.clone().to_xdr(env);
        data.append(&claim_bytes);

        let ts_bytes = timestamp.to_be_bytes();
        data.append(&Bytes::from_array(env, &ts_bytes));

        let hash = env.crypto().sha256(&data);
        let hex_chars: &[u8] = b"0123456789abcdef";
        let hash_bytes = hash.to_array();

        let mut arr = [0u8; 64];
        for (i, byte) in hash_bytes.iter().enumerate() {
            arr[i * 2] = hex_chars[(byte >> 4) as usize];
            arr[i * 2 + 1] = hex_chars[(byte & 0xf) as usize];
        }
        String::from_bytes(env, &arr)
    }

    /// Compute the current [`AttestationStatus`] given `current_time`.
    pub fn get_status(&self, current_time: u64) -> AttestationStatus {
        if self.revoked {
            return AttestationStatus::Revoked;
        }
        if let Some(exp) = self.expiration {
            if current_time > exp {
                return AttestationStatus::Expired;
            }
        }
        AttestationStatus::Valid
    }
}
