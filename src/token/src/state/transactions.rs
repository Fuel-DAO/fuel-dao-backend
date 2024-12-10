use candid::{Deserialize, CandidType};
use candid::Nat;

#[derive(Default, CandidType, Deserialize, Debug, Clone)]
pub struct TxnIndexStore {
    index: Nat,
}

impl TxnIndexStore {
    /// Creates a new instance of `TxnIndexStore`.
    pub fn new() -> Self {
        Self {
            index: Nat::from(0u64),
        }
    }

    /// Gets the current index.
    pub fn index(&self) -> &Nat {
        &self.index
    }

    /// Increments the index by 1.
    pub fn increment(&mut self) {
        self.index += 1u64;
    }

}
