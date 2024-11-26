use bdk_wallet::KeychainKind as BdkKeychainKind;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Types of keychains
pub enum KeychainKind {
    /// External keychain, used for deriving recipient addresses.
    External,
    /// Internal keychain, used for deriving change addresses.
    Internal,
}

impl From<BdkKeychainKind> for KeychainKind {
    fn from(keychain_kind: BdkKeychainKind) -> Self {
        match keychain_kind {
            BdkKeychainKind::External => KeychainKind::External,
            BdkKeychainKind::Internal => KeychainKind::Internal,
        }
    }
}

impl From<KeychainKind> for BdkKeychainKind {
    fn from(keychain_kind: KeychainKind) -> Self {
        match keychain_kind {
            KeychainKind::External => BdkKeychainKind::External,
            KeychainKind::Internal => BdkKeychainKind::Internal,
        }
    }
}
