use serde_with::DefaultOnNull;
use std::collections::HashMap;
use alloy_primitives::{Address, BlockHash, Bytes, TxHash, TxIndex, TxNonce, B256, U256, U64};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SentioTracerConfig {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub functions: HashMap<Address, Vec<FunctionInfo>>,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub calls: HashMap<Address, Vec<usize>>,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub debug: bool,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub with_internal_calls: bool,
}

#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct FunctionInfo {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub signature_hash: Bytes,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub pc: usize,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub input_size: usize,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub input_memory: bool,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub output_size: usize,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub output_memory: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentioTrace {
    #[serde(rename = "type")]
    pub typ: String,

    pub pc: usize,
    pub start_index: usize,
    pub end_index: usize,

    // gas remaining before the op
    pub gas: U256,
    // gas for the entire call
    pub gas_used: U256,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    // used by call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert_reason: Option<String>,

    // used by jump
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_stack: Option<Vec<U256>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_memory: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_stack: Option<Vec<U256>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_memory: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_pc: Option<usize>,

    // used by log
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<B256>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub traces: Vec<Box<SentioTrace>>,

    // only in root trace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<SentioReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracer_config: Option<SentioTracerConfig>
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentioReceipt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<TxNonce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<TxHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<U64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<BlockHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_index: Option<TxIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
}