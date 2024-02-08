use napi_derive::napi;
use primitive_types::{H256, U256};
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

#[napi(object)]
#[derive(Clone)]
pub struct MemoryAccount {
    pub nonce: String,
    pub balance: String,
    pub storage: HashMap<String, String>,
    pub code: Vec<u8>,
}

type EvmAccount = twenty48k_evm::miscalleneous::backend::MemoryAccount;

impl MemoryAccount {
    pub fn new(
        nonce: String,
        balance: String,
        storage: HashMap<String, String>,
        code: Vec<u8>,
    ) -> Self {
        Self {
            nonce,
            balance,
            storage,
            code,
        }
    }

    fn get_storage(&self) -> BTreeMap<H256, H256> {
        let mut bmap: BTreeMap<H256, H256> = BTreeMap::new();

        for (key, val) in &self.storage {
            bmap.insert(
                primitive_types::H256::from_str(key.as_str()).unwrap(),
                primitive_types::H256::from_str(val.as_str()).unwrap(),
            );
        }

        bmap
    }

    pub fn as_evm(&self) -> EvmAccount {
        return EvmAccount {
            nonce: primitive_types::U256::from_str(self.nonce.as_str()).unwrap(),
            balance: primitive_types::U256::from_str(self.balance.as_str()).unwrap(),
            storage: self.get_storage(),
            code: self.code.clone(),
        };
    }
}

impl From<MemoryAccount> for EvmAccount {
    fn from(w: MemoryAccount) -> EvmAccount {
        MemoryAccount::new(w.nonce, w.balance, w.storage, w.code).as_evm()
    }
}

pub fn get_napi_memory_account_storage(storage: &BTreeMap<H256, H256>) -> HashMap<String, String> {
    let mut storage_map: HashMap<String, String> = HashMap::new();

    for (key, val) in storage {
        storage_map.insert(
            format!("{:?}", key),
            format!("{:?}", val),
        );
    }

    storage_map
}

pub fn to_napi_memory_account(memory_account: &EvmAccount) -> MemoryAccount {
    MemoryAccount {
        nonce: memory_account.nonce.to_string(),
        balance: memory_account.balance.to_string(),
        storage: get_napi_memory_account_storage(&memory_account.storage),
        code: memory_account.code.clone()
    }
}