mod pb;
use pb::aptos::extractor::v1 as aptos;
use substreams::store;

#[substreams::handlers::store]
fn store_count(block: aptos::Block, store: store::StoreAddInt64) {
    store.add(
        block.height,
        generate_total_trx_key(),
        block.transactions.len() as i64,
    );

    block.transactions.iter().for_each(|transaction| {
        store.add(
            transaction.version,
            generate_total_trx_type_key(transaction.r#type()),
            1,
        );
    });
}

fn generate_total_trx_key() -> String {
    return format!("total");
}

fn generate_total_trx_type_key(trx_type: aptos::transaction::TransactionType) -> String {
    use aptos::transaction::TransactionType;

    match trx_type {
        TransactionType::Genesis => "genesis",
        TransactionType::BlockMetadata => "block_metadata",
        TransactionType::StateCheckpoint => "state_checkpoint",
        TransactionType::User => "user",
    }
    .to_string()
}
