use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SetStateResponseEmpty {}

#[derive(Deserialize)]
pub struct SetStateResponse {
    pub data: SetStateResponseEmpty,
    pub error: String,
    pub code: String
}

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetStateAddress {
    pub address: Option<String>,
    pub balance: Option<String>,
    pub code: Option<String>,
    pub root_hash: Option<String>,
    pub code_metadata: Option<String>,
    pub code_hash: Option<String>,
    pub developer_reward: Option<String>,
    pub owner_address: Option<String>,
    pub keys: Option<HashMap<String, String>>
}

impl SetStateAddress {
    pub fn new() -> SetStateAddress {
        Self::default()
    }

    pub fn with_address(mut self, address: String) -> SetStateAddress {
        self.address = Some(address);

        self
    }

    pub fn with_balance(mut self, balance: String) -> SetStateAddress {
        self.balance = Some(balance);

        self
    }

    pub fn with_code(mut self, code: String) -> SetStateAddress {
        self.code = Some(code);

        self
    }

    pub fn with_root_hash(mut self, root_hash: String) -> SetStateAddress {
        self.root_hash = Some(root_hash);

        self
    }

    pub fn with_code_metadata(mut self, code_metadata: String) -> SetStateAddress {
        self.code_metadata = Some(code_metadata);

        self
    }

    pub fn with_code_hash(mut self, code_hash: String) -> SetStateAddress {
        self.code_hash = Some(code_hash);

        self
    }

    pub fn with_developer_reward(mut self, developer_reward: String) -> SetStateAddress {
        self.developer_reward = Some(developer_reward);

        self
    }

    pub fn with_owner_address(mut self, owner_address: String) -> SetStateAddress {
        self.owner_address = Some(owner_address);

        self
    }

    pub fn with_keys(mut self, keys: HashMap<String, String>) -> SetStateAddress {
        self.keys = Some(keys);

        self
    }
}

impl Default for SetStateAddress {
    fn default() -> Self {
        Self {
            address: None,
            balance: None,
            code: None,
            root_hash: None,
            code_metadata: None,
            code_hash: None,
            developer_reward: None,
            owner_address: None,
            keys: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SetStateAddress;

    #[test]
    fn test_with_address() {
        let result = SetStateAddress::new()
            .with_address("test".to_string());

        let expected = SetStateAddress {
            address: Some("test".to_string()),
            balance: None,
            code: None,
            root_hash: None,
            code_metadata: None,
            code_hash: None,
            developer_reward: None,
            owner_address: None,
            keys: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_balance() {
        let result = SetStateAddress::new()
            .with_balance("100".to_string());

        let expected = SetStateAddress {
            address: None,
            balance: Some("100".to_string()),
            code: None,
            root_hash: None,
            code_metadata: None,
            code_hash: None,
            developer_reward: None,
            owner_address: None,
            keys: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_code() {
        let result = SetStateAddress::new()
            .with_code("test".to_string());

        let expected = SetStateAddress {
            address: None,
            balance: None,
            code: Some("test".to_string()),
            root_hash: None,
            code_metadata: None,
            code_hash: None,
            developer_reward: None,
            owner_address: None,
            keys: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_root_hash() {
        let result = SetStateAddress::new()
            .with_root_hash("test".to_string());

        let expected = SetStateAddress {
            address: None,
            balance: None,
            code: None,
            root_hash: Some("test".to_string()),
            code_metadata: None,
            code_hash: None,
            developer_reward: None,
            owner_address: None,
            keys: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_code_metadata() {
        let result = SetStateAddress::new()
            .with_code_metadata("test".to_string());

        let expected = SetStateAddress {
            address: None,
            balance: None,
            code: None,
            root_hash: None,
            code_metadata: Some("test".to_string()),
            code_hash: None,
            developer_reward: None,
            owner_address: None,
            keys: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_code_hash() {
        let result = SetStateAddress::new()
            .with_code_hash("test".to_string());

        let expected = SetStateAddress {
            address: None,
            balance: None,
            code: None,
            root_hash: None,
            code_metadata: None,
            code_hash: Some("test".to_string()),
            developer_reward: None,
            owner_address: None,
            keys: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_developer_reward() {
        let result = SetStateAddress::new()
            .with_developer_reward("100".to_string());

        let expected = SetStateAddress {
            address: None,
            balance: None,
            code: None,
            root_hash: None,
            code_metadata: None,
            code_hash: None,
            developer_reward: Some("100".to_string()),
            owner_address: None,
            keys: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_owner_address() {
        let result = SetStateAddress::new()
            .with_owner_address("test".to_string());

        let expected = SetStateAddress {
            address: None,
            balance: None,
            code: None,
            root_hash: None,
            code_metadata: None,
            code_hash: None,
            developer_reward: None,
            owner_address: Some("test".to_string()),
            keys: None,
        };

        assert_eq!(result, expected);
    }
}