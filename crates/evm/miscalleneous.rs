use evm::executor::stack::PrecompileFn;
use primitive_types::{H160, U256};
use std::collections::BTreeMap;

pub use evm::*;

pub type ForkConfig = evm::Config;

pub struct EvmAccount {
    pub address: String,
    pub memory: Option<evm::backend::MemoryAccount>,
}

pub struct EvmConfig {
    pub program: String,
    pub contract: EvmAccount,
    pub state: Option<BTreeMap<H160, evm::backend::MemoryAccount>>,
}

pub struct EvmContract<'a> {
    pub memory: evm::backend::MemoryBackend<'a>,
    pub contract_address: H160,
}

pub struct EvmCall {
    pub caller_address: H160,
    pub input: String,
    pub amount: Option<U256>,
}

pub struct EvmRt {
    pub fork_config: ForkConfig,
    pub precompiles: BTreeMap<H160, PrecompileFn>,
}

pub struct Evm<'a> {
    pub contract: EvmContract<'a>,
    pub rt: EvmRt,
}

pub struct EvmOpts {
    pub evm_config: EvmConfig,
    pub fork_config: ForkConfig,
}

pub struct CallData {
    pub tx_id: String,
    pub caller: H160,
    pub input: String,
    pub amount: Option<U256>,
}
