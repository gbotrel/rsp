use reth_chainspec::{Chain, ChainSpec, ChainSpecBuilder, OP_MAINNET};

/// Returns the [ChainSpec] for Ethereum mainnet.
pub fn mainnet() -> eyre::Result<ChainSpec> {
    Ok(ChainSpecBuilder::mainnet().shanghai_activated().build())
}

/// Returns the [ChainSpec] for OP Mainnet.
pub fn op_mainnet() -> eyre::Result<ChainSpec> {
    Ok((*OP_MAINNET.clone()).clone())
}

/// Returns the [ChainSpec] for Linea Mainnet.
pub fn linea_mainnet() -> eyre::Result<ChainSpec> {
    let genesis = include_str!("../../../genesis-linea-mainnet.json");
    let genesis: reth_primitives::Genesis = serde_json::from_str(genesis).unwrap();

    let chain_spec = ChainSpecBuilder::default()
        .chain(Chain::linea())
        .genesis(genesis)
        .homestead_activated()
        .tangerine_whistle_activated()
        .spurious_dragon_activated()
        .byzantium_activated()
        .constantinople_activated()
        .petersburg_activated()
        .istanbul_activated()
        .berlin_activated()
        .london_activated()
        .build();

    Ok(chain_spec)
}
