pub mod redis_handler;

use async_trait::async_trait;
use aurora_engine_sdk::types::near_account_to_evm_address;
use aurora_engine_transactions::{EthTransactionKind, NormalizedEthTransaction};
use aurora_engine_types::borsh;
use aurora_engine_types::parameters::engine::{CallArgs, SubmitArgs, SubmitResult};
use aurora_engine_types::types::Wei;
use inindexer::near_indexer_primitives::types::{AccountId, BlockHeight};
use inindexer::near_indexer_primitives::views::{ActionView, ExecutionStatusView, ReceiptEnumView};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_indexer_primitives::StreamerMessage;
use inindexer::{IncompleteTransaction, Indexer, TransactionReceipt};
use intear_events::events::aurora::transaction::{AuroraTransactionEvent, TransactionStatus};

#[async_trait]
pub trait AuroraEventHandler: Send + Sync {
    async fn handle_transaction(&mut self, event: AuroraTransactionEvent);

    /// Called after each block
    async fn flush_events(&mut self, block_height: BlockHeight);
}

pub struct AuroraIndexer<T: AuroraEventHandler + Send + Sync + 'static>(pub T);

#[async_trait]
impl<T: AuroraEventHandler + Send + Sync + 'static> Indexer for AuroraIndexer<T> {
    type Error = String;

    async fn on_receipt(
        &mut self,
        receipt: &TransactionReceipt,
        transaction: &IncompleteTransaction,
        block: &StreamerMessage,
    ) -> Result<(), Self::Error> {
        if receipt.receipt.receipt.receiver_id == "aurora" {
            if let ReceiptEnumView::Action { actions, .. } = &receipt.receipt.receipt.receipt {
                for action in actions {
                    if let ActionView::FunctionCall {
                        method_name, args, ..
                    } = action
                    {
                        match method_name.as_str() {
                            "submit_with_args" => {
                                if let Ok(args) = borsh::de::from_slice::<SubmitArgs>(args) {
                                    if let Ok(transaction_kind) =
                                        EthTransactionKind::try_from(args.tx_data.as_slice())
                                    {
                                        if let Ok(aurora_transaction) =
                                            NormalizedEthTransaction::try_from(transaction_kind)
                                        {
                                            if let ExecutionStatusView::SuccessValue(value) =
                                                &receipt.receipt.execution_outcome.outcome.status
                                            {
                                                if let Ok(result) =
                                                    borsh::de::from_slice::<SubmitResult>(value)
                                                {
                                                    let tx_hash =
                                                        aurora_engine_sdk::keccak(&args.tx_data);
                                                    let tx = AuroraTransactionEvent {
                                                        block_height: block.block.header.height,
                                                        block_timestamp_nanosec: block
                                                            .block
                                                            .header
                                                            .timestamp_nanosec
                                                            as u128,
                                                        transaction_id: transaction
                                                            .transaction
                                                            .transaction
                                                            .hash,
                                                        receipt_id: receipt.receipt.receipt.receipt_id,
                                                        chain_id: aurora_transaction.chain_id,
                                                        aurora_tx_hash: tx_hash.to_string(),
                                                        from: aurora_transaction.address,
                                                        to: aurora_transaction
                                                            .to,
                                                        value: aurora_transaction.value,
                                                        input: aurora_transaction.data,
                                                        status: match result.status {
                                                            aurora_engine_types::parameters::engine::TransactionStatus::Succeed(v) => TransactionStatus::Succeed(v),
                                                            aurora_engine_types::parameters::engine::TransactionStatus::Revert(v) => TransactionStatus::Revert(v),
                                                            aurora_engine_types::parameters::engine::TransactionStatus::OutOfGas => TransactionStatus::OutOfGas,
                                                            aurora_engine_types::parameters::engine::TransactionStatus::OutOfFund => TransactionStatus::OutOfFund,
                                                            aurora_engine_types::parameters::engine::TransactionStatus::OutOfOffset => TransactionStatus::OutOfOffset,
                                                            aurora_engine_types::parameters::engine::TransactionStatus::CallTooDeep => TransactionStatus::CallTooDeep,
                                                            _ => TransactionStatus::Revert("".as_bytes().to_vec()), // there are more error types added since the indexer was created. Since it's a dynamic field, and mostly no one cares about the specifics of the failure, we just make a backwards compatible default
                                                        },
                                                    };
                                                    self.0.handle_transaction(tx).await;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            "submit" => {
                                if let Ok(transaction_kind) =
                                    EthTransactionKind::try_from(args.as_slice())
                                {
                                    if let Ok(aurora_transaction) =
                                        NormalizedEthTransaction::try_from(transaction_kind)
                                    {
                                        if let ExecutionStatusView::SuccessValue(value) =
                                            &receipt.receipt.execution_outcome.outcome.status
                                        {
                                            if let Ok(result) =
                                                borsh::de::from_slice::<SubmitResult>(value)
                                            {
                                                let tx_hash = aurora_engine_sdk::keccak(args);
                                                let tx = AuroraTransactionEvent {
                                                    block_height: block.block.header.height,
                                                    block_timestamp_nanosec: block
                                                        .block
                                                        .header
                                                        .timestamp_nanosec
                                                        as u128,
                                                    transaction_id: transaction
                                                        .transaction
                                                        .transaction
                                                        .hash,
                                                    receipt_id: receipt.receipt.receipt.receipt_id,
                                                    chain_id: aurora_transaction.chain_id,
                                                    aurora_tx_hash: tx_hash.to_string(),
                                                    from: aurora_transaction.address,
                                                    to: aurora_transaction
                                                        .to,
                                                    value: aurora_transaction.value,
                                                    input: aurora_transaction.data,
                                                    status: match result.status {
                                                        aurora_engine_types::parameters::engine::TransactionStatus::Succeed(v) => TransactionStatus::Succeed(v),
                                                        aurora_engine_types::parameters::engine::TransactionStatus::Revert(v) => TransactionStatus::Revert(v),
                                                        aurora_engine_types::parameters::engine::TransactionStatus::OutOfGas => TransactionStatus::OutOfGas,
                                                        aurora_engine_types::parameters::engine::TransactionStatus::OutOfFund => TransactionStatus::OutOfFund,
                                                        aurora_engine_types::parameters::engine::TransactionStatus::OutOfOffset => TransactionStatus::OutOfOffset,
                                                        aurora_engine_types::parameters::engine::TransactionStatus::CallTooDeep => TransactionStatus::CallTooDeep,
                                                        _ => TransactionStatus::Revert("".as_bytes().to_vec()),
                                                    },
                                                };
                                                self.0.handle_transaction(tx).await;
                                            }
                                        }
                                    }
                                }
                            }
                            "call" => {
                                if let Some(call_args) = CallArgs::deserialize(args) {
                                    let _from = near_account_to_evm_address(
                                        receipt.receipt.receipt.predecessor_id.as_bytes(),
                                    );
                                    let (_to, _value, _input) = match call_args {
                                        CallArgs::V2(args) => {
                                            (args.contract, args.value.into(), args.input)
                                        }
                                        CallArgs::V1(args) => {
                                            (args.contract, Wei::zero(), args.input)
                                        }
                                    };
                                    if let ExecutionStatusView::SuccessValue(v) =
                                        &receipt.receipt.execution_outcome.outcome.status
                                    {
                                        if let Ok(_result) =
                                            borsh::de::from_slice::<SubmitResult>(v)
                                        {
                                            // TODO how to get the tx hash
                                        }
                                    }
                                }
                            }
                            "deploy_code" => {
                                // TODO same, how to get the tx hash
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn process_block_end(&mut self, block: &StreamerMessage) -> Result<(), Self::Error> {
        self.0.flush_events(block.block.header.height).await;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventContext {
    pub transaction_id: CryptoHash,
    pub receipt_id: CryptoHash,
    pub block_height: BlockHeight,
    pub block_timestamp_nanosec: u128,
    pub predecessor_id: AccountId,
    pub contract_id: AccountId,
}
