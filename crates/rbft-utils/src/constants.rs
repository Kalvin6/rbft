// SPDX-License-Identifier: Apache-2.0
//! Shared constants for RBFT tooling.

/// Fallback admin private key used when `RBFT_ADMIN_KEY` is not set.
///
/// Used by `genesis`, `validator add`, and other commands that require an admin key.
/// Override with the `RBFT_ADMIN_KEY` environment variable to use a different key.
/// **This key is only the actual genesis admin when the testnet was generated without
/// `RBFT_ADMIN_KEY` set.**
pub const DEFAULT_ADMIN_KEY: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000001";

/// Ethereum address derived from [`DEFAULT_ADMIN_KEY`].
///
/// `cast wallet address --private-key 0x000...0001` → `0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf`
pub const DEFAULT_ADMIN_ADDRESS: &str = "0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf";
