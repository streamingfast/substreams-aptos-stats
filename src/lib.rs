mod pb;
use pb::{
    aptos::{self, BlockMetadataTransaction},
    stats::BlockMetadataWrapper,
};
use substreams::store;

use crate::pb::{aptos::transaction::TxnData, stats::BlockMetadata};

#[substreams::handlers::map]
fn block_metadata(
    transaction: aptos::Transaction,
) -> Result<BlockMetadataWrapper, substreams::errors::Error> {
    Ok(match transaction.txn_data {
        Some(TxnData::BlockMetadata(data)) => BlockMetadataWrapper {
            metadata: Some(BlockMetadata {
                id: data.id,
                round: data.round,
                timestamp: transaction.timestamp,
            }),
        },
        _ => BlockMetadataWrapper { metadata: None },
    })
}

#[substreams::handlers::store]
fn store_count(transaction: aptos::Transaction, store: store::StoreAddInt64) {
    store.add(transaction.version, generate_trx_key(), 1);
    store.add(
        transaction.version,
        generate_trx_type_key(transaction.r#type()),
        1,
    );
}

fn generate_trx_key() -> String {
    return format!("total");
}

fn generate_trx_type_key(trx_type: aptos::transaction::TransactionType) -> String {
    use aptos::transaction::TransactionType;

    match trx_type {
        TransactionType::Genesis => "genesis",
        TransactionType::BlockMetadata => "block_metadata",
        TransactionType::StateCheckpoint => "state_checkpoint",
        TransactionType::User => "user",
    }
    .to_string()
}
