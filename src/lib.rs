mod pb;
use pb::aptos;
use substreams::store;

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
