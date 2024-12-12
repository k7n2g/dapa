use crate::crypto::Hash;

pub const VERSION: &str = env!("BUILD_VERSION");
pub const DAPA_ASSET: Hash = Hash::zero();
// 0.00000001 = a dap with 8 decimals to 1 DAPA coin
// Lowest fee per KB possible on the network
// 0.00010000 DAPA per KB
pub const FEE_PER_KB: u64 = 10000;
// 0.00100000 DAPA per account creation
// User can create an account with 0.001 DAPA
// Or can mine a block to be registered for free
pub const FEE_PER_ACCOUNT_CREATION: u64 = 100000;
// 0.00005000 DAPA per KB
// Each transfer has a overhead of 5000 atomic units
pub const FEE_PER_TRANSFER: u64 = 5000;

// 8 decimals numbers
pub const COIN_DECIMALS: u8 = 8;
// 100 000 000 to represent 1 DAPA Coin
pub const COIN_VALUE: u64 = 10u64.pow(COIN_DECIMALS as u32);
// 100 M coin
pub const MAXIMUM_SUPPLY: u64 = 100_000_000 * COIN_VALUE;

// Addresses format
// mainnet prefix address
pub const PREFIX_ADDRESS: &str = "dap";
// testnet prefix address
pub const TESTNET_PREFIX_ADDRESS: &str = "dah";

// 1 KB = 1024 bytes
pub const BYTES_PER_KB: usize = 1024;

// Max transaction size in bytes
pub const MAX_TRANSACTION_SIZE: usize = BYTES_PER_KB * BYTES_PER_KB; // 1 MB

// Max block size in bytes
// 1024 * 1024 + (256 * 1024) bytes = 1.25 MB maximum size per block with txs
pub const MAX_BLOCK_SIZE: usize = (BYTES_PER_KB * BYTES_PER_KB) + (256 * BYTES_PER_KB);

// BlockDAG rules
pub const TIPS_LIMIT: usize = 3; // maximum 3 TIPS per block