# TrustLink Delegation Implementation TODO

Current working directory: /Users/mac/Documents/drips-2/TrustLink

## Plan Summary
Implement delegation where issuers can delegate attestation creation for specific claim_types to sub-issuers (delegates). Delegates do not need to be registered issuers.

## Steps (0/7 complete)

### 1. [ ] Update src/types.rs
- Add `Delegation` struct

### 2. [ ] Update src/errors.rs  
- Add delegation-specific errors

### 3. [ ] Update src/storage.rs
- Add `StorageKey::Delegation((Address, Address, String))`
- Add set_delegation/get_delegation/remove_delegation functions

### 4. [ ] Update src/events.rs
- Add delegation_created/delegation_revoked events

### 5. [ ] Update src/validation.rs
- Add is_valid_delegate_for_claim function

### 6. [ ] Update src/lib.rs - Part 1
- Add delegate_claim_type and revoke_delegation entrypoints
- Update create_attestation_internal auth check

### 7. [ ] Update src/lib.rs - Part 2
- Update create_attestations_batch auth check

### 8. [ ] Verify/Update Tests
- Check snapshot tests pass
- Run cargo test

## Commands to verify completion
```bash
cargo check
cargo test
```

**Next step: Implement step 1 (types.rs)**

