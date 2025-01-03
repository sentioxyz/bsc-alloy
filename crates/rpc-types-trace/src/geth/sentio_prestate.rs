use serde_with::DefaultOnNull;
use std::collections::BTreeMap;
use alloy_primitives::{Address, Bytes, B256, B512, U256};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SentioPrestateTracerConfig {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub diff_mode: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentioPrestateResult {
    pub pre: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<State>
}

pub type State = BTreeMap<Address, AccountState>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountState {
    /// The optional balance of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<U256>,
    /// The optional code of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Bytes>,
    /// The optional nonce of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
    /// The storage of the account.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub storage: BTreeMap<B256, B256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_address: Option<Address>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub code_address_by_slot: BTreeMap<B256, Address>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub mapping_keys: BTreeMap<B512, B256>
}

impl AccountState {
    pub fn from_account_info(nonce: u64, balance: U256, code: Option<Bytes>) -> Self {
        Self {
            balance: Some(balance),
            code: code.filter(|code| !code.is_empty()),
            nonce: (nonce != 0).then_some(nonce),
            storage: Default::default(),
            code_address: None,
            code_address_by_slot: BTreeMap::new(),
            mapping_keys: BTreeMap::new(),
        }
    }

    pub fn remove_matching_account_info(&mut self, other: &Self) {
        if self.balance == other.balance {
            self.balance = None;
        }
        if self.nonce == other.nonce {
            self.nonce = None;
        }
        if self.code == other.code {
            self.code = None;
        }
    }
}