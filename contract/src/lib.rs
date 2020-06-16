use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Map, Set};
use near_sdk::{env, near_bindgen};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Avrit {
    // UserId
    // ProductId
    // ReviewId
    // Map(Username, UserId)
    // Map(UserId, User)
    // Map(UserId, Set(ProductId))
    // Map(ProductId, Product)
    // Map(ProductId, Set(ReviewId))
    // Map(ReviewId, Review)
    user_id: u128,
    product_id: u128,
    review_id: u128,
    user_map: Map<String, u128>,               // (username, user_id)
    user_profile_map: Map<u128, User>,         // (user_id, User)
    product_map: Map<u128, Product>,           // (product_id, Product)
    review_map: Map<u128, Review>,             // (review_id, Review)
    user_products_map: Map<u128, Set<u128>>,   // (user_id, set<product_id>)
    product_reviews_map: Map<u128, Set<u128>>, // (product_id, set<review_id>)
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct User {
    profile_hash: String,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Review {
    product_id: u128,
    user_id: u128,
    review_hash: String, //IPFS Hash
}

#[derive(Default, BorshDeserialize, BorshSerialize, Debug)]
pub struct Product {
    user_id: u128,
    product_tag: String,
    product_details: String, //IPFS Hash
}

#[near_bindgen]
impl Avrit {
    pub fn create_profile(&mut self) {
        let account_id = env::signer_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        match account_id_exists_option {
            Some(user_id) => panic!(
                "Username {:?} name already exists and has id {:?}",
                account_id, user_id
            ),
            None => {
                self.user_id += 1;
                self.user_map.insert(&account_id, &self.user_id);
                println!("{:?}: {:?}", account_id, self.user_id);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use rand::Rng;

    fn rand_vector() -> Vec<u8> {
        let mut rng = rand::thread_rng();

        let mut randvec: Vec<u8> = Vec::new();
        let mut counter = 0;
        let result = loop {
            counter += 1;
            let n1: u8 = rng.gen();
            randvec.push(n1);

            if counter == 32 {
                break randvec;
            }
        };
        return result;
    }

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(), // The id of the account that owns the current contract
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2], // The public key of the account that did the signing.
            predecessor_account_id: "carol_near".to_string(), // The id of the account that was the previous contract in the chain of cross-contract calls. If this is the first contract, it is equal to signer_account_id.
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: rand_vector(),
            is_view,
            epoch_height: 0,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn profile() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Avrit::default();
        contract.create_profile();
        
    }

    #[test]
    #[should_panic(expected = "Username bob_near name already exists and has id 1")]
    fn create_again_profile() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Avrit::default();
        contract.create_profile();

    }
}
