use reth_node_api::{evm::EvmConfig, ConfigureEvmEnv};
use reth_primitives::{
    revm::{config::revm_spec, env::fill_op_tx_env},
    revm_primitives::{AnalysisKind, CfgEnvWithHandlerCfg, TxEnv},
    Address, Bytes, ChainSpec, Head, Header, Transaction, U256,
};

/// Optimism-related EVM configuration.
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct OptimismEvmConfig;

impl ConfigureEvmEnv for OptimismEvmConfig {
    type TxMeta = Bytes;

    fn fill_tx_env<T>(tx_env: &mut TxEnv, transaction: T, sender: Address, meta: Bytes)
    where
        T: AsRef<Transaction>,
    {
        fill_op_tx_env(tx_env, transaction, sender, meta);
    }

    fn fill_cfg_env(
        cfg_env: &mut CfgEnvWithHandlerCfg,
        chain_spec: &ChainSpec,
        header: &Header,
        total_difficulty: U256,
    ) {
        let spec_id = revm_spec(
            chain_spec,
            Head {
                number: header.number,
                timestamp: header.timestamp,
                difficulty: header.difficulty,
                total_difficulty,
                hash: Default::default(),
            },
        );

        cfg_env.chain_id = chain_spec.chain().id();
        cfg_env.perf_analyse_created_bytecodes = AnalysisKind::Analyse;

        cfg_env.handler_cfg.spec_id = spec_id;
        cfg_env.handler_cfg.is_optimism = chain_spec.is_optimism();
    }
}

// TODO
impl EvmConfig for OptimismEvmConfig {}

#[cfg(test)]
mod tests {
    use super::*;
    use reth_primitives::revm_primitives::{BlockEnv, CfgEnv, SpecId};

    #[test]
    #[ignore]
    fn test_fill_cfg_and_block_env() {
        let mut cfg_env = CfgEnvWithHandlerCfg::new(CfgEnv::default(), SpecId::LATEST);
        let mut block_env = BlockEnv::default();
        let header = Header::default();
        let chain_spec = ChainSpec::default();
        let total_difficulty = U256::ZERO;

        OptimismEvmConfig::fill_cfg_and_block_env(
            &mut cfg_env,
            &mut block_env,
            &chain_spec,
            &header,
            total_difficulty,
        );

        assert_eq!(cfg_env.chain_id, chain_spec.chain().id());
    }
}