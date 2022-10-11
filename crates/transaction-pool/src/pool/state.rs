bitflags::bitflags! {
    /// Marker to represents the current state of a transaction in the pool and from which the corresponding sub-pool is derived, depending on what bits are set.
    ///
    /// This mirrors [erigon's ephemeral state field](https://github.com/ledgerwatch/erigon/wiki/Transaction-Pool-Design#ordering-function).
    #[derive(Default)]
    pub(crate) struct TxState: u8 {
        /// Set to `1` if the `feeCap` of the transaction meets the chain's minimum `feeCap` requirement.
        ///
        /// This is different from `ENOUGH_FEE_CAP_BLOCK` which tracks on a per-block basis.
        const ENOUGH_FEE_CAP_PROTOCOL = 0b100000;
        /// Set to `1` of the transaction is either the next transaction of the sender (on chain nonce == tx.nonce) or all prior transactions are also present in the pool.
        const NO_NONCE_GAPS = 0b010000;
        /// Bit derived from the sender's balance.
        ///
        /// Set to `1` if the sender's balance can cover the maximum cost for this transaction (`feeCap * gasLimit + value`).
        /// This includes cumulative costs of prior transactions, which ensures that the sender has enough funds for all max cost of prior transactions.
        const ENOUGH_BALANCE = 0b001000;
        /// Bit set to true if the transaction has a lower gas limit than the block's gas limit
        const NOT_TOO_MUCH_GAS = 0b000100;
        /// Covers the Dynamic fee requirement.
        ///
        /// Set to 1 if `feeCap` of the transaction meets the requirement of the pending block.
        const ENOUGH_FEE_CAP_BLOCK = 0b000010;
        const IS_LOCAL = 0b000001;

        const BASE_FEE_POOL_BITS = Self::ENOUGH_FEE_CAP_PROTOCOL.bits | Self::NO_NONCE_GAPS.bits | Self::ENOUGH_BALANCE.bits | Self::NOT_TOO_MUCH_GAS.bits;

        const QUEUED_POOL_BITS  = Self::ENOUGH_FEE_CAP_PROTOCOL.bits;
    }
}

/// Identifier for the used Sub-pool
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub(crate) enum SubPool {
    Queued = 0,
    Pending,
    BaseFee,
}

// === impl PoolDestination ===

impl SubPool {
    /// Whether this transaction is to be moved to the pending sub-pool.
    pub(crate) fn is_pending(&self) -> bool {
        matches!(self, SubPool::Pending)
    }

    /// Returns whether this is a promotion depending on the current sub-pool location.
    pub(crate) fn is_promoted(&self, other: SubPool) -> bool {
        self > &other
    }
}

impl From<TxState> for SubPool {
    fn from(value: TxState) -> Self {
        if value > TxState::BASE_FEE_POOL_BITS {
            return SubPool::Pending
        }
        if value < TxState::QUEUED_POOL_BITS {
            return SubPool::Queued
        }
        SubPool::BaseFee
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tx_state() {
        let mut state = TxState::default();
        state |= TxState::NO_NONCE_GAPS;
        assert!(state.intersects(TxState::NO_NONCE_GAPS))
    }
}