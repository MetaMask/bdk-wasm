use bdk_wallet::bitcoin::{Network as BdkNetwork, NetworkKind as BdkNetworkKind};
use wasm_bindgen::prelude::wasm_bindgen;

/// What kind of network we are on.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetworkKind {
    /// The Bitcoin mainnet network.
    Main,
    /// Some kind of testnet network.
    Test,
}

impl From<BdkNetworkKind> for NetworkKind {
    fn from(network_kind: BdkNetworkKind) -> Self {
        match network_kind {
            BdkNetworkKind::Main => NetworkKind::Main,
            _ => NetworkKind::Test,
        }
    }
}

impl From<NetworkKind> for BdkNetworkKind {
    fn from(network_kind: NetworkKind) -> Self {
        match network_kind {
            NetworkKind::Main => BdkNetworkKind::Main,
            _ => BdkNetworkKind::Test,
        }
    }
}

impl From<BdkNetwork> for NetworkKind {
    fn from(network: BdkNetwork) -> Self {
        match network {
            BdkNetwork::Bitcoin => NetworkKind::Main,
            _ => NetworkKind::Test,
        }
    }
}

/// The cryptocurrency network to act on.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Network {
    /// Mainnet Bitcoin.
    Bitcoin = "bitcoin",
    /// Bitcoin's testnet network.
    Testnet = "testnet",
    /// Bitcoin's testnet4 network.
    Testnet4 = "testnet4",
    /// Bitcoin's signet network.
    Signet = "signet",
    /// Bitcoin's regtest network.
    Regtest = "regtest",
}

impl From<BdkNetwork> for Network {
    fn from(network: BdkNetwork) -> Self {
        match network {
            BdkNetwork::Testnet => Network::Testnet,
            BdkNetwork::Testnet4 => Network::Testnet4,
            BdkNetwork::Signet => Network::Signet,
            BdkNetwork::Regtest => Network::Regtest,
            _ => Network::Bitcoin,
        }
    }
}

impl From<Network> for BdkNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BdkNetwork::Bitcoin,
            Network::Testnet => BdkNetwork::Testnet,
            Network::Testnet4 => BdkNetwork::Testnet4,
            Network::Signet => BdkNetwork::Signet,
            Network::Regtest => BdkNetwork::Regtest,
            _ => BdkNetwork::Bitcoin,
        }
    }
}

impl From<Network> for BdkNetworkKind {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BdkNetworkKind::Main,
            _ => BdkNetworkKind::Test,
        }
    }
}
