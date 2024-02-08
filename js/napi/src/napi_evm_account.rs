use crate::MemoryAccount;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct EvmAccount {
    pub address: String,
    pub memory: Option<MemoryAccount>,
}

impl From<EvmAccount> for twenty48k_evm::miscalleneous::EvmAccount {
    fn from(w: EvmAccount) -> twenty48k_evm::miscalleneous::EvmAccount {
        twenty48k_evm::miscalleneous::EvmAccount {
            address: w.address.clone(),
            memory: w.memory.map(|item| item.as_evm()),
        }
    }
}
