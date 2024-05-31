pub mod miscalleneous;

use crate::miscalleneous::{CallData, Evm, EvmContract, EvmOpts, EvmRt};
use evm::ExitReason;
use primitive_types::{H160, U256};
use std::collections::BTreeMap;
use std::str::FromStr;

impl<'memory> Evm<'memory> {
    pub fn new(config: EvmOpts, vicinity: &'memory evm::backend::MemoryVicinity) -> Self {
        let mut state: BTreeMap<H160, evm::backend::MemoryAccount> =
            config.evm_config.state.unwrap_or_else(|| BTreeMap::new());

        let contract_address = H160::from_str(config.evm_config.contract.address.as_str()).unwrap();

        let contract_memory = config.evm_config.contract.memory.clone();

        if let Some(mut contract_mem) = contract_memory {
            contract_mem.code = hex::decode(config.evm_config.program.clone().as_str()).unwrap();

            // Add Contract
            state.insert(contract_address.clone(), contract_mem);
        }

        // Prepare the executor.
        let backend = evm::backend::MemoryBackend::new(&vicinity, state);

        Self {
            contract: EvmContract {
                memory: backend,
                contract_address,
            },
            rt: EvmRt {
                fork_config: config.fork_config,
                precompiles: BTreeMap::new(),
            },
        }
    }

    pub fn call_raw(
        &mut self,
        caller: H160,
        input: String,
        amount: Option<U256>,
    ) -> (ExitReason, Vec<u8>) {
        let metadata =
            evm::executor::stack::StackSubstateMetadata::new(u64::MAX, &self.rt.fork_config);
        let state =
            evm::executor::stack::MemoryStackState::new(metadata, &mut self.contract.memory);
        let mut executor = evm::executor::stack::StackExecutor::new_with_precompiles(
            state,
            &self.rt.fork_config,
            &self.rt.precompiles,
        );

        let res = executor.transact_call(
            caller,
            self.contract.contract_address.clone(),
            amount.unwrap_or(U256::zero()),
            hex::decode(input.as_str()).unwrap(),
            u64::MAX,
            Vec::new(),
        );

        res
    }

    pub fn call(&mut self, data: CallData) -> (ExitReason, Vec<u8>) {
        self.call_raw(data.caller, data.input, data.amount)
    }
}

pub fn get_default_vicinity() -> evm::backend::MemoryVicinity {
    evm::backend::MemoryVicinity {
        gas_price: U256::zero(),
        origin: H160::default(),
        block_hashes: Vec::new(),
        block_number: Default::default(),
        block_coinbase: Default::default(),
        block_timestamp: Default::default(),
        block_difficulty: Default::default(),
        block_gas_limit: Default::default(),
        chain_id: U256::one(),
        block_base_fee_per_gas: U256::zero(),
        block_randomness: None,
    }
}
