pub mod napi_evm_account;
pub mod napi_evm_calldata;
pub mod napi_evm_config;
pub mod napi_evm_opts;
pub mod napi_evm_vicinity;
pub mod napi_fork_config;
pub mod napi_memory_account;

use crate::napi_evm_account::EvmAccount;
use crate::napi_evm_calldata::EvmCallData;
use crate::napi_evm_config::{EvmConfig, EvmState};
use crate::napi_evm_opts::EvmOpts;
use crate::napi_evm_vicinity::Vicinity;
use crate::napi_fork_config::ForkConfig;
use crate::napi_memory_account::{MemoryAccount, to_napi_memory_account};
use napi::bindgen_prelude::*;
use napi::sys::{napi_env, napi_value};
use napi_derive::napi;
use std::collections::HashMap;
use std::ops::Deref;
use twenty48k_evm::{get_default_vicinity, miscalleneous};

#[cfg(target_os = "macos")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[napi(object)]
pub struct EvmContext {
    pub evm_opts: EvmOpts,
    pub vicinity: Option<Vicinity>,
    pub calls: Vec<EvmCallData>,
}

#[napi(object)]
pub struct EvmResponse {
    pub validity: HashMap<String, serde_json::Value>,
    pub res: Option<String>,
    pub state: HashMap<String, MemoryAccount>
}

#[napi]
async fn run(context: EvmContext) -> EvmResponse {
    let opts = context.evm_opts.as_evm();
    let vicinity: miscalleneous::backend::MemoryVicinity = context
        .vicinity
        .map(|v| v.into())
        .unwrap_or_else(|| get_default_vicinity());
    let mut rt = miscalleneous::Evm::new(opts, &vicinity);
    let calls: Vec<miscalleneous::CallData> = context
        .calls
        .iter()
        .map(|v| {
            let val: miscalleneous::CallData = (*v).clone().into();
            val
        })
        .collect();

    let mut map: HashMap<String, serde_json::Value> = HashMap::new();
    let mut result: Option<String> = None;

    for call_data in calls {
        let tx_id = call_data.tx_id.clone();
        let (exit_reason, res) = rt.call(call_data);
        let exit_reason_value = serde_json::to_value(exit_reason).unwrap();

        map.insert(tx_id, exit_reason_value);
        let hex_result = hex::encode(res);
        result = Some(hex_result);
    }

    let mut state: EvmState = HashMap::new();


    for (name, val) in rt.contract.memory.state() {
        let key = format!("{:?}", name);
        let val = to_napi_memory_account(val);
        state.insert(key, val);
    }

    EvmResponse {
        validity: map,
        res: result,
        state
    }
}
