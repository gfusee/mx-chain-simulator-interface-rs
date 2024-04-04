use std::collections::HashMap;
use serde::Deserialize;

pub type InitialWallets = InitialWalletsResponseData;
pub type InitialWalletInfo = InitialWalletsResponseWallet;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitialWalletsResponseWallet {
    pub address: String,
    pub private_key_hex: String
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitialWalletsResponseData {
    pub initial_wallet_with_stake: InitialWalletInfo,
    pub shard_wallets: HashMap<String, InitialWalletInfo>
}

#[derive(Deserialize, Clone, Debug)]
pub struct InitialWalletsResponse {
    pub data: Option<InitialWalletsResponseData>,
    pub error: String,
    pub code: String
}