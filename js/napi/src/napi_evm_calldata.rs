use napi_derive::napi;
use primitive_types::{H160, U256};
use std::str::FromStr;
use twenty48k_evm::miscalleneous::CallData;

#[napi(object)]
#[derive(Clone)]
pub struct EvmCallData {
    pub tx_id: String,
    pub caller: String, //h160
    pub input: String,
    pub amount: Option<String>, //u256
}

impl From<EvmCallData> for CallData {
    fn from(value: EvmCallData) -> Self {
        CallData {
            tx_id: value.tx_id,
            caller: H160::from_str(value.caller.as_str()).unwrap(),
            input: value.input,
            amount: value.amount.map(|v| U256::from_str(v.as_str()).unwrap()),
        }
    }
}
