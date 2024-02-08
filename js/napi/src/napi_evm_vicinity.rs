use napi_derive::napi;
use primitive_types::{H160, H256, U256};
use std::str::FromStr;

pub type EvmVicinity = twenty48k_evm::miscalleneous::backend::MemoryVicinity;

#[napi(object)]
pub struct Vicinity {
    /// Gas price.
    pub gas_price: String, //U256
    /// Origin.
    pub origin: String, //H160
    /// Chain ID.
    pub chain_id: String, //H160
    /// Environmental block hashes.
    pub block_hashes: Vec<String>, //H256
    /// Environmental block number.
    pub block_number: String, //U256
    /// Environmental coinbase.
    pub block_coinbase: String, //H160
    /// Environmental block timestamp.
    pub block_timestamp: String, //U256
    /// Environmental block difficulty.
    pub block_difficulty: String, //U256
    /// Environmental block gas limit.
    pub block_gas_limit: String, //U256
    /// Environmental base fee per gas.
    pub block_base_fee_per_gas: String, //U256
    /// Environmental randomness.
    ///
    /// In Ethereum, this is the randomness beacon provided by the beacon
    /// chain and is only enabled post Merge.
    pub block_randomness: Option<String>, //U256
}

impl From<Vicinity> for EvmVicinity {
    fn from(value: Vicinity) -> Self {
        EvmVicinity {
            gas_price: U256::from_str(value.gas_price.as_str()).unwrap(),
            origin: H160::from_str(value.origin.as_str()).unwrap(),
            chain_id: U256::from_str(value.chain_id.as_str()).unwrap(),
            block_hashes: value
                .block_hashes
                .iter()
                .map(|e| H256::from_str(e.clone().as_str()).unwrap())
                .collect(),
            block_number: U256::from_str(value.block_number.as_str()).unwrap(),
            block_coinbase: H160::from_str(value.block_coinbase.as_str()).unwrap(),
            block_timestamp: U256::from_str(value.block_timestamp.as_str()).unwrap(),
            block_difficulty: U256::from_str(value.block_difficulty.as_str()).unwrap(),
            block_gas_limit: U256::from_str(value.block_gas_limit.as_str()).unwrap(),
            block_base_fee_per_gas: U256::from_str(value.block_base_fee_per_gas.as_str()).unwrap(),
            block_randomness: value
                .block_randomness
                .map(|v| H256::from_str(v.as_str()).unwrap()),
        }
    }
}
