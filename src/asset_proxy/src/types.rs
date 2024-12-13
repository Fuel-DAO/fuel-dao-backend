use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

/// Equivalent to AssetStoreArg
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AssetStoreArg {
    pub key: String,
    pub content_type: String,
    pub content_encoding: String,
    pub content: Vec<u8>, // blob translates to Vec<u8> in Rust
    pub sha256: Option<Vec<u8>>, // Opt(blob) translates to Option<Vec<u8>> in Rust
}

/// Equivalent to ApproveFilesArg
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ApproveFilesArg {
    pub files: Vec<String>, // Vec(text) translates to Vec<String> in Rust
    pub asset_canister: Principal,
}


#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAssetArg {
    pub key: String,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct GetAssetArg {
    pub key: String,
    pub accept_encodings: Vec<String>,
}

#[derive(CandidType, Debug, Clone, Serialize, candid::Deserialize, )]
pub struct GetAssetResponse {
    pub content: Vec<u8>,            // The content of the asset as a vector of bytes
    pub sha256: Option<Vec<u8>>,     // Optional SHA256 hash of the content
    pub content_type: String,        // MIME type of the content
    pub content_encoding: String,    // Encoding used for the content
    pub total_length: Nat,           // Total length of the content
}