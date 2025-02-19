use crate::EngineApiSender;
use reth_interfaces::consensus::ForkchoiceState;
use reth_primitives::{BlockHash, BlockNumber, H64};
use reth_rpc_types::engine::{
    ExecutionPayload, ExecutionPayloadBodies, ForkchoiceUpdated, PayloadAttributes, PayloadStatus,
    TransitionConfiguration,
};

/// Message type for communicating with [`EngineApi`][crate::EngineApi].
#[derive(Debug)]
pub enum EngineApiMessage {
    /// New payload message
    NewPayload(EngineApiMessageVersion, ExecutionPayload, EngineApiSender<PayloadStatus>),
    /// Get payload message
    GetPayload(H64, EngineApiSender<ExecutionPayload>),
    /// Get payload bodies by range message
    GetPayloadBodiesByRange(BlockNumber, u64, EngineApiSender<ExecutionPayloadBodies>),
    /// Get payload bodies by hash message
    GetPayloadBodiesByHash(Vec<BlockHash>, EngineApiSender<ExecutionPayloadBodies>),
    /// Forkchoice updated message
    ForkchoiceUpdated(
        EngineApiMessageVersion,
        ForkchoiceState,
        Option<PayloadAttributes>,
        EngineApiSender<ForkchoiceUpdated>,
    ),
    /// Exchange transition configuration message
    ExchangeTransitionConfiguration(
        TransitionConfiguration,
        EngineApiSender<TransitionConfiguration>,
    ),
}

/// The version of Engine API message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineApiMessageVersion {
    /// Version 1
    V1,
    /// Version 2
    V2,
}
