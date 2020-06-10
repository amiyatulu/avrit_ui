use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Map, Set, Vector};
use near_sdk::{env, near_bindgen};
use serde::{Deserialize, Serialize};
use std::fmt;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize, Debug)]
pub struct TextMessage {
    text: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: Map<String, String>,
}

#[near_bindgen]
impl Welcome {
    pub fn set_greeting(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn welcome(&self, account_id: String) -> TextMessage {
        match self.records.get(&account_id) {
            None => {
                env::log(b"Using default message.");
                return TextMessage {
                    text: format!("Hello {}", account_id),
                };
            }
            _ => {
                return TextMessage {
                    text: format!("{} {}", self.records.get(&account_id).unwrap(), account_id),
                }
            }
        }
    }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Profile {
    profile_map: Map<String, ProfileDetails>,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct ProductMap {
    product_map: Map<String, Product>,
}


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Debug)]
pub struct Product {
    product_tag: String,
    product_details: String, //IPFS Hash
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct ProfileDetails {
    profile_hash: String, //IPFS Hash
    products: Set<ProductMap>,
}

#[near_bindgen]
impl Profile {
    pub fn set_profile(&mut self, product_tag: String, product_details: String) {
        let account_id = String::from("amiyatulu.test");
        println!("{}", product_tag);
        let p = Product {
            product_tag,
            product_details,
        };
        let id = account_id.clone().into_bytes();
        let mut p_map = Map::new(id);
        let id2 = account_id.clone().into_bytes();
        p_map.insert(&"productid124243".to_string(), &p);
        let pmap = ProductMap { product_map: p_map };
        let mut id_products = ProfileDetails {
            products: Set::new(id2),
            profile_hash: "IPFSHASH".to_string(),
        };
        id_products.products.insert(&pmap);
        self.profile_map.insert(&account_id, &id_products);
    }

    pub fn push_product_to_profile(&mut self, product_tag: String, product_details: String) {
        let account_id = String::from("amiyatulu.test");
        let p = Product {
            product_tag,
            product_details,
        };
        let mut id = account_id.clone().into_bytes();
        id.push(28);

        let mut p_map = Map::new(id);
        p_map.insert(&"productidabc".to_string(), &p);
        let pmap = ProductMap { product_map: p_map };
        let profile_details_option = self.profile_map.get(&account_id);
        match profile_details_option {
            Some(mut profile_details) => {
                let productmapset = profile_details.products.to_vec();
                for i in 0..productmapset.len() {
                    println!("{:?}", productmapset[i].product_map.to_vec());
                }
                profile_details.products.insert(&pmap);
                self.profile_map.insert(&account_id, &profile_details);

            }
            None => println!("Can't get profile details"),
        }
    }

    pub fn get_product_list(&mut self) {
        let account_id = String::from("amiyatulu.test");
        let my_products_option = self.profile_map.get(&account_id);
        match my_products_option {
            Some(my_products) => {
                let data = my_products.products.to_vec();
                for i in 0..data.len() {
                    println!("{:?}", data[i].product_map.to_vec());
                }
            }
            None => println!("Can't get the profile tag"),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            epoch_height: 0,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Welcome::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            "howdy bob_near".to_string(),
            contract.welcome("bob_near".to_string()).text
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Welcome::default();
        println!(
            "Hello World {:?}",
            contract.welcome("francis.near".to_string())
        );
        assert_eq!(
            "Hello francis.near".to_string(),
            contract.welcome("francis.near".to_string()).text
        );
    }

    #[test]
    fn profile() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Profile::default();
        contract.set_profile("Cell biology".to_string(), "DNA".to_string());
        contract.push_product_to_profile("Mathematics".to_string(), "Set Theory".to_string());
        contract.get_product_list();
    }
}
