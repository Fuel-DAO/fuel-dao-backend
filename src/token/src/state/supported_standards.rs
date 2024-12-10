use candid::CandidType;
use serde::{Deserialize, Serialize};

// Define the SupportedStandard record
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SupportedStandard {
    pub name: String,
    pub url: String,
}


// Define the service interface
#[ic_cdk_macros::query]
fn icrc10_supported_standards() -> Vec<SupportedStandard> {
    vec![
        SupportedStandard {
            name: "ICRC-7".to_string(),
            url: "https://github.com/dfinity/ICRC/tree/main/ICRCs/ICRC-7".to_string(),
        },
        SupportedStandard {
            name: "ICRC-10".to_string(),
            url: "https://github.com/dfinity/ICRC/tree/main/ICRCs/ICRC-10".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icrc10_supported_standards() {
        let standards = icrc10_supported_standards();
        assert_eq!(standards.len(), 2);
        assert_eq!(standards[0].name, "ICRC-7");
        assert_eq!(standards[0].url, "https://github.com/dfinity/ICRC/tree/main/ICRCs/ICRC-7");
        assert_eq!(standards[1].name, "ICRC-10");
        assert_eq!(standards[1].url, "https://github.com/dfinity/ICRC/tree/main/ICRCs/ICRC-10");
    }
}
