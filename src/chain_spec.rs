use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::MultiSigner;
use node_template_runtime::{
    BalancesConfig, GenesisConfig, SudoConfig, SystemConfig, WASM_BINARY, AccountId, Signature,
};
use sc_chain_spec::ChainSpecExtension;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sc_telemetry::TelemetryEndpoints;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

#[derive(Debug, Clone, ChainSpecExtension)]
pub struct Extensions {
    #[serde(skip)]
    pub telemetry: Option<TelemetryEndpoints>,
}

fn get_from_seed<T: Public>(seed: &str) -> T::Pair {
    T::Pair::from_string(&format!("//{}", seed), None).expect("static values are valid; qed")
        .public()
}

fn get_account_id_from_seed<T: Public>(seed: &str) -> AccountId
where
    AccountId: From<<T::Pair as Pair>::Public>,
{
    AccountId::from(get_from_seed::<T>(seed))
}

/// Decode the sudo account from the SS58 address string
fn get_account_id_from_ss58(address: &str) -> AccountId {
    use sp_core::crypto::Ss58Codec;
    AccountId::from_string(address).expect("Invalid SS58 address")
}

fn shinde_genesis(wasm_binary: &[u8]) -> GenesisConfig {
    // Initial supply: 589,552,695,333,683 SHINDE coins
    let initial_supply: u128 = 589_552_695_333_683 * 10u128.pow(12); // Assuming 12 decimal places
    
    // Your custom sudo account address
    let sudo_address = "5E2dY5eu1fz1AyyBnG9NSwpWtAxRhve8Q88aDrfu6DwHQQii";
    let root_key = get_account_id_from_ss58(sudo_address);

    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            balances: vec![
                (root_key.clone(), initial_supply),
            ],
        },
        sudo: SudoConfig {
            key: Some(root_key.clone()),
        },
        ..Default::default()
    }
}

pub fn development_chain_spec() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        Some("shinde is born - 19 May -2025".to_string()),
    )
    .with_name("Shinde Development")
    .with_id("shinde-dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config(move || shinde_genesis(wasm_binary))
    .build())
}

pub fn local_chain_spec() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Local wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        Some("shinde is born - 19 May -2025".to_string()),
    )
    .with_name("Shinde Local Testnet")
    .with_id("shinde-local")
    .with_chain_type(ChainType::Local)
    .with_genesis_config(move || shinde_genesis(wasm_binary))
    .build())
}
