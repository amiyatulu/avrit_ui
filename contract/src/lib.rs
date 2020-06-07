use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::Map;
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen};
use serde::{Deserialize, Serialize};

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
pub struct ProfileDetails {
    profile_tags: Map<String, ProductList>,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Debug)]
pub struct Product {
    product_name: String,
    product_details: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct ProductList {
    products: Vector<Product>,
}

#[near_bindgen]
impl ProfileDetails {
    pub fn set_profile(&mut self, product_name: String, product_details: String) {
        let account_id = String::from("amiyatulu.test");
        println!("{}", product_name);
        let p = Product {
            product_name,
            product_details,
        };
        let id = account_id.clone().into_bytes();
        let mut id_products = ProductList {
            products: Vector::new(id),
        };
        id_products.products.push(&p);
        self.profile_tags.insert(&account_id, &id_products);
    }

    pub fn push_product_to_profile(&mut self, product_name: String, product_details: String) {
        let account_id = String::from("amiyatulu.test");
        let p = Product {
            product_name,
            product_details,
        };
        let my_products_option = self.profile_tags.get(&account_id);
        match my_products_option {
            Some(mut my_products) => {
                my_products.products.push(&p);
                self.profile_tags.insert(&account_id, &my_products);
                println!("Hello myproducts push");
            }
            None => println!("Can't get the profile tag"),
        }
    }

    pub fn get_product_list(&mut self) {
        let account_id = String::from("amiyatulu.test");
        let my_products_option = self.profile_tags.get(&account_id);
        match my_products_option {
            Some(my_products) => {
                let data = my_products.products.get(0).unwrap();
                println!("{:?}", data);
                let data2 = my_products.products.get(1).unwrap();
                println!("{:?}", data2);
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
        let mut contract = ProfileDetails::default();
        contract.set_profile("Cell biology".to_string(), "DNA".to_string());
        contract.push_product_to_profile("Mathematics".to_string(), "Set Theory".to_string());
        contract.get_product_list();
    }
}
