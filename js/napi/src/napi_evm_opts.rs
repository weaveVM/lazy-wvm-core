use crate::{EvmConfig, ForkConfig};
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct EvmOpts {
    pub evm_config: EvmConfig,
    pub fork_config: Option<ForkConfig>,
}

impl EvmOpts {
    pub fn as_evm(&self) -> twenty48k_evm::miscalleneous::EvmOpts {
        twenty48k_evm::miscalleneous::EvmOpts {
            evm_config: self.evm_config.clone().as_evm(),
            fork_config: self
                .fork_config
                .clone()
                .map(|e| e.into())
                .unwrap_or_else(|| twenty48k_evm::miscalleneous::ForkConfig::istanbul())
        }
    }
}
