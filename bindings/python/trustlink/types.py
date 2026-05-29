"""TrustLink type definitions"""

from dataclasses import dataclass
from typing import Optional, List
from enum import Enum


class AttestationStatus(Enum):
    """Status of an attestation"""
    VALID = "Valid"
    EXPIRED = "Expired"
    REVOKED = "Revoked"
    PENDING = "Pending"


class AuditAction(Enum):
    """Actions recorded in audit logs"""
    CREATED = "Created"
    REVOKED = "Revoked"
    RENEWED = "Renewed"
    UPDATED = "Updated"
    TRANSFERRED = "Transferred"


class IssuerTier(Enum):
    """Trust tier for registered issuers"""
    BASIC = 0
    VERIFIED = 1
    PREMIUM = 2


@dataclass
class Attestation:
    """An attestation record"""
    id: str
    issuer: str
    subject: str
    claim_type: str
    timestamp: int
    expiration: Optional[int]
    revoked: bool
    metadata: Optional[str]
    jurisdiction: Optional[str]
    valid_from: Optional[int]
    imported: bool
    bridged: bool
    source_chain: Optional[str]
    source_tx: Optional[str]
    tags: Optional[List[str]]
    revocation_reason: Optional[str]
    deleted: bool


@dataclass
class AuditEntry:
    """An audit log entry tracking attestation lifecycle events"""
    attestation_id: str
    action: AuditAction
    timestamp: int
    actor: str
    details: Optional[str]


@dataclass
class FeeConfig:
    """Fee configuration for attestation creation"""
    attestation_fee: int
    fee_collector: str
    fee_token: Optional[str]


@dataclass
class TtlConfig:
    """Time-to-live configuration"""
    ttl_days: int


@dataclass
class ContractConfig:
    """Complete contract configuration"""
    ttl_config: TtlConfig
    fee_config: FeeConfig
    contract_name: str
    contract_version: str
    contract_description: str


@dataclass
class IssuerMetadata:
    """Metadata for a registered issuer"""
    name: str
    url: str
    description: str


@dataclass
class IssuerStats:
    """Statistics for an issuer"""
    total_issued: int


@dataclass
class GlobalStats:
    """Global contract statistics"""
    total_attestations: int
    total_revocations: int
    total_issuers: int


@dataclass
class HealthStatus:
    """Contract health status"""
    initialized: bool
    admin_set: bool
    issuer_count: int
    total_attestations: int


class Error(Exception):
    """TrustLink contract error"""
    
    def __init__(self, code: int, message: str):
        self.code = code
        self.message = message
        super().__init__(f"Error {code}: {message}")