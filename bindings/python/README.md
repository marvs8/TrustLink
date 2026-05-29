# TrustLink Python SDK

Python client library for interacting with TrustLink on-chain attestation system on Stellar.

## Installation

```bash
pip install trustlink-sdk
```

## Quick Start

```python
from trustlink import TrustLinkClient

# Initialize client
client = TrustLinkClient(
    contract_id="CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQAHHAGK6Z6E",
    rpc_url="https://soroban-testnet.stellar.org:443"
)

# Check contract health
health = client.health_check()
print(f"Contract initialized: {health.initialized}")

# Get an attestation
attestation = client.get_attestation("attestation_id_here")
print(f"Attestation for {attestation.subject}: {attestation.claim_type}")

# Get audit log for compliance tracking
audit_log = client.get_audit_log("attestation_id_here")
for entry in audit_log:
    print(f"{entry.timestamp}: {entry.action.value} by {entry.actor}")
```

## Audit Log Usage

The `get_audit_log()` method is essential for compliance tools that need to reconstruct the full lifecycle of an attestation:

```python
# Get complete audit trail for an attestation
attestation_id = "att_123456789"
audit_entries = client.get_audit_log(attestation_id)

# Reconstruct attestation lifecycle
for entry in audit_entries:
    print(f"Action: {entry.action.value}")
    print(f"Timestamp: {entry.timestamp}")
    print(f"Actor: {entry.actor}")
    if entry.details:
        print(f"Details: {entry.details}")
    print("---")

# Example output:
# Action: Created
# Timestamp: 1640995200
# Actor: GDXLKEY5TR4IDEVSTRYUNYY3DPXQKQNSTDJ7HIVNFTJYQHOZXB7CRQME
# ---
# Action: Renewed
# Timestamp: 1672531200
# Actor: GDXLKEY5TR4IDEVSTRYUNYY3DPXQKQNSTDJ7HIVNFTJYQHOZXB7CRQME
# Details: Extended expiration by 365 days
# ---
```

## API Reference

### TrustLinkClient

Main client class for interacting with TrustLink contracts.

#### Methods

- `get_audit_log(attestation_id: str) -> List[AuditEntry]`: Get audit log for an attestation
- `get_attestation(attestation_id: str) -> Attestation`: Get attestation by ID
- `get_attestation_status(attestation_id: str) -> AttestationStatus`: Get attestation status
- `health_check() -> HealthStatus`: Get contract health status
- `get_global_stats() -> GlobalStats`: Get global contract statistics

### Types

#### AuditEntry

Represents an audit log entry:

```python
@dataclass
class AuditEntry:
    attestation_id: str
    action: AuditAction  # Created, Revoked, Renewed, Updated, Transferred
    timestamp: int
    actor: str
    details: Optional[str]
```

#### Attestation

Represents an attestation record:

```python
@dataclass
class Attestation:
    id: str
    issuer: str
    subject: str
    claim_type: str
    timestamp: int
    expiration: Optional[int]
    revoked: bool
    metadata: Optional[str]
    # ... additional fields
```

## Error Handling

```python
from trustlink import TrustLinkClient, Error

try:
    audit_log = client.get_audit_log("invalid_id")
except Error as e:
    print(f"TrustLink error {e.code}: {e.message}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

## Development

```bash
# Install development dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Format code
black trustlink/

# Type checking
mypy trustlink/
```

## License

MIT License - see LICENSE file for details.