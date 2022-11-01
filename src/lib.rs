use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DomainName {
    records: LookupMap<DomainString, AccountId>,
}

//pub struct DNSEntry {
//    records: LookupMap<FQDN, IPaddress>,
//}

impl Default for DomainName {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl DomainName {
    pub fn set_owner(&mut self, domain: String) {
        self.records.get(&domain);
        let account_id = env::signer_account_id();
        self.records.insert(&domain, &account_id);
    }

    pub fn get_owner(&self, domain: String) -> Option<String> {
        return self.records.get(&domain);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    use super::*;

    // Allows for modifying the environment of the mocked blockchain
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn set_get_message() {
        let mut context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        // Set the testing environment for the subsequent calls
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .build());

        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status(accounts(1)).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.near".parse().unwrap()));
    }
}
