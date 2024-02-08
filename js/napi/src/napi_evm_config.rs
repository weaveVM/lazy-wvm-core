use crate::{EvmAccount, MemoryAccount};
use napi_derive::napi;
use primitive_types::H160;
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

#[napi(object)]
#[derive(Clone)]
pub struct EvmConfig {
    pub program: String,
    pub contract: EvmAccount,
    pub state: Option<HashMap<String, MemoryAccount>>,
}

pub type EvmConfigMisc = twenty48k_evm::miscalleneous::EvmConfig;
pub type EvmState = HashMap<String, MemoryAccount>;

impl EvmConfig {
    pub fn new(
        program: String,
        contract: EvmAccount,
        state: Option<EvmState>,
    ) -> Self {
        Self {
            program,
            contract,
            state,
        }
    }

    pub fn state_as_evm(
        &self,
    ) -> Option<BTreeMap<H160, twenty48k_evm::miscalleneous::backend::MemoryAccount>> {
        self.state.as_ref().map(|data| {
            let mut map: BTreeMap<H160, twenty48k_evm::miscalleneous::backend::MemoryAccount> =
                BTreeMap::new();
            for (key, val) in data {
                map.insert(H160::from_str(key.clone().as_str()).unwrap(), val.as_evm())
                    .unwrap();
            }
            map
        })
    }

    pub fn as_evm(&self) -> EvmConfigMisc {
        EvmConfigMisc {
            program: self.program.clone(),
            contract: self.contract.clone().into(),
            state: self.state_as_evm(),
        }
    }
}
