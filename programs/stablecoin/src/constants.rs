use anchor_lang::prelude::*;

#[constant]
pub const SEEDS_CONFIG_ACCOUNT: &[u8] = b"config";

#[constant]
pub const SEEDS_MINT_ACCOUNT: &[u8] = b"mint";

#[constant]
pub const SEEDS_COLLATERAL_ACCOUNT: &[u8] = b"collateeral";

#[constant]
pub const SEEDS_SOL_ACCOUNT: &[u8] = b"sol";

#[constant]
pub const MINT_DECIMALS: u8 = 9;

#[constant]
pub const LIQUIDATION_THRESHOLD: u64 = 50;

#[constant]
pub const LIQUIDATION_BONUS: u64 = 10;

#[constant]
pub const MIN_HEALTH_FACTOR: u64 = 1;