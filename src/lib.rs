use std::net::Ipv4Addr;
use std::str::FromStr;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DomainName {
    domain_name: LookupMap<String, AccountId>,
    fqdn: LookupMap<String, Ipv4Addr>,
}

impl Default for DomainName {
    fn default() -> Self {
        Self {
            domain_name: LookupMap::new(b"r".to_vec()),
            fqdn: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl DomainName {
    pub fn set_owner(&mut self, domain: String) {
        let account_id = env::signer_account_id();
        if let None = self.domain_name.get(&domain) {
            self.domain_name.insert(&domain, &account_id);
        }
    }

    pub fn get_owner(&self, domain: String) -> AccountId {
        return self.domain_name.get(&domain).unwrap();
    }

    pub fn set_fqdn(&mut self, fqdn: String, ip: String) {
        let account_id = env::signer_account_id();
        let mut split = fqdn.split(".");
        let mut vec = split.collect::<Vec<&str>>();
        let tld = vec.pop().unwrap();
        let domain_name = vec.pop().unwrap();
        let mut domain_name = domain_name.to_owned();
        domain_name.push_str(".");
        domain_name.push_str(tld);
        if let Some(domain_owner) = self.domain_name.get(&domain_name) {
            let ipv4 = Ipv4Addr::from_str(&ip).unwrap();
            if account_id == domain_owner {
                self.fqdn.insert(&fqdn, &ipv4);
            }
        }
    }

    pub fn get_fqdn(&self, fqdn: String) -> String{
        return self.fqdn.get(&fqdn).unwrap().to_string();
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
    fn test_dns() {
        let mut context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        // Set the testing environment for the subsequent calls
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .build());

        let mut contract = DomainName::default();
        contract.set_owner(String::from("toto.fr"));
        dbg!(contract.get_owner(String::from("toto.fr")));
        contract.set_fqdn(String::from("www.toto.fr"), String::from("8.8.8.8"));
        dbg!(contract.get_fqdn(String::from("www.toto.fr")));
    }
    /*
    
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
    */
}
