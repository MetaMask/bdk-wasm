use serde::Deserialize;

/// A wrapper for SLIP-10 Hierarchical Deterministic (HD) tree nodes, i.e.
/// cryptographic keys used to generate key pairs and addresses for cryptocurrency protocols.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SLIP10Node {
    /// The 0-indexed path depth of this node.
    pub depth: u8,

    /// The fingerprint of the master node, i.e., the node at depth 0. May be
    /// undefined if this node was created from an extended key.
    pub master_fingerprint: Option<u32>,

    /// The fingerprint of the parent key, or 0 if this is a master node.
    pub parent_fingerprint: u32,

    /// The index of the node, or 0 if this is a master node.
    pub index: u32,

    /// The (optional) private key of this node.
    pub private_key: Option<String>,

    /// The public key of this node.
    pub public_key: String,

    /// The chain code of this node.
    pub chain_code: String,

    ///The name of the curve used by the node.
    pub curve: String,
}
