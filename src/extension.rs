//! Defines an extension trait for Sommelier's gravity module queries and messages
use async_trait::async_trait;
use eyre::{bail, Report, Result};
use gravity_proto::gravity::*;
use ocular::{
    grpc::{GrpcClient, PageRequest, ConstructClient}, cosmrs::Coin, tx::{UnsignedTx, ModuleMsg},
};
use prost_types::Any;

pub type SommGravityParams = gravity_proto::gravity::Params;

/// The (Sommelier) gravity module's query client proto definition wrapper
pub struct SommGravityQueryClient {
    inner: gravity_proto::gravity::query_client::QueryClient<tonic::transport::Channel>,
}


#[async_trait]
impl ConstructClient<SommGravityQueryClient> for SommGravityQueryClient {
    async fn new_client(endpoint: String) -> Result<Self> {
        Ok(Self {
            inner: gravity_proto::gravity::query_client::QueryClient::<tonic::transport::Channel>::connect(endpoint.to_owned()).await?
        })
    }
}

#[async_trait(?Send)]
pub trait SommGravityExt {
    async fn query_somm_gravity_params(&self) -> Result<ParamsResponse>;
    async fn query_signer_set_tx(&self, nonce: u64) -> Result<SignerSetTxResponse>;
    async fn query_latest_signer_set_tx(&self) -> Result<SignerSetTxResponse>;
    async fn query_batch_tx(&self, token_contract_address: &str, nonce: u64) -> Result<BatchTxResponse>;
    async fn query_contract_call_tx(&self, invalidation_scope: Vec<u8>, invalidation_nonce: u64) -> Result<ContractCallTxResponse>;
    async fn query_signer_set_txs(&self, pagination: Option<PageRequest>)
        -> Result<SignerSetTxsResponse>;
    async fn query_batch_txs(&self, pagination: Option<PageRequest>) -> Result<BatchTxsResponse>;
    async fn query_contract_call_txs(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<ContractCallTxsResponse>;
    async fn query_signer_set_tx_confirmations(
        &self,
        nonce: u64,
    ) -> Result<SignerSetTxConfirmationsResponse>;
    async fn query_batch_tx_confirmations(
        &self,
        nonce: u64,
        token_contract_address: &str,
    ) -> Result<BatchTxConfirmationsResponse>;
    async fn query_contract_call_tx_confirmations(
        &self,
        invalidation_scope: Vec<u8>,
        invalidation_nonce: u64,
    ) -> Result<ContractCallTxConfirmationsResponse>;
    async fn query_unsigned_signer_set_txs(&self, address: &str) -> Result<UnsignedSignerSetTxsResponse>;
    async fn query_unsigned_batch_txs(&self, address: &str) -> Result<UnsignedBatchTxsResponse>;
    async fn query_unsigned_contract_call_txs(&self, address: &str) -> Result<UnsignedContractCallTxsResponse>;
    async fn query_last_submitted_ethereum_event(
        &self,
        address: &str,
    ) -> Result<LastSubmittedEthereumEventResponse>;
    async fn query_erc20_to_denom(&self, erc20: &str) -> Result<String>;
    async fn query_denom_to_erc20_params(&self, denom: &str) -> Result<DenomToErc20ParamsResponse>;
    async fn query_denom_to_erc20(&self, denom: &str) -> Result<String>;
    async fn query_delegate_keys_by_validator(
        &self,
        validator_address: &str,
    ) -> Result<DelegateKeysByValidatorResponse>;
    async fn query_delegate_keys_by_ethereum_signer(
        &self,
        ethereum_signer_address: &str,
    ) -> Result<DelegateKeysByEthereumSignerResponse>;
    async fn query_delegate_keys_by_orchestrator(
        &self,
        orchestrator_address: &str,
    ) -> Result<DelegateKeysByOrchestratorResponse>;
    async fn query_delegate_keys(&self) -> Result<DelegateKeysResponse>;
    async fn query_batched_send_to_ethereums(
        &self,
        sender_address: &str,
    ) -> Result<BatchedSendToEthereumsResponse>;
    async fn query_unbatched_send_to_ethereums(
        &self,
        sender_address: &str,
        paginationi: Option<PageRequest>,
    ) -> Result<UnbatchedSendToEthereumsResponse>;
}

#[async_trait(?Send)]
impl SommGravityExt for GrpcClient {
    async fn query_somm_gravity_params(&self) -> Result<ParamsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = ParamsRequest {};

        Ok(client.inner.params(request).await?.into_inner())
    }

    async fn query_signer_set_tx(&self, nonce: u64) -> Result<SignerSetTxResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = SignerSetTxRequest {
            signer_set_nonce: nonce,
        };

        Ok(client.inner.signer_set_tx(request).await?.into_inner())
    }

    async fn query_latest_signer_set_tx(&self) -> Result<SignerSetTxResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = LatestSignerSetTxRequest {};

        Ok(client.inner.latest_signer_set_tx(request).await?.into_inner())
    }

    async fn query_batch_tx(&self, token_contract_address: &str, nonce: u64) -> Result<BatchTxResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = BatchTxRequest {
            token_contract: token_contract_address.to_string(),
            batch_nonce: nonce,
        };

        Ok(client.inner.batch_tx(request).await?.into_inner())
    }

    async fn query_contract_call_tx(&self, invalidation_scope: Vec<u8>, invalidation_nonce: u64) -> Result<ContractCallTxResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = ContractCallTxRequest {
            invalidation_scope,
            invalidation_nonce,
        };

        Ok(client.inner.contract_call_tx(request).await?.into_inner())
    }

    async fn query_signer_set_txs(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<SignerSetTxsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = SignerSetTxsRequest {
            pagination,
        };

        Ok(client.inner.signer_set_txs(request).await?.into_inner())
    }

    async fn query_batch_txs(&self, pagination: Option<PageRequest>) -> Result<BatchTxsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = BatchTxsRequest {
            pagination,
        };

        Ok(client.inner.batch_txs(request).await?.into_inner())
    }

    async fn query_contract_call_txs(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<ContractCallTxsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = ContractCallTxsRequest {
            pagination,
        };

        Ok(client.inner.contract_call_txs(request).await?.into_inner())
    }

    async fn query_signer_set_tx_confirmations(
        &self,
        nonce: u64,
    ) -> Result<SignerSetTxConfirmationsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = SignerSetTxConfirmationsRequest {
            signer_set_nonce: nonce,
        };

        Ok(client.inner.signer_set_tx_confirmations(request).await?.into_inner())
    }

    async fn query_batch_tx_confirmations(
        &self,
        nonce: u64,
        token_contract_address: &str,
    ) -> Result<BatchTxConfirmationsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = BatchTxConfirmationsRequest {
            token_contract: token_contract_address.to_string(),
            batch_nonce: nonce,
        };

        Ok(client.inner.batch_tx_confirmations(request).await?.into_inner())
    }

    async fn query_contract_call_tx_confirmations(
        &self,
        invalidation_scope: Vec<u8>,
        invalidation_nonce: u64,
    ) -> Result<ContractCallTxConfirmationsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = ContractCallTxConfirmationsRequest {
            invalidation_scope,
            invalidation_nonce,
        };

        Ok(client.inner.contract_call_tx_confirmations(request).await?.into_inner())
    }

    async fn query_unsigned_signer_set_txs(
        &self,
        address: &str,
    ) -> Result<UnsignedSignerSetTxsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = UnsignedSignerSetTxsRequest {
            address: address.to_string(),
        };

        Ok(client.inner.unsigned_signer_set_txs(request).await?.into_inner())
    }

    async fn query_unsigned_batch_txs(
        &self,
        address: &str,
    ) -> Result<UnsignedBatchTxsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = UnsignedBatchTxsRequest {
            address: address.to_string(),
        };

        Ok(client.inner.unsigned_batch_txs(request).await?.into_inner())
    }

    async fn query_unsigned_contract_call_txs(
        &self,
        address: &str,
    ) -> Result<UnsignedContractCallTxsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = UnsignedContractCallTxsRequest {
            address: address.to_string(),
        };

        Ok(client.inner.unsigned_contract_call_txs(request).await?.into_inner())
    }

    async fn query_last_submitted_ethereum_event(
        &self,
        address: &str,
    ) -> Result<LastSubmittedEthereumEventResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = LastSubmittedEthereumEventRequest {
            address: address.to_string(),
        };

        Ok(client.inner.last_submitted_ethereum_event(request).await?.into_inner())
    }

    async fn query_erc20_to_denom(&self, erc20: &str) -> Result<String> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = Erc20ToDenomRequest {
            erc20: erc20.to_string(),
        };

        Ok(client.inner.erc20_to_denom(request).await?.into_inner().denom)
    }

    async fn query_denom_to_erc20_params(&self, denom: &str) -> Result<DenomToErc20ParamsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = DenomToErc20ParamsRequest {
            denom: denom.to_string(),
        };

        Ok(client.inner.denom_to_erc20_params(request).await?.into_inner())
    }

    async fn query_denom_to_erc20(&self, denom: &str) -> Result<String> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = DenomToErc20Request {
            denom: denom.to_string(),
        };

        Ok(client.inner.denom_to_erc20(request).await?.into_inner().erc20)
    }

    async fn query_delegate_keys_by_validator(
        &self,
        validator_address: &str,
    ) -> Result<DelegateKeysByValidatorResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = DelegateKeysByValidatorRequest {
            validator_address: validator_address.to_string(),
        };

        Ok(client.inner.delegate_keys_by_validator(request).await?.into_inner())
    }

    async fn query_delegate_keys_by_ethereum_signer(
        &self,
        ethereum_signer_address: &str,
    ) -> Result<DelegateKeysByEthereumSignerResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = DelegateKeysByEthereumSignerRequest {
            ethereum_signer: ethereum_signer_address.to_string(),
        };

        Ok(client.inner.delegate_keys_by_ethereum_signer(request).await?.into_inner())
    }

    async fn query_delegate_keys_by_orchestrator(
        &self,
        orchestrator_address: &str,
    ) -> Result<DelegateKeysByOrchestratorResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = DelegateKeysByOrchestratorRequest {
            orchestrator_address: orchestrator_address.to_string(),
        };

        Ok(client.inner.delegate_keys_by_orchestrator(request).await?.into_inner())
    }

    async fn query_delegate_keys(&self) -> Result<DelegateKeysResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = DelegateKeysRequest {};

        Ok(client.inner.delegate_keys(request).await?.into_inner())
    }

    async fn query_batched_send_to_ethereums(
        &self,
        sender_address: &str,
    ) -> Result<BatchedSendToEthereumsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = BatchedSendToEthereumsRequest {
            sender_address: sender_address.to_string(),
        };

        Ok(client.inner.batched_send_to_ethereums(request).await?.into_inner())
    }

    async fn query_unbatched_send_to_ethereums(
        &self,
        sender_address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<UnbatchedSendToEthereumsResponse> {
        let mut client = SommGravityQueryClient::new_client(self.grpc_endpoint()).await?;
        let request = UnbatchedSendToEthereumsRequest {
            sender_address: sender_address.to_string(),
            pagination,
        };

        Ok(client.inner.unbatched_send_to_ethereums(request).await?.into_inner())
    }
}

pub enum SommGravity<'m> {
    /// Represents a MsgSendToEthereum
    SendToEthereum {
        sender: &'m str,
        ethereum_recipient: &'m str,
        amount: Coin,
        bridge_fee: Coin,
    },
    /// Represents a MsgCancelSendToEthereum
    CancelSendToEthereum { sender: &'m str, id: u64 },
    /// Represents a MsgRequestBatchTx
    RequestBatchTx { denom: &'m str, signer: &'m str },
    /// Represents a MsgSubmitEthereumTxConfirmation
    SubmitEthereumTxConfirmation { confirmation: Any, signer: &'m str },
    /// Represent a ContractCallTxConfirmation
    ContractCallTxConfirmation {
        invalidation_scope: Vec<u8>,
        invalidation_nonce: u64,
        ethereum_signer: &'m str,
        signature: Vec<u8>,
    },
    /// Represents a BatchTxConfirmation
    BatchTxConfirmation {
        token_contract_address: &'m str,
        batch_nonce: u64,
        ethereum_signer: &'m str,
        signature: Vec<u8>,
    },
    /// Represents a SignerSetTxConfirmation
    SignerSetTxConfirmation {
        signer_set_nonce: u64,
        ethereum_signer: &'m str,
        signature: Vec<u8>,
    },
    /// Represents a MsgSubmitEthereumEvent
    SubmitEthereumEvent { event: Any, signer: &'m str },
    /// Represents a MsgSetDelegateKeys
    SetDelegateKeys {
        validator_address: &'m str,
        orchestrator_address: &'m str,
        ethereum_address: &'m str,
        eth_signature: Vec<u8>,
    },
    /// Represents a DelegateKeysMsg
    DelegateKeysSignMsg {
        validator_address: &'m str,
        nonce: u64,
    },
    /// Represents a MsgSubmitEthereumHeightVote
    SubmitEthereumHeightVote {
        ethereum_height: u64,
        signer: &'m str,
    },
}

impl ModuleMsg for SommGravity<'_> {
    type Error = Report;

    /// Converts the enum into an [`Any`] for use in a transaction
    fn into_any(self) -> Result<Any> {
        match self {
            SommGravity::SendToEthereum {
                sender,
                ethereum_recipient,
                amount,
                bridge_fee,
            } => {
                let msg = gravity_proto::gravity::MsgSendToEthereum {
                    sender: sender.to_string(),
                    ethereum_recipient: ethereum_recipient.to_string(),
                    amount: Some(amount.into()),
                    bridge_fee: Some(bridge_fee.into()),
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode MsgSendToEthereum: {}", e)
                };
                any.type_url = "/gravity.v1.MsgSendToEthereum".to_string();
                Ok(any)
            },
            SommGravity::CancelSendToEthereum { sender, id } => {
                let msg = gravity_proto::gravity::MsgCancelSendToEthereum {
                    sender: sender.to_string(),
                    id,
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode MsgCancelSendToEthereum: {}", e)
                };
                any.type_url = "/gravity.v1.MsgCancelSendToEthereum".to_string();
                Ok(any)
            },
            SommGravity::RequestBatchTx { denom, signer } => {
                let msg = gravity_proto::gravity::MsgRequestBatchTx {
                    denom: denom.to_string(),
                    signer: signer.to_string(),
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode MsgRequestBatchTx: {}", e)
                };
                any.type_url = "/gravity.v1.MsgRequestBatchTx".to_string();
                Ok(any)
            },
            SommGravity::SubmitEthereumTxConfirmation {
                confirmation,
                signer,
            } => {
                let msg = gravity_proto::gravity::MsgSubmitEthereumTxConfirmation {
                    confirmation: Some(confirmation),
                    signer: signer.to_string(),
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode MsgSubmitEthereumTxConfirmation: {}", e)
                };
                any.type_url = "/gravity.v1.MsgSubmitEthereumTxConfirmation".to_string();
                Ok(any)
            },
            SommGravity::ContractCallTxConfirmation {
                invalidation_scope,
                invalidation_nonce,
                ethereum_signer,
                signature,
            } => {
                let msg = gravity_proto::gravity::ContractCallTxConfirmation {
                    invalidation_scope,
                    invalidation_nonce,
                    ethereum_signer: ethereum_signer.to_string(),
                    signature,
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode ContractCallTxConfirmation: {}", e)
                };
                any.type_url = "/gravity.v1.ContractCallTxConfirmation".to_string();
                Ok(any)
            },
            SommGravity::BatchTxConfirmation {
                token_contract_address,
                batch_nonce,
                ethereum_signer,
                signature,
            } => {
                let msg = gravity_proto::gravity::BatchTxConfirmation {
                    token_contract: token_contract_address.to_string(),
                    batch_nonce,
                    ethereum_signer: ethereum_signer.to_string(),
                    signature,
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode BatchTxConfirmation: {}", e)
                };
                any.type_url = "/gravity.v1.BatchTxConfirmation".to_string();
                Ok(any)
            },
            SommGravity::SignerSetTxConfirmation {
                signer_set_nonce,
                ethereum_signer,
                signature,
            } => {
                let msg = gravity_proto::gravity::SignerSetTxConfirmation {
                    signer_set_nonce,
                    ethereum_signer: ethereum_signer.to_string(),
                    signature,
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode SignerSetTxConfirmation: {}", e)
                };
                any.type_url = "/gravity.v1.SignerSetTxConfirmation".to_string();
                Ok(any)
            },
            SommGravity::SubmitEthereumEvent { event, signer } => {
                let msg = gravity_proto::gravity::MsgSubmitEthereumEvent {
                    event: Some(event),
                    signer: signer.to_string(),
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode MsgSubmitEthereumEvent: {}", e)
                };
                any.type_url = "/gravity.v1.MsgSubmitEthereumEvent".to_string();
                Ok(any)
            },
            SommGravity::SetDelegateKeys { validator_address, orchestrator_address, ethereum_address, eth_signature } => {
                let msg = gravity_proto::gravity::MsgDelegateKeys {
                    validator_address: validator_address.to_string(),
                    orchestrator_address: orchestrator_address.to_string(),
                    ethereum_address: ethereum_address.to_string(),
                    eth_signature,
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode MsgDelegateKeys: {}", e)
                };
                any.type_url = "/gravity.v1.MsgDelegateKeys".to_string();
                Ok(any)
            },
            SommGravity::DelegateKeysSignMsg { validator_address, nonce } => {
                let msg = gravity_proto::gravity::DelegateKeysSignMsg {
                    validator_address: validator_address.to_string(),
                    nonce,
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode DelegateKeysSignMsg: {}", e)
                };
                any.type_url = "/gravity.v1.DelegateKeysSignMsg".to_string();
                Ok(any)
            },
            SommGravity::SubmitEthereumHeightVote { ethereum_height, signer } => {
                let msg = gravity_proto::gravity::MsgEthereumHeightVote {
                    ethereum_height,
                    signer: signer.to_string(),
                };
                let mut any = Any::default();
                if let Err(e) = prost::Message::encode(&msg, &mut any.value) {
                    bail!("failed to encode MsgEthereumHeightVote: {}", e)
                };
                any.type_url = "/gravity.v1.MsgEthereumHeightVote".to_string();
                Ok(any)
            },
        }
    }

    /// Converts the message enum representation into an [`UnsignedTx`] containing the corresponding Msg
    fn into_tx(self) -> Result<UnsignedTx> {
        // Since we include some confirmation messages in the enum to make getting an Any to insert into SubmitEthereumEventConfirmation
        // easier, we need to make sure we don't try to submit those directly in a transaction because it's guaranteed to fail.
        Ok(match self {
            SommGravity::ContractCallTxConfirmation {
                invalidation_scope: _,
                invalidation_nonce: _,
                ethereum_signer: _,
                signature: _,
            } => bail!("ContractCallTxConfirmation does not represent a transaction Msg. use into_any() to get the Any representation"),
            SommGravity::BatchTxConfirmation {
                token_contract_address: _,
                batch_nonce: _,
                ethereum_signer: _,
                signature: _,
            } => bail!("BatchTxConfirmation does not represent a transaction Msg. use into_any() to get the Any representation"),
            SommGravity::SignerSetTxConfirmation {
                signer_set_nonce: _,
                ethereum_signer: _,
                signature: _,
            } => {
                bail!("SignerSetTxConfirmation does not represent a transaction Msg. use into_any() to get the Any representation")
            }
            _ => {
                let mut tx = UnsignedTx::new();
                tx.add_msg(self.into_any()?);
                tx
            }
        })
    }
}
