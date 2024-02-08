use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct ForkConfig {
    /// Gas paid for extcode.
    pub gas_ext_code: String,
    /// Gas paid for extcodehash.
    pub gas_ext_code_hash: String,
    /// Gas paid for sstore set.
    pub gas_sstore_set: String,
    /// Gas paid for sstore reset.
    pub gas_sstore_reset: String,
    /// Gas paid for sstore refund.
    pub refund_sstore_clears: i64,
    /// EIP-3529
    pub max_refund_quotient: String,
    /// Gas paid for BALANCE opcode.
    pub gas_balance: String,
    /// Gas paid for SLOAD opcode.
    pub gas_sload: String,
    /// Gas paid for cold SLOAD opcode.
    pub gas_sload_cold: String,
    /// Gas paid for SUICIDE opcode.
    pub gas_suicide: String,
    /// Gas paid for SUICIDE opcode when it hits a new account.
    pub gas_suicide_new_account: String,
    /// Gas paid for CALL opcode.
    pub gas_call: String,
    /// Gas paid for EXP opcode for every byte.
    pub gas_expbyte: String,
    /// Gas paid for a contract creation transaction.
    pub gas_transaction_create: String,
    /// Gas paid for a message call transaction.
    pub gas_transaction_call: String,
    /// Gas paid for zero data in a transaction.
    pub gas_transaction_zero_data: String,
    /// Gas paid for non-zero data in a transaction.
    pub gas_transaction_non_zero_data: String,
    /// Gas paid per address in transaction access list (see EIP-2930).
    pub gas_access_list_address: String,
    /// Gas paid per storage key in transaction access list (see EIP-2930).
    pub gas_access_list_storage_key: String,
    /// Gas paid for accessing cold account.
    pub gas_account_access_cold: String,
    /// Gas paid for accessing ready storage.
    pub gas_storage_read_warm: String,
    /// EIP-1283.
    pub sstore_gas_metering: bool,
    /// EIP-1706.
    pub sstore_revert_under_stipend: bool,
    /// EIP-2929
    pub increase_state_access_gas: bool,
    /// EIP-3529
    pub decrease_clears_refund: bool,
    /// EIP-3541
    pub disallow_executable_format: bool,
    /// EIP-3651
    pub warm_coinbase_address: bool,
    /// Whether to throw out of gas error when
    /// CALL/CALLCODE/DELEGATECALL requires more than maximum amount
    /// of gas.
    pub err_on_call_with_more_gas: bool,
    /// Take l64 for callcreate after gas.
    pub call_l64_after_gas: bool,
    /// Whether empty account is considered exists.
    pub empty_considered_exists: bool,
    /// Whether create transactions and create opcode increases nonce by one.
    pub create_increase_nonce: bool,
    /// Stack limit.
    pub stack_limit: String,
    /// Memory limit.
    pub memory_limit: String,
    /// Call limit.
    pub call_stack_limit: String,
    /// Create contract limit.
    pub create_contract_limit: Option<String>,
    /// EIP-3860, maximum size limit of init_code.
    pub max_initcode_size: Option<String>,
    /// Call stipend.
    pub call_stipend: String,
    /// Has delegate call.
    pub has_delegate_call: bool,
    /// Has create2.
    pub has_create2: bool,
    /// Has revert.
    pub has_revert: bool,
    /// Has return data.
    pub has_return_data: bool,
    /// Has bitwise shifting.
    pub has_bitwise_shifting: bool,
    /// Has chain ID.
    pub has_chain_id: bool,
    /// Has self balance.
    pub has_self_balance: bool,
    /// Has ext code hash.
    pub has_ext_code_hash: bool,
    /// Has ext block fee. See [EIP-3198](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-3198.md)
    pub has_base_fee: bool,
    /// Has PUSH0 opcode. See [EIP-3855](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-3855.md)
    pub has_push0: bool,
    /// Whether the gasometer is running in estimate mode.
    pub estimate: bool,
}

impl From<ForkConfig> for twenty48k_evm::miscalleneous::ForkConfig {
    fn from(value: ForkConfig) -> Self {
        twenty48k_evm::miscalleneous::ForkConfig {
            gas_ext_code: value.gas_ext_code.parse::<u64>().unwrap(),
            gas_ext_code_hash: value.gas_ext_code_hash.parse::<u64>().unwrap(),
            gas_sstore_set: value.gas_sstore_set.parse::<u64>().unwrap(),
            gas_sstore_reset: value.gas_sstore_reset.parse::<u64>().unwrap(),
            refund_sstore_clears: value.refund_sstore_clears,
            max_refund_quotient: value.max_refund_quotient.parse::<u64>().unwrap(),
            gas_balance: value.gas_balance.parse::<u64>().unwrap(),
            gas_sload: value.gas_sload.parse::<u64>().unwrap(),
            gas_sload_cold: value.gas_sload_cold.parse::<u64>().unwrap(),
            gas_suicide: value.gas_suicide.parse::<u64>().unwrap(),
            gas_suicide_new_account: value.gas_suicide_new_account.parse::<u64>().unwrap(),
            gas_call: value.gas_call.parse::<u64>().unwrap(),
            gas_expbyte: value.gas_expbyte.parse::<u64>().unwrap(),
            gas_transaction_create: value.gas_transaction_create.parse::<u64>().unwrap(),
            gas_transaction_call: value.gas_transaction_call.parse::<u64>().unwrap(),
            gas_transaction_zero_data: value.gas_transaction_zero_data.parse::<u64>().unwrap(),
            gas_transaction_non_zero_data: value
                .gas_transaction_non_zero_data
                .parse::<u64>()
                .unwrap(),
            gas_access_list_address: value.gas_access_list_address.parse::<u64>().unwrap(),
            gas_access_list_storage_key: value.gas_access_list_storage_key.parse::<u64>().unwrap(),
            gas_account_access_cold: value.gas_account_access_cold.parse::<u64>().unwrap(),
            gas_storage_read_warm: value.gas_storage_read_warm.parse::<u64>().unwrap(),
            sstore_gas_metering: value.sstore_gas_metering,
            sstore_revert_under_stipend: value.sstore_revert_under_stipend,
            increase_state_access_gas: value.increase_state_access_gas,
            decrease_clears_refund: value.decrease_clears_refund,
            disallow_executable_format: value.disallow_executable_format,
            warm_coinbase_address: value.warm_coinbase_address,
            err_on_call_with_more_gas: value.err_on_call_with_more_gas,
            call_l64_after_gas: value.call_l64_after_gas,
            empty_considered_exists: value.empty_considered_exists,
            create_increase_nonce: value.create_increase_nonce,
            stack_limit: value.stack_limit.parse::<usize>().unwrap(),
            memory_limit: value.memory_limit.parse::<usize>().unwrap(),
            call_stack_limit: value.call_stack_limit.parse::<usize>().unwrap(),
            create_contract_limit: value
                .create_contract_limit
                .map(|v| v.parse::<usize>().unwrap()),
            max_initcode_size: value.max_initcode_size.map(|v| v.parse::<usize>().unwrap()),
            call_stipend: value.call_stipend.parse::<u64>().unwrap(),
            has_delegate_call: value.has_delegate_call,
            has_create2: value.has_create2,
            has_revert: value.has_revert,
            has_return_data: value.has_return_data,
            has_bitwise_shifting: value.has_bitwise_shifting,
            has_chain_id: value.has_chain_id,
            has_self_balance: value.has_self_balance,
            has_ext_code_hash: value.has_ext_code_hash,
            has_base_fee: value.has_base_fee,
            has_push0: value.has_push0,
            estimate: value.estimate,
        }
    }
}
