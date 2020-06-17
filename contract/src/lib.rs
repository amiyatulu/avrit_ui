use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Map, Set};
use near_sdk::{env, near_bindgen};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use uuid::{Builder, Uuid, Variant, Version};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn get_uuid(seed_vec: Vec<u8>) -> Uuid {
    let mut seed = [0u8; 32];
    let mut counter = 0;
    for v in seed_vec.iter() {
        seed[counter] = *v;
        counter += 1;
    }

    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    let uuid = Builder::from_bytes(bytes)
        .set_variant(Variant::RFC4122)
        .set_version(Version::Random)
        .build();
    return uuid;
}

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
    profile_hash: String, //IPFS Hash
    kyc_done: bool,
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
    product_details_hash: String, //IPFS Hash
}

#[near_bindgen]
impl Avrit {
    pub fn create_profile(&mut self, profile_hash: String) {
        let account_id = env::signer_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        let u = User {
            profile_hash,
            kyc_done: false,
        };
        match account_id_exists_option {
            Some(user_id) => {
                self.user_profile_map.insert(&user_id, &u);
            }
            None => {
                self.user_id += 1;
                self.user_map.insert(&account_id, &self.user_id);
                println!("{:?}: {:?}", account_id, self.user_id);
                self.user_profile_map.insert(&self.user_id, &u);
            }
        }
    }

    pub fn get_profile_hash(&self) -> String {
        let account_id = env::signer_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        match account_id_exists_option {
            Some(user_id) => {
                let userdata = self.user_profile_map.get(&user_id).unwrap();
                println!("{:?}", userdata.profile_hash);
                return userdata.profile_hash;
            }
            None => {
                panic!("User profile does not exists");
            }
        }
    }

    pub fn create_product(&mut self, product_tag: String, product_details_hash: String) {
        let account_id = env::signer_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        match account_id_exists_option {
            Some(user_id) => {
                let prod = Product {
                    user_id,
                    product_tag,
                    product_details_hash,
                };
                self.product_id += 1;
                self.product_map.insert(&self.product_id, &prod);
                let user_products_option = self.user_products_map.get(&user_id);
                match user_products_option {
                    Some(mut product_ids_set) => {
                        product_ids_set.insert(&self.product_id);
                        self.user_products_map.insert(&user_id, &product_ids_set);
                    }
                    None => {
                        let random_vec = env::random_seed();
                        let id = get_uuid(random_vec).to_string().into_bytes();
                        let mut product_ids_set = Set::new(id);
                        product_ids_set.insert(&self.product_id);
                        self.user_products_map.insert(&user_id, &product_ids_set);
                    }
                }
            }
            None => {
                panic!("User profile does not exists");
            }
        }
    }

    pub fn get_products_of_user(&self) -> Vec<u128> {
        let account_id = env::signer_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        match account_id_exists_option {
            Some(user_id) => {
                let products_set = self.user_products_map.get(&user_id).unwrap();
                return products_set.to_vec();
            }
            None => {
                panic!("User profile does not exists");
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
        let hash_string = "QmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();
        let hash_string2 = hash_string.clone();
        contract.create_profile(hash_string);
        let profile_hash = contract.get_profile_hash();
        assert_eq!(hash_string2, profile_hash);
        contract.create_profile("QmxeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned());
        let profile_hash = contract.get_profile_hash();
        assert_eq!(
            "QmxeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            profile_hash
        );
        contract.create_product(
            "evidence".to_owned(),
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );

        contract.create_product(
            "books".to_owned(),
            "Product2xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );
        let ids = contract.get_products_of_user();
        println!("{:?}", ids);
    }
}
