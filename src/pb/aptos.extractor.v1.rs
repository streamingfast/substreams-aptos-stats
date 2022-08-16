// @generated
/// A block on Aptos holds transactions in chronological order (ordered by a transactions monotonically increasing `version` field)
/// All blocks start with a `BlockMetadataTransaction`, and are followed by zero or more transactions.
/// The next `BlockMetadataTransaction` denotes the end of the current block, and the start of the next one.
///
/// The Block `height` is a strictly monotonically increasing count of the number of blocks,
/// and there will never be a gap in the numbers. It is also a unique identifier: there will never be two blocks with
/// the same `height`.
///
/// The Genesis Transaction (version 0) is contained within the first block, which has a height of `0`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Timestamp represents the timestamp of the `BlockMetadataTransaction` (or `GenesisTransaction` for the genesis block)
    /// and every transaction in the `transactions` will have the same `timestamp` as the block.
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<super::super::util::timestamp::Timestamp>,
    /// Height represents the block number and ultimately, is the count of `BlockMetadataTransaction` that happened on the chain.
    #[prost(uint64, tag="2")]
    pub height: u64,
    /// Transactions holds all transactions that happened in the Block, which is transactions that happened starting with (and including)
    /// a `BlockMetadataTransaction`, and every other transaction up to (but excluding) the next `BlockMetadataTransaction`
    #[prost(message, repeated, tag="3")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
/// Transaction as it happened on the chain, there are 4 types of transactions:
/// - User Transaction: a user initiated transaction to interact with the chain
/// - Block Metadata Transaction: transactions generated by the chain to group together transactions forming a "block"
/// - State Checkpoint Transaction: transactions generated by the chain so when validator agreed on a particular global state
/// - Genesis Transaction: the first transaction of the chain, with all core contract and validator information baked in
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<super::super::util::timestamp::Timestamp>,
    #[prost(uint64, tag="2")]
    pub version: u64,
    #[prost(message, optional, tag="3")]
    pub info: ::core::option::Option<TransactionInfo>,
    #[prost(uint64, tag="4")]
    pub epoch: u64,
    #[prost(uint64, tag="5")]
    pub block_height: u64,
    #[prost(enumeration="transaction::TransactionType", tag="6")]
    pub r#type: i32,
    #[prost(oneof="transaction::TxnData", tags="7, 8, 9, 10")]
    pub txn_data: ::core::option::Option<transaction::TxnData>,
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransactionType {
        Genesis = 0,
        BlockMetadata = 1,
        StateCheckpoint = 2,
        User = 3,
    }
    impl TransactionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TransactionType::Genesis => "GENESIS",
                TransactionType::BlockMetadata => "BLOCK_METADATA",
                TransactionType::StateCheckpoint => "STATE_CHECKPOINT",
                TransactionType::User => "USER",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TxnData {
        #[prost(message, tag="7")]
        BlockMetadata(super::BlockMetadataTransaction),
        #[prost(message, tag="8")]
        Genesis(super::GenesisTransaction),
        #[prost(message, tag="9")]
        StateCheckpoint(super::StateCheckpointTransaction),
        #[prost(message, tag="10")]
        User(super::UserTransaction),
    }
}
/// TransactionTrimmed is a real Transaction with most of the fields removed so that
/// we can easily decode only the few fields that we have interest in in certain situations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionTrimmed {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<super::super::util::timestamp::Timestamp>,
    #[prost(uint64, tag="2")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMetadataTransaction {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub round: u64,
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(bytes="vec", tag="4")]
    pub previous_block_votes_bitvec: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub proposer: ::prost::alloc::string::String,
    #[prost(uint32, repeated, tag="6")]
    pub failed_proposer_indices: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisTransaction {
    #[prost(message, optional, tag="1")]
    pub payload: ::core::option::Option<WriteSet>,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateCheckpointTransaction {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserTransaction {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<UserTransactionRequest>,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<EventKey>,
    #[prost(uint64, tag="2")]
    pub sequence_number: u64,
    #[prost(message, optional, tag="3")]
    pub r#type: ::core::option::Option<MoveType>,
    #[prost(string, tag="4")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub state_root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub event_root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub gas_used: u64,
    #[prost(bool, tag="5")]
    pub success: bool,
    #[prost(string, tag="6")]
    pub vm_status: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub accumulator_root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="8")]
    pub changes: ::prost::alloc::vec::Vec<WriteSetChange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventKey {
    #[prost(uint64, tag="1")]
    pub creation_number: u64,
    #[prost(string, tag="2")]
    pub account_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserTransactionRequest {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub sequence_number: u64,
    #[prost(uint64, tag="3")]
    pub max_gas_amount: u64,
    #[prost(uint64, tag="4")]
    pub gas_unit_price: u64,
    #[prost(message, optional, tag="5")]
    pub expiration_timestamp_secs: ::core::option::Option<super::super::util::timestamp::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub payload: ::core::option::Option<TransactionPayload>,
    #[prost(message, optional, tag="7")]
    pub signature: ::core::option::Option<Signature>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteSet {
    #[prost(enumeration="write_set::WriteSetType", tag="1")]
    pub write_set_type: i32,
    #[prost(oneof="write_set::WriteSet", tags="2, 3")]
    pub write_set: ::core::option::Option<write_set::WriteSet>,
}
/// Nested message and enum types in `WriteSet`.
pub mod write_set {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WriteSetType {
        ScriptWriteSet = 0,
        DirectWriteSet = 1,
    }
    impl WriteSetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WriteSetType::ScriptWriteSet => "SCRIPT_WRITE_SET",
                WriteSetType::DirectWriteSet => "DIRECT_WRITE_SET",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WriteSet {
        #[prost(message, tag="2")]
        ScriptWriteSet(super::ScriptWriteSet),
        #[prost(message, tag="3")]
        DirectWriteSet(super::DirectWriteSet),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptWriteSet {
    #[prost(string, tag="1")]
    pub execute_as: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub script: ::core::option::Option<ScriptPayload>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectWriteSet {
    #[prost(message, repeated, tag="1")]
    pub write_set_change: ::prost::alloc::vec::Vec<WriteSetChange>,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteSetChange {
    #[prost(enumeration="write_set_change::Type", tag="1")]
    pub r#type: i32,
    #[prost(oneof="write_set_change::Change", tags="2, 3, 4, 5, 6, 7")]
    pub change: ::core::option::Option<write_set_change::Change>,
}
/// Nested message and enum types in `WriteSetChange`.
pub mod write_set_change {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        DeleteModule = 0,
        DeleteResource = 1,
        DeleteTableItem = 2,
        WriteModule = 3,
        WriteResource = 4,
        WriteTableItem = 5,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::DeleteModule => "DELETE_MODULE",
                Type::DeleteResource => "DELETE_RESOURCE",
                Type::DeleteTableItem => "DELETE_TABLE_ITEM",
                Type::WriteModule => "WRITE_MODULE",
                Type::WriteResource => "WRITE_RESOURCE",
                Type::WriteTableItem => "WRITE_TABLE_ITEM",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        #[prost(message, tag="2")]
        DeleteModule(super::DeleteModule),
        #[prost(message, tag="3")]
        DeleteResource(super::DeleteResource),
        #[prost(message, tag="4")]
        DeleteTableItem(super::DeleteTableItem),
        #[prost(message, tag="5")]
        WriteModule(super::WriteModule),
        #[prost(message, tag="6")]
        WriteResource(super::WriteResource),
        #[prost(message, tag="7")]
        WriteTableItem(super::WriteTableItem),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModule {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub state_key_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub module: ::core::option::Option<MoveModuleId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResource {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub state_key_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<MoveStructTag>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTableItem {
    #[prost(bytes="vec", tag="1")]
    pub state_key_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub handle: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub data: ::core::option::Option<DeleteTableData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTableData {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteModule {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub state_key_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<MoveModuleBytecode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResource {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub state_key_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<MoveResource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteTableData {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key_type: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub value_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteTableItem {
    #[prost(bytes="vec", tag="1")]
    pub state_key_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub handle: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub data: ::core::option::Option<WriteTableData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionPayload {
    #[prost(enumeration="transaction_payload::Type", tag="1")]
    pub r#type: i32,
    #[prost(oneof="transaction_payload::Payload", tags="2, 3, 4, 5")]
    pub payload: ::core::option::Option<transaction_payload::Payload>,
}
/// Nested message and enum types in `TransactionPayload`.
pub mod transaction_payload {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        ScriptFunctionPayload = 0,
        ScriptPayload = 1,
        ModuleBundlePayload = 2,
        WriteSetPayload = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::ScriptFunctionPayload => "SCRIPT_FUNCTION_PAYLOAD",
                Type::ScriptPayload => "SCRIPT_PAYLOAD",
                Type::ModuleBundlePayload => "MODULE_BUNDLE_PAYLOAD",
                Type::WriteSetPayload => "WRITE_SET_PAYLOAD",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag="2")]
        ScriptFunctionPayload(super::ScriptFunctionPayload),
        #[prost(message, tag="3")]
        ScriptPayload(super::ScriptPayload),
        #[prost(message, tag="4")]
        ModuleBundlePayload(super::ModuleBundlePayload),
        #[prost(message, tag="5")]
        WriteSetPayload(super::WriteSetPayload),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptFunctionPayload {
    #[prost(message, optional, tag="1")]
    pub function: ::core::option::Option<ScriptFunctionId>,
    #[prost(message, repeated, tag="2")]
    pub type_arguments: ::prost::alloc::vec::Vec<MoveType>,
    #[prost(string, repeated, tag="3")]
    pub arguments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveScriptBytecode {
    #[prost(bytes="vec", tag="1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub abi: ::core::option::Option<MoveFunction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptPayload {
    #[prost(message, optional, tag="1")]
    pub code: ::core::option::Option<MoveScriptBytecode>,
    #[prost(message, repeated, tag="2")]
    pub type_arguments: ::prost::alloc::vec::Vec<MoveType>,
    #[prost(string, repeated, tag="3")]
    pub arguments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBundlePayload {
    #[prost(message, repeated, tag="1")]
    pub modules: ::prost::alloc::vec::Vec<MoveModuleBytecode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveModuleBytecode {
    #[prost(bytes="vec", tag="1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub abi: ::core::option::Option<MoveModule>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveModule {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub friends: ::prost::alloc::vec::Vec<MoveModuleId>,
    #[prost(message, repeated, tag="4")]
    pub exposed_functions: ::prost::alloc::vec::Vec<MoveFunction>,
    #[prost(message, repeated, tag="5")]
    pub structs: ::prost::alloc::vec::Vec<MoveStruct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFunction {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="move_function::Visibility", tag="2")]
    pub visibility: i32,
    #[prost(bool, tag="3")]
    pub is_entry: bool,
    #[prost(message, repeated, tag="4")]
    pub generic_type_params: ::prost::alloc::vec::Vec<MoveFunctionGenericTypeParam>,
    #[prost(message, repeated, tag="5")]
    pub params: ::prost::alloc::vec::Vec<MoveType>,
    #[prost(message, repeated, tag="6")]
    pub r#return: ::prost::alloc::vec::Vec<MoveType>,
}
/// Nested message and enum types in `MoveFunction`.
pub mod move_function {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Visibility {
        Private = 0,
        Public = 1,
        Friend = 2,
    }
    impl Visibility {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Visibility::Private => "PRIVATE",
                Visibility::Public => "PUBLIC",
                Visibility::Friend => "FRIEND",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStruct {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_native: bool,
    #[prost(enumeration="MoveAbility", repeated, tag="3")]
    pub abilities: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag="4")]
    pub generic_type_params: ::prost::alloc::vec::Vec<MoveStructGenericTypeParam>,
    #[prost(message, repeated, tag="5")]
    pub fields: ::prost::alloc::vec::Vec<MoveStructField>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStructGenericTypeParam {
    #[prost(enumeration="MoveAbility", repeated, tag="1")]
    pub constraints: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, tag="2")]
    pub is_phantom: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStructField {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub r#type: ::core::option::Option<MoveType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFunctionGenericTypeParam {
    #[prost(enumeration="MoveAbility", repeated, tag="1")]
    pub constraints: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveType {
    #[prost(enumeration="MoveTypes", tag="1")]
    pub r#type: i32,
    #[prost(oneof="move_type::Content", tags="3, 4, 5, 6, 7")]
    pub content: ::core::option::Option<move_type::Content>,
}
/// Nested message and enum types in `MoveType`.
pub mod move_type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReferenceType {
        #[prost(bool, tag="1")]
        pub mutable: bool,
        #[prost(message, optional, boxed, tag="2")]
        pub to: ::core::option::Option<::prost::alloc::boxed::Box<super::MoveType>>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag="3")]
        Vector(::prost::alloc::boxed::Box<super::MoveType>),
        #[prost(message, tag="4")]
        Struct(super::MoveStructTag),
        #[prost(uint32, tag="5")]
        GenericTypeParamIndex(u32),
        #[prost(message, tag="6")]
        Reference(::prost::alloc::boxed::Box<ReferenceType>),
        #[prost(string, tag="7")]
        Unparsable(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteSetPayload {
    #[prost(message, optional, tag="1")]
    pub write_set: ::core::option::Option<WriteSet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptFunctionId {
    #[prost(message, optional, tag="1")]
    pub module: ::core::option::Option<MoveModuleId>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveResource {
    #[prost(message, optional, tag="1")]
    pub r#type: ::core::option::Option<MoveStructTag>,
    #[prost(string, tag="2")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveModuleId {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStructTag {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub module: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub generic_type_params: ::prost::alloc::vec::Vec<MoveType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(enumeration="signature::Type", tag="1")]
    pub r#type: i32,
    #[prost(oneof="signature::Signature", tags="2, 3, 4")]
    pub signature: ::core::option::Option<signature::Signature>,
}
/// Nested message and enum types in `Signature`.
pub mod signature {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Ed25519 = 0,
        MultiEd25519 = 1,
        MultiAgent = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Ed25519 => "ED25519",
                Type::MultiEd25519 => "MULTI_ED25519",
                Type::MultiAgent => "MULTI_AGENT",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Signature {
        #[prost(message, tag="2")]
        Ed25519(super::Ed25519Signature),
        #[prost(message, tag="3")]
        MultiEd25519(super::MultiEd25519Signature),
        #[prost(message, tag="4")]
        MultiAgent(super::MultiAgentSignature),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ed25519Signature {
    #[prost(bytes="vec", tag="1")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiEd25519Signature {
    #[prost(bytes="vec", repeated, tag="1")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, tag="3")]
    pub threshold: u32,
    #[prost(bytes="vec", tag="4")]
    pub bitmap: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiAgentSignature {
    #[prost(message, optional, tag="1")]
    pub sender: ::core::option::Option<AccountSignature>,
    #[prost(string, repeated, tag="2")]
    pub secondary_signer_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub secondary_signers: ::prost::alloc::vec::Vec<AccountSignature>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSignature {
    #[prost(enumeration="account_signature::Type", tag="1")]
    pub r#type: i32,
    #[prost(oneof="account_signature::Signature", tags="2, 3")]
    pub signature: ::core::option::Option<account_signature::Signature>,
}
/// Nested message and enum types in `AccountSignature`.
pub mod account_signature {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Ed25519 = 0,
        MultiEd25519 = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Ed25519 => "ED25519",
                Type::MultiEd25519 => "MULTI_ED25519",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Signature {
        #[prost(message, tag="2")]
        Ed25519(super::Ed25519Signature),
        #[prost(message, tag="3")]
        MultiEd25519(super::MultiEd25519Signature),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MoveTypes {
    Bool = 0,
    U8 = 1,
    U64 = 2,
    U128 = 3,
    Address = 4,
    Signer = 5,
    /// { items: Box<MoveType> },
    Vector = 6,
    /// (MoveStructTag),
    Struct = 7,
    /// { index: u16 },
    GenericTypeParam = 8,
    /// { mutable: bool, to: Box<MoveType> },
    Reference = 9,
    /// (String),
    Unparsable = 10,
}
impl MoveTypes {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MoveTypes::Bool => "Bool",
            MoveTypes::U8 => "U8",
            MoveTypes::U64 => "U64",
            MoveTypes::U128 => "U128",
            MoveTypes::Address => "Address",
            MoveTypes::Signer => "Signer",
            MoveTypes::Vector => "Vector",
            MoveTypes::Struct => "Struct",
            MoveTypes::GenericTypeParam => "GenericTypeParam",
            MoveTypes::Reference => "Reference",
            MoveTypes::Unparsable => "Unparsable",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MoveAbility {
    Copy = 0,
    Drop = 1,
    Store = 2,
    Key = 3,
}
impl MoveAbility {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MoveAbility::Copy => "COPY",
            MoveAbility::Drop => "DROP",
            MoveAbility::Store => "STORE",
            MoveAbility::Key => "KEY",
        }
    }
}
// @@protoc_insertion_point(module)