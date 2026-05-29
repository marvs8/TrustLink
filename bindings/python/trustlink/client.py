"""TrustLink Python Client"""

import json
from typing import List, Optional, Dict, Any
from stellar_sdk import Keypair, Server, TransactionBuilder, Network
from stellar_sdk.contract import Contract
from stellar_sdk.xdr import SCVal

from .types import (
    Attestation,
    AttestationStatus,
    AuditEntry,
    AuditAction,
    ContractConfig,
    FeeConfig,
    TtlConfig,
    IssuerMetadata,
    IssuerStats,
    IssuerTier,
    GlobalStats,
    HealthStatus,
    Error,
)


class TrustLinkClient:
    """Python client for TrustLink smart contract"""
    
    def __init__(
        self,
        contract_id: str,
        rpc_url: str = "https://soroban-testnet.stellar.org:443",
        network_passphrase: str = Network.TESTNET_NETWORK_PASSPHRASE,
    ):
        """Initialize TrustLink client
        
        Args:
            contract_id: The deployed contract address
            rpc_url: Soroban RPC endpoint URL
            network_passphrase: Stellar network passphrase
        """
        self.contract_id = contract_id
        self.server = Server(rpc_url)
        self.network_passphrase = network_passphrase
        self.contract = Contract(contract_id)
    
    def _simulate_call(self, method: str, args: List[SCVal]) -> Any:
        """Simulate a contract call and return the result"""
        # Create a dummy source account for simulation
        source_keypair = Keypair.random()
        source_account = self.server.load_account(source_keypair.public_key)
        
        # Build transaction for simulation
        transaction = (
            TransactionBuilder(
                source_account=source_account,
                network_passphrase=self.network_passphrase,
                base_fee=100,
            )
            .add_host_function_op(
                host_function=self.contract.get_footprint_and_invoke_contract_op(
                    method, args
                ).host_function
            )
            .set_timeout(30)
            .build()
        )
        
        # Simulate the transaction
        response = self.server.simulate_transaction(transaction)
        
        if response.error:
            raise Error(0, f"Simulation failed: {response.error}")
        
        # Parse the result
        if response.result and response.result.auth:
            return self._parse_scval(response.result.auth[0])
        
        return None
    
    def _parse_scval(self, scval: SCVal) -> Any:
        """Parse SCVal to Python types"""
        # This is a simplified parser - in practice you'd need more comprehensive parsing
        if hasattr(scval, 'str'):
            return scval.str.decode('utf-8')
        elif hasattr(scval, 'u64'):
            return int(scval.u64)
        elif hasattr(scval, 'bool'):
            return bool(scval.bool)
        elif hasattr(scval, 'vec'):
            return [self._parse_scval(item) for item in scval.vec]
        elif hasattr(scval, 'map'):
            result = {}
            for item in scval.map:
                key = self._parse_scval(item.key)
                val = self._parse_scval(item.val)
                result[key] = val
            return result
        else:
            return str(scval)
    
    def _str_to_scval(self, s: str) -> SCVal:
        """Convert string to SCVal"""
        return SCVal.scv_string(s.encode('utf-8'))
    
    def _u64_to_scval(self, n: int) -> SCVal:
        """Convert int to u64 SCVal"""
        return SCVal.scv_u64(n)
    
    def _bool_to_scval(self, b: bool) -> SCVal:
        """Convert bool to SCVal"""
        return SCVal.scv_bool(b)
    
    def get_audit_log(self, attestation_id: str) -> List[AuditEntry]:
        """Get the audit log for an attestation
        
        Args:
            attestation_id: The attestation ID to get audit log for
            
        Returns:
            List of AuditEntry objects representing the attestation's lifecycle
            
        Raises:
            Error: If the attestation is not found or other contract error
        """
        try:
            args = [self._str_to_scval(attestation_id)]
            result = self._simulate_call("get_audit_log", args)
            
            if not result:
                return []
            
            # Parse the result into AuditEntry objects
            audit_entries = []
            for entry_data in result:
                if isinstance(entry_data, dict):
                    audit_entry = AuditEntry(
                        attestation_id=entry_data.get("attestation_id", attestation_id),
                        action=AuditAction(entry_data.get("action", "Created")),
                        timestamp=int(entry_data.get("timestamp", 0)),
                        actor=entry_data.get("actor", ""),
                        details=entry_data.get("details")
                    )
                    audit_entries.append(audit_entry)
            
            return audit_entries
            
        except Exception as e:
            raise Error(0, f"Failed to get audit log: {str(e)}")
    
    def get_attestation(self, attestation_id: str) -> Attestation:
        """Get an attestation by ID"""
        try:
            args = [self._str_to_scval(attestation_id)]
            result = self._simulate_call("get_attestation", args)
            
            if not result:
                raise Error(4, "Attestation not found")
            
            # Parse result into Attestation object
            return Attestation(
                id=result.get("id", ""),
                issuer=result.get("issuer", ""),
                subject=result.get("subject", ""),
                claim_type=result.get("claim_type", ""),
                timestamp=int(result.get("timestamp", 0)),
                expiration=result.get("expiration"),
                revoked=bool(result.get("revoked", False)),
                metadata=result.get("metadata"),
                jurisdiction=result.get("jurisdiction"),
                valid_from=result.get("valid_from"),
                imported=bool(result.get("imported", False)),
                bridged=bool(result.get("bridged", False)),
                source_chain=result.get("source_chain"),
                source_tx=result.get("source_tx"),
                tags=result.get("tags"),
                revocation_reason=result.get("revocation_reason"),
                deleted=bool(result.get("deleted", False))
            )
            
        except Exception as e:
            raise Error(0, f"Failed to get attestation: {str(e)}")
    
    def get_attestation_status(self, attestation_id: str) -> AttestationStatus:
        """Get the status of an attestation"""
        try:
            args = [self._str_to_scval(attestation_id)]
            result = self._simulate_call("get_attestation_status", args)
            return AttestationStatus(result)
        except Exception as e:
            raise Error(0, f"Failed to get attestation status: {str(e)}")
    
    def health_check(self) -> HealthStatus:
        """Get contract health status"""
        try:
            result = self._simulate_call("health_check", [])
            return HealthStatus(
                initialized=bool(result.get("initialized", False)),
                admin_set=bool(result.get("admin_set", False)),
                issuer_count=int(result.get("issuer_count", 0)),
                total_attestations=int(result.get("total_attestations", 0))
            )
        except Exception as e:
            raise Error(0, f"Failed to get health status: {str(e)}")
    
    def get_global_stats(self) -> GlobalStats:
        """Get global contract statistics"""
        try:
            result = self._simulate_call("get_global_stats", [])
            return GlobalStats(
                total_attestations=int(result.get("total_attestations", 0)),
                total_revocations=int(result.get("total_revocations", 0)),
                total_issuers=int(result.get("total_issuers", 0))
            )
        except Exception as e:
            raise Error(0, f"Failed to get global stats: {str(e)}")