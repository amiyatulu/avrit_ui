use chrono::{Duration, NaiveDateTime};
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, TreeMap, UnorderedSet};
use near_sdk::json_types::{ValidAccountId, U128, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use sha3::{Digest, Keccak256};

pub mod account;
pub use self::account::Account;
pub mod avritstructs;
pub use self::avritstructs::{CommentProduct, CommentReview, Product, Review, User};

/// Price per 1 byte of storage from mainnet genesis config.
pub const STORAGE_PRICE_PER_BYTE: Balance = 100000000000000000000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
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
    owner_id: AccountId,
    saving_id: AccountId,
    user_id: u128,
    product_id: u128,
    review_id: u128,
    comment_product_id: u128,
    comment_review_id: u128,
    update_user_ids: LookupMap<u128, u128>, //(incremental time number, update_user_id)
    update_user_id_time_counter: u128,
    update_product_ids: LookupMap<u128, u128>, //(incremental time number, updated_product_id)
    update_product_id_time_counter: u128,
    update_review_ids: LookupMap<u128, u128>, //(incremental time number, updated_review_id)
    update_review_id_time_counter: u128,
    user_map: TreeMap<String, u128>,       // (username, user_id)
    user_profile_map: TreeMap<u128, User>, // (user_id, User)
    product_map: TreeMap<u128, Product>,   // (product_id, Product)
    review_map: TreeMap<u128, Review>,     // (review_id, Review)
    comment_product_map: LookupMap<u128, CommentProduct>, // (comment_product_id, CommentProduct)
    comment_review_map: LookupMap<u128, CommentReview>, // (comment_review_id, CommentReview)
    user_products_map: TreeMap<u128, UnorderedSet<u128>>, // (user_id, set<product_id>)
    product_reviews_map: TreeMap<u128, UnorderedSet<u128>>, // (product_id, set<review_id>)
    product_commentproduct_map: LookupMap<u128, UnorderedSet<u128>>, // (product_id, set<commentproduct_id>)
    review_commentreview_map: LookupMap<u128, UnorderedSet<u128>>, // (review_id, set<commentreview_id>)
    product_bounty: LookupMap<u128, u64>, // (product_id, (bounty -> 0 index,  0_bountyperiodover 1_bountyperiodexists -> 1 index))
    review_bounty: LookupMap<u128, u64>, // (review_id, (bounty -> 0 index,  0_bountyperiodover 1_bountyperiodexists -> 1 index))
    min_review_bounty: u64,
    min_product_bounty: u64,
    min_jury_stake: u64,
    // max_number_of_jury_can_stake: u64,
    user_juror_stakes: LookupMap<u128, LookupMap<u128, u128>>, // <reviewer_id, <jurorid, stakes>> #Delete
    user_juror_stakes_clone: LookupMap<u128, TreeMap<u128, u128>>, // #Delete
    user_juror_stake_count: LookupMap<u128, u64>, // <review_id, juror that staked count>
    juror_stake_unique_id: u128,
    juror_unstaked: LookupMap<u128, LookupSet<u128>>, // <review_id, jurorid>
    selected_juror_count: LookupMap<u128, u64>,       // <review_id, selected_juror_count> #Delete
    selected_juror: LookupMap<u128, LookupSet<u128>>, // <reviewer_id, jurorid>  #Delete
    juror_selection_time: LookupMap<u128, u64>,       // <review_id, timestamp>
    jury_application_start_time: LookupMap<u128, u64>, // <review_id, time>
    product_id_set_ucount: u128,
    review_id_set_ucount: u128,
    jury_count: u64,
    jury_application_phase_time: u64, // Jury selection time in seconds
    commit_phase_time: u64,           // Commit phase time in seconds
    reveal_phase_time: u64,           // Reveal phase time in seconds
    voter_commit: LookupMap<u128, LookupMap<String, u8>>, // review_id, vote_commits, 1 if commited, 2 if revealed
    juror_voting_status: LookupMap<u128, LookupMap<u128, u8>>, // review_id, <juror id, 0 or null =not commited, 1=commited, 2=revealed, 3=got the incentives>
    schelling_decisions_juror: LookupMap<u128, LookupMap<u128, u8>>, // <review_id, <jurorid, 1=true 0=false>>
    schelling_decision_true_count: LookupMap<u128, u128>,            // <review_id, true_count>
    schelling_decision_false_count: LookupMap<u128, u128>,           // <review_id, false_count>
    jury_incentives: u128,                                           // Extra incentives on winning
    review_incentives: u128,                                         // Extra incentives on winning
    product_oa_incentives: u128, // Extra incentives for each review for open access content
    product_evidence_incentives: u128, // Extra incentives for each review for evidence of learning
    review_got_incentives: LookupMap<u128, u8>, // <review_id, 1 if got incentives>
    product_got_incentives: LookupMap<u128, LookupMap<u128, u8>>, // product_id <review_id, 1 if got incentives>
    product_incentives_count: LookupMap<u128, u128>, // product_id, product_incentives_count
    max_allowed_product_oa_incentives_count: u128,
    max_allowed_product_evidence_incentives_count: u128,
    number_of_allowed_reviews_per_product: u64,
    product_review_count: LookupMap<u128, u64>,
    ft: FungibleToken,
}

// Owner functions
#[near_bindgen]
impl Avrit {
    pub fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }
    pub fn change_owner(&mut self, new_owner: AccountId) {
        self.assert_owner();
        self.owner_id = new_owner;
    }
    pub fn change_saving_id(&mut self, new_saving_id: AccountId) {
        self.assert_owner();
        self.saving_id = new_saving_id;
    }
    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }
    pub fn set_commit_phase_time(&mut self, time_in_secs: u64) {
        self.assert_owner();
        self.commit_phase_time = time_in_secs;
    }
    pub fn get_commit_phase_time(self) -> U64 {
        self.commit_phase_time.into()
    }
    pub fn set_reveal_phase_time(&mut self, time_in_secs: u64) {
        self.assert_owner();
        self.reveal_phase_time = time_in_secs;
    }
    pub fn get_reveal_phase_time(&self) -> U64 {
        self.reveal_phase_time.into()
    }
    pub fn set_jury_application_phase_time(&mut self, time_in_secs: u64) {
        self.assert_owner();
        self.jury_application_phase_time = time_in_secs;
    }
    pub fn get_jury_application_phase_time(&self) -> U64 {
        self.jury_application_phase_time.into()
    }
    pub fn set_jury_count(&mut self, jury_count: u64) {
        self.assert_owner();
        self.jury_count = jury_count;
    }
    pub fn get_jury_count(&self) -> U64 {
        self.jury_count.into()
    }

    // pub fn set_max_number_of_jury_can_stake(&mut self, jury_count: U64) {
    //     self.assert_owner();
    //     self.max_number_of_jury_can_stake = jury_count.into();
    // }
    // pub fn get_max_number_of_jury_can_stake(&self) -> U64 {
    //     self.max_number_of_jury_can_stake.into()
    // }
    pub fn set_jury_incentives(&mut self, incentives: u128) {
        self.assert_owner();
        self.jury_incentives = incentives;
    }
    pub fn get_jury_incentives(&self) -> U128 {
        self.jury_incentives.into()
    }
    pub fn set_review_incentives(&mut self, incentives: u128) {
        self.assert_owner();
        self.review_incentives = incentives;
    }
    pub fn get_review_incentives(&self) -> U128 {
        self.review_incentives.into()
    }
    pub fn set_product_oa_incentives(&mut self, incentives: u128) {
        self.assert_owner();
        self.product_oa_incentives = incentives;
    }
    pub fn get_product_oa_incentives(&self) -> U128 {
        self.product_oa_incentives.into()
    }
    pub fn set_product_evidence_incentives(&mut self, incentives: u128) {
        self.assert_owner();
        self.product_evidence_incentives = incentives;
    }
    pub fn get_product_evidence_incentives(&self) -> U128 {
        self.product_evidence_incentives.into()
    }
    pub fn set_max_allowed_product_oa_incentives(&mut self, count: u128) {
        self.assert_owner();
        self.max_allowed_product_oa_incentives_count = count;
    }
    pub fn get_max_allowed_product_oa_incentives(&self) -> U128 {
        self.max_allowed_product_oa_incentives_count.into()
    }
    pub fn set_max_allowed_product_evidence_incentives(&mut self, count: u128) {
        self.assert_owner();
        self.max_allowed_product_evidence_incentives_count = count;
    }
    pub fn get_max_allowed_product_evidence_incentives(&self) -> U128 {
        self.max_allowed_product_evidence_incentives_count.into()
    }
    pub fn set_min_product_bounty(&mut self, bounty: u64) {
        self.assert_owner();
        self.min_product_bounty = bounty;
    }
    pub fn get_min_product_bounty(&self) -> U64 {
        self.min_product_bounty.into()
    }
    pub fn set_min_review_bounty(&mut self, bounty: u64) {
        self.assert_owner();
        self.min_review_bounty = bounty;
    }
    pub fn get_min_review_bounty(&self) -> U64 {
        self.min_review_bounty.into()
    }
    pub fn set_min_jury_stake(&mut self, stake: u64) {
        self.assert_owner();
        self.min_jury_stake = stake;
    }
    pub fn get_min_jury_stake(&self) -> U64 {
        self.min_jury_stake.into()
    }
    pub fn set_update_user_id_time_counter_zero(&mut self) {
        self.assert_owner();
        self.update_user_id_time_counter = 0;
    }
    pub fn get_update_user_id_time_counter(&self) -> U128 {
        self.update_user_id_time_counter.into()
    }
    pub fn set_update_product_id_time_counter_zero(&mut self) {
        self.assert_owner();
        self.update_product_id_time_counter = 0;
    }
    pub fn get_update_product_id_time_counter(&self) -> U128 {
        self.update_product_id_time_counter.into()
    }
    pub fn set_update_review_id_time_counter_zero(&mut self) {
        self.assert_owner();
        self.update_review_id_time_counter = 0;
    }
    pub fn get_update_review_id_time_counter(&self) -> U128 {
        self.update_review_id_time_counter.into()
    }
    pub fn get_update_user_ids(&self, counter: U128) -> U128 {
        let counter: u128 = counter.into();
        let update_user_ids_options = self.update_user_ids.get(&counter);
        match update_user_ids_options {
            Some(user_id) => user_id.into(),
            None => {
                panic!("The counter value key don't exists");
            }
        }
    }
    pub fn get_update_product_ids(&self, counter: U128) -> U128 {
        let counter: u128 = counter.into();
        let update_product_ids_options = self.update_product_ids.get(&counter);
        match update_product_ids_options {
            Some(product_id) => product_id.into(),
            None => {
                panic!("The counter value key don't exists");
            }
        }
    }
    pub fn get_update_review_ids(&self, counter: U128) -> U128 {
        let counter: u128 = counter.into();
        let update_review_ids_options = self.update_review_ids.get(&counter);
        match update_review_ids_options {
            Some(review_id) => review_id.into(),
            None => {
                panic!("The counter value key don't exists");
            }
        }
    }

    pub fn set_burn_percentage(&mut self, value: U128) {
        self.assert_owner();
        self.ft.change_burn_percentage(value.into());
    }

    pub fn get_burn_percentage(&self) -> u128 {
        self.ft.burn_percentage
    }
}

#[near_bindgen]
impl Avrit {
    fn get_user_id(&self, account_id: &AccountId) -> u128 {
        let user_id_option = self.user_map.get(&account_id);
        match user_id_option {
            Some(user_id) => user_id,
            None => {
                panic!("User id doesnot exist for AccountId {}", account_id);
            }
        }
    }

    pub fn get_user_id_js(&self, account_id: &AccountId) -> U128 {
        let user_id = self.get_user_id(account_id);
        user_id.into()
    }
    pub fn get_user_details(&self, user_id: U128) -> User {
        let user_id: u128 = user_id.into();
        let user_profile_option = self.user_profile_map.get(&user_id);
        let user = user_profile_option.unwrap();
        user
    }
    pub fn create_profile(&mut self, profile_hash: String) {
        let account_id = env::predecessor_account_id();
        if !self.ft.accounts.contains_key(&account_id) {
            self.ft.internal_register_account(&account_id);
        }
        let account_id_exists_option = self.user_map.get(&account_id);
        let u = User {
            profile_hash,
            kyc_done: false,
            username: account_id.clone(),
        };
        match account_id_exists_option {
            Some(_user_id) => {
                // self.user_profile_map.insert(&user_id, &u);
                panic!("User profile already exists");
            }
            None => {
                self.user_id += 1;
                self.user_map.insert(&account_id, &self.user_id);
                // println!("{:?}: {:?}", account_id, self.user_id);
                self.user_profile_map.insert(&self.user_id, &u);
            }
        }
    }

    pub fn update_profile(&mut self, profile_hash: String) {
        let account_id = env::predecessor_account_id();
        let account_id_number = self.get_user_id(&account_id);
        let user_profile_exists_option = self.user_profile_map.get(&account_id_number);
        match user_profile_exists_option {
            Some(mut user_profile) => {
                user_profile.profile_hash = profile_hash;
                self.user_profile_map
                    .insert(&account_id_number, &user_profile);
                self.update_user_id_time_counter += 1;
                self.update_user_ids
                    .insert(&self.update_user_id_time_counter, &account_id_number);
            }
            None => {
                panic!("Create user profile first");
            }
        }
    }

    pub fn get_profile_hash(&self) -> String {
        let account_id = env::predecessor_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        match account_id_exists_option {
            Some(user_id) => {
                println!("{:?}", user_id);
                let userdata = self.user_profile_map.get(&user_id).unwrap();
                println!("{:?}", userdata.profile_hash);
                return userdata.profile_hash;
            }
            None => {
                panic!("User profile does not exists");
            }
        }
    }

    pub fn get_profile_hash_from_id(&self, user_id: U128) -> String {
        let userdata_option = self.user_profile_map.get(&user_id.into());
        match userdata_option {
            Some(userdata) => userdata.profile_hash,
            None => {
                panic!("User profile does not exists");
            }
        }
    }

    pub fn get_user_profile_js(&self, user_id: U128) -> User {
        let user_id: u128 = user_id.into();
        self.get_user_profile(user_id)
    }

    fn get_user_profile(&self, user_id: u128) -> User {
        let user_option = self.user_profile_map.get(&user_id);
        match user_option {
            Some(user) => user,
            None => panic!("User profile for this id doesnot exist"),
        }
    }

    pub fn create_product(&mut self, product_details_hash: String, product_type: String) {
        let account_id = env::predecessor_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        match account_id_exists_option {
            Some(user_id) => {
                self.product_id += 1;
                let prod = Product {
                    user_id,
                    product_details_hash,
                    product_type,
                    product_expired: false,
                    product_id: self.product_id,
                };
                self.product_map.insert(&self.product_id, &prod);
                let user_products_option = self.user_products_map.get(&user_id);
                match user_products_option {
                    Some(mut product_ids_set) => {
                        product_ids_set.insert(&self.product_id);
                        self.user_products_map.insert(&user_id, &product_ids_set);
                    }
                    None => {
                        let s = "productidssetkey";
                        self.product_id_set_ucount = self.product_id_set_ucount + 1;
                        let t = format!("{}{}", s, self.product_id_set_ucount);
                        let id = t.to_string().into_bytes();
                        let mut product_ids_set = UnorderedSet::new(id);
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

    pub fn get_products_of_user_id_js(&self, user_id: U128, start: usize, end: usize) -> Vec<U128> {
        let user_id = user_id.into();
        let products_set_option = self.user_products_map.get(&user_id);
        match products_set_option {
            Some(products_set) => products_set
                .iter()
                .skip(start)
                .take(end)
                .map(|x| x.into())
                .collect(),
            None => {
                panic!("No products for user");
            }
        }
    }

    pub fn get_products_of_user_id(&self, user_id: u128, start: usize, end: usize) -> Vec<u128> {
        let products_set_option = self.user_products_map.get(&user_id);
        match products_set_option {
            Some(products_set) => products_set.iter().skip(start).take(end).collect(),
            None => {
                panic!("No products for user");
            }
        }
    }
    pub fn get_products_of_user(&self, start: usize, end: usize) -> Vec<u128> {
        let account_id = env::predecessor_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        match account_id_exists_option {
            Some(user_id) => {
                let productvec = self.get_products_of_user_id(user_id, start, end);
                productvec
            }
            None => {
                panic!("User profile does not exists");
            }
        }
    }

    pub fn update_product(&mut self, product_id: U128, product_details_hash: String) {
        let product_id = product_id.into();
        let account_id = env::predecessor_account_id();
        let mut product = self.get_product(product_id);
        // println!("{:?} user_id", product.user_id);
        let user_id = self.get_user_id(&account_id);
        // println!("{:?} user_id from account", user_id);
        if user_id == product.user_id {
            product.product_details_hash = product_details_hash;
        } else {
            panic!("You are not the product owner");
        }
        // println!("{:?} product", product);
        self.product_map.insert(&product_id, &product);
        self.update_product_id_time_counter += 1;
        self.update_product_ids
            .insert(&self.update_product_id_time_counter, &product_id);
    }

    pub fn get_product_js(&self, product_id: U128) -> Product {
        let product_id: u128 = product_id.into();
        self.get_product(product_id)
    }

    fn get_product(&self, product_id: u128) -> Product {
        let product_option = self.product_map.get(&product_id);
        match product_option {
            Some(product) => product,
            None => {
                panic!("No products for the id");
            }
        }
    }

    pub fn create_review(&mut self, product_id: U128, review_hash: String, rating: u8) {
        let product_id: u128 = product_id.into();
        let account_id = env::predecessor_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        let _product_exist = self.product_map.get(&product_id).unwrap();
        if rating > 5 {
            panic!("Rating can not be greater than 5");
        }
        match account_id_exists_option {
            Some(user_id) => {
                let rev = Review {
                    product_id,
                    user_id,
                    rating,
                    review_hash,
                };
                self.review_id += 1;
                self.review_map.insert(&self.review_id, &rev);
                let product_reviews_option = self.product_reviews_map.get(&product_id);
                match product_reviews_option {
                    Some(mut review_ids_set) => {
                        review_ids_set.insert(&self.review_id);
                        self.product_reviews_map
                            .insert(&product_id, &review_ids_set);
                    }
                    None => {
                        let s = "reviewidsetkey";
                        self.review_id_set_ucount = self.review_id_set_ucount + 1;
                        let t = format!("{}{}", s, self.review_id_set_ucount);
                        let id = t.to_string().into_bytes();
                        let mut review_ids_set = UnorderedSet::new(id);
                        review_ids_set.insert(&self.review_id);
                        self.product_reviews_map
                            .insert(&product_id, &review_ids_set);
                    }
                }
            }

            None => {
                panic!("User profile does not exists");
            }
        }
    }
    fn can_not_update_review_if_staked(&self, review_id: u128) {
        let bounty_option = self.review_bounty.get(&review_id);
        match bounty_option {
            Some(_bounty) => {
                panic!("Can not update review after it is staked");
            }
            None => {}
        }
    }
    pub fn update_review(&mut self, review_id: U128, review_hash: String, rating: u8) {
        let review_id = review_id.into();
        self.can_not_update_review_if_staked(review_id);
        let account_id = env::predecessor_account_id();
        let mut review = self.get_review(review_id);
        let user_id = self.get_user_id(&account_id);
        if user_id == review.user_id {
            review.review_hash = review_hash;
            review.rating = rating;
        } else {
            panic!("You are not the review owner");
        }

        self.review_map.insert(&review_id, &review);
        self.update_review_id_time_counter += 1;
        self.update_review_ids
            .insert(&self.update_review_id_time_counter, &review_id);
    }

    pub fn get_review_js(&self, review_id: U128) -> Review {
        let review_id = review_id.into();
        self.get_review(review_id)
    }
    fn get_review(&self, review_id: u128) -> Review {
        let reviewoption = self.review_map.get(&review_id);
        match reviewoption {
            Some(review) => review,
            None => {
                panic!("Review does not exists for the review");
            }
        }
    }

    pub fn get_review_ids_by_product_id(
        &self,
        product_id: U128,
        start: usize,
        end: usize,
    ) -> Vec<U128> {
        let product_id = product_id.into();
        let review_set_option = self.product_reviews_map.get(&product_id);
        match review_set_option {
            Some(review_set) => review_set
                .iter()
                .skip(start)
                .take(end)
                .map(|x| x.into())
                .collect(),
            None => {
                panic!("No reviews for product id");
            }
        }
    }

    pub fn create_comment_product(&mut self, product_id: U128, comment_hash: String) {
        let product_id: u128 = product_id.into();
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let comment = CommentProduct {
            product_id,
            user_id,
            comment_hash,
        };
        self.comment_product_id += 1;
        self.comment_product_map
            .insert(&self.comment_product_id, &comment);
        let product_commentproduct_option = self.product_commentproduct_map.get(&product_id);
        match product_commentproduct_option {
            Some(mut product_commentproduct_set) => {
                product_commentproduct_set.insert(&self.comment_product_id);
                self.product_commentproduct_map
                    .insert(&product_id, &product_commentproduct_set);
            }
            None => {
                let s = "commentproductsetkey";
                let t = format!("{}{}", s, product_id);
                let id = t.to_string().into_bytes();
                let mut product_commentproduct_set = UnorderedSet::new(id);
                product_commentproduct_set.insert(&self.comment_product_id);
                self.product_commentproduct_map
                    .insert(&product_id, &product_commentproduct_set);
            }
        }
    }

    pub fn create_comment_review(&mut self, review_id: U128, comment_hash: String) {
        let review_id: u128 = review_id.into();
        let account_id = env::predecessor_account_id();

        let user_id = self.get_user_id(&account_id);
        let comment = CommentReview {
            review_id,
            user_id,
            comment_hash,
        };
        self.comment_review_id += 1;
        self.comment_review_map
            .insert(&self.comment_review_id, &comment);
        let review_commentreview_option = self.review_commentreview_map.get(&review_id);
        match review_commentreview_option {
            Some(mut review_commentreview_set) => {
                review_commentreview_set.insert(&self.comment_review_id);
                self.review_commentreview_map
                    .insert(&review_id, &review_commentreview_set);
            }
            None => {
                let s = "reviewcommentsetkey";
                let t = format!("{}{}", s, review_id);
                let id = t.to_string().into_bytes();
                let mut review_commentreview_set = UnorderedSet::new(id);
                review_commentreview_set.insert(&self.comment_review_id);
                self.review_commentreview_map
                    .insert(&review_id, &review_commentreview_set);
            }
        }
    }

    pub fn get_commentproduct_by_product_id(
        &self,
        product_id: u128,
        start: usize,
        end: usize,
    ) -> Vec<u128> {
        let product_commentproduct_option = self.product_commentproduct_map.get(&product_id);
        match product_commentproduct_option {
            Some(commentproduct_set) => commentproduct_set.iter().skip(start).take(end).collect(),
            None => {
                panic!("No comments on product");
            }
        }
    }

    pub fn get_commentreview_by_review_id(
        &self,
        review_id: u128,
        start: usize,
        end: usize,
    ) -> Vec<u128> {
        let review_commentreview_option = self.review_commentreview_map.get(&review_id);
        match review_commentreview_option {
            Some(commentreview_set) => commentreview_set.iter().skip(start).take(end).collect(),
            None => {
                panic!("No comments on review");
            }
        }
    }
    pub fn add_product_bounty(&mut self, bounty: u64, product_id: U128) {
        let product_id: u128 = product_id.into();
        assert!(
            bounty >= self.min_product_bounty,
            "Bounty can not be less than minimum product bounty"
        );
        let account_id = env::predecessor_account_id();
        // println!(">>>>add product bounty{}<<<<<<<<<<", account_id);
        let product_bounty_exists_option = self.product_bounty.get(&product_id);
        match product_bounty_exists_option {
            Some(bountyvalue) => {
                if bounty > bountyvalue {
                    self.burn(&account_id, (bounty - bountyvalue) as u128);
                    self.product_bounty.insert(&product_id, &bounty);
                } else {
                    panic!("Please enter amount of higher value");
                }
            }
            None => {
                self.burn(&account_id, bounty as u128);
                self.product_bounty.insert(&product_id, &bounty);
            }
        }
    }

    pub fn get_product_bounty_js(&self, product_id: U128) -> U64 {
        let product_id: u128 = product_id.into();
        let bounty = self.get_product_bounty(product_id);
        bounty.into()
    }

    fn get_product_bounty(&self, product_id: u128) -> u64 {
        let bounty_option = self.product_bounty.get(&product_id);
        match bounty_option {
            Some(bounty) => bounty,
            None => {
                panic!("Bounty doesn't exists");
            }
        }
    }
    pub fn add_review_bounty(&mut self, bounty: u64, review_id: U128) {
        let review_id: u128 = review_id.into();
        self.check_products_reviewer_crossed(review_id);
        assert!(
            bounty >= self.min_review_bounty,
            "Bounty can not be less than minimum review bounty"
        );
        let account_id = env::predecessor_account_id();
        let timestamp = env::block_timestamp();
        let review_bounty_exists_option = self.review_bounty.get(&review_id);
        match review_bounty_exists_option {
            Some(bountyvalue) => {
                if bounty > bountyvalue {
                    self.burn(&account_id, (bounty - bountyvalue) as u128);
                    self.review_bounty.insert(&review_id, &bounty);
                } else {
                    panic!("Please enter amount of higher value");
                }
            }
            None => {
                self.burn(&account_id, bounty as u128);
                self.review_bounty.insert(&review_id, &bounty);
                self.jury_application_start_time
                    .insert(&review_id, &timestamp);
            }
        }
    }

    pub fn get_review_bounty_js(&self, review_id: U128) -> U64 {
        let review_id: u128 = review_id.into();
        let bounty = self.get_review_bounty(review_id);
        bounty.into()
    }

    fn get_review_bounty(&self, review_id: u128) -> u64 {
        let bounty_option = self.review_bounty.get(&review_id);
        match bounty_option {
            Some(bounty) => bounty,
            None => {
                panic!("Bounty does not exists");
            }
        }
    }

    fn check_products_reviewer_crossed(&self, review_id: u128) {
        let review_option = self.review_map.get(&review_id);
        let product_id;
        match review_option {
            Some(review) => {
                product_id = review.product_id;
            }
            None => {
                panic!("Product for the review donot exists");
            }
        }

        let product_review_count_option = self.product_review_count.get(&product_id);
        match product_review_count_option {
            Some(value) => {
                if value >= 10 {
                    panic!("You cannot stake more reviews")
                }
            }
            None => {}
        }
    }
}

#[near_bindgen]
impl Avrit {
    /// Initializes the contract with the given total supply owned by the given `owner_id`.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut this = Self {
            ft: FungibleToken::new(b"a".to_vec(), 0),
            owner_id: owner_id.clone(),
            saving_id: owner_id.clone(),
            user_id: 0,
            product_id: 0,
            review_id: 0,
            comment_product_id: 0,
            comment_review_id: 0,
            update_user_ids: LookupMap::new(b"f657bf68".to_vec()),
            update_user_id_time_counter: 0,
            update_product_ids: LookupMap::new(b"ef434fcd".to_vec()),
            update_product_id_time_counter: 0,
            update_review_ids: LookupMap::new(b"099e3a0a".to_vec()),
            update_review_id_time_counter: 0,
            user_map: TreeMap::new(b"061af613".to_vec()),
            user_profile_map: TreeMap::new(b"589d167f".to_vec()),
            product_map: TreeMap::new(b"cf27d94f".to_vec()),
            review_map: TreeMap::new(b"5fc2c77f".to_vec()),
            comment_product_map: LookupMap::new(b"fc337b34".to_vec()),
            comment_review_map: LookupMap::new(b"7859e655".to_vec()),
            user_products_map: TreeMap::new(b"e7b6e8a6".to_vec()),
            product_reviews_map: TreeMap::new(b"ea4ee217".to_vec()),
            product_commentproduct_map: LookupMap::new(b"fadfdeca".to_vec()),
            review_commentreview_map: LookupMap::new(b"00e72970".to_vec()),
            product_bounty: LookupMap::new(b"0566cfb4".to_vec()),
            review_bounty: LookupMap::new(b"00423f89".to_vec()),
            min_review_bounty: 10,
            min_product_bounty: 10,
            min_jury_stake: 10,
            product_id_set_ucount: 0,
            review_id_set_ucount: 0,
            user_juror_stakes: LookupMap::new(b"e56291ef".to_vec()),
            user_juror_stakes_clone: LookupMap::new(b"4e74c845".to_vec()),
            user_juror_stake_count: LookupMap::new(b"8c198c11".to_vec()),
            juror_unstaked: LookupMap::new(b"66fcbcd3".to_vec()),
            juror_stake_unique_id: 0,
            selected_juror: LookupMap::new(b"89390257".to_vec()),
            jury_count: 10,
            // max_number_of_jury_can_stake: 20,
            jury_application_phase_time: 1296000, // 15 days in secs
            commit_phase_time: 2592000,           // 30 days in secs
            reveal_phase_time: 1296000,           // 15 days in secs
            jury_incentives: 10,
            review_incentives: 50,
            product_oa_incentives: 10,
            product_evidence_incentives: 10,
            review_got_incentives: LookupMap::new(b"c296306e".to_vec()),
            product_got_incentives: LookupMap::new(b"2cdd4a9d".to_vec()),
            product_incentives_count: LookupMap::new(b"d2e3cb69".to_vec()),
            max_allowed_product_oa_incentives_count: 5,
            max_allowed_product_evidence_incentives_count: 5,
            selected_juror_count: LookupMap::new(b"532caf99".to_vec()),
            jury_application_start_time: LookupMap::new(b"1bff54ac".to_vec()),
            juror_selection_time: LookupMap::new(b"5942be3d".to_vec()),
            voter_commit: LookupMap::new(b"a11fe88d".to_vec()),
            juror_voting_status: LookupMap::new(b"4c4879f8".to_vec()),
            schelling_decisions_juror: LookupMap::new(b"8c7b8f85".to_vec()),
            schelling_decision_true_count: LookupMap::new(b"4bf8d29d".to_vec()),
            schelling_decision_false_count: LookupMap::new(b"98396f41".to_vec()),
            number_of_allowed_reviews_per_product: 10,
            product_review_count: LookupMap::new(b"05d53b2b".to_vec()),
        };
        this.ft.internal_register_account(&owner_id);
        this.ft.internal_deposit(&owner_id, total_supply.into());
        this
    }
}
near_contract_standards::impl_fungible_token_core!(Avrit, ft);
near_contract_standards::impl_fungible_token_storage!(Avrit, ft);

impl FungibleTokenMetadataProvider for Avrit {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("Avrit fungible token"),
            symbol: String::from("AVRIT"),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 24,
        }
    }
}

#[near_bindgen]
impl Avrit {
    pub fn register_account(&mut self, account_id: ValidAccountId) {
        self.ft.internal_register_account(account_id.as_ref());
    }
}

// Burn and mint
#[near_bindgen]
impl Avrit {
    fn mint(&mut self, owner_id: &AccountId, amount: u128) {
        if amount == 0 {
            env::panic(b"Can't transfer 0 tokens");
        }
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "New owner's account ID is invalid"
        );
        self.ft.internal_deposit(&owner_id, amount);
    }

    fn burn(&mut self, owner_id: &AccountId, amount: u128) {
        if amount == 0 {
            env::panic(b"Can't transfer 0 tokens");
        }
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Owner's account ID is invalid"
        );
        self.ft.internal_withdraw(&owner_id, amount);
    }
}

//shelling game
#[near_bindgen]
impl Avrit {
    fn get_rng(&self, seed_vec: Vec<u8>) -> StdRng {
        let mut seed = [0u8; 32];
        let mut counter = 0;
        for v in seed_vec.iter() {
            seed[counter] = *v;
            counter += 1;
        }
        let rng: StdRng = SeedableRng::from_seed(seed);
        rng
    }
    /// Main function: Juror application
    /// 1. Get the predecessor accound id number
    /// 2. Call user juror stake store and clone store
    pub fn apply_jurors(&mut self, review_id: U128, stake: U128) {
        let review_id: u128 = review_id.into();
        let stake: u128 = stake.into();
        let bountyvalue = self.get_review_bounty(review_id);
        if bountyvalue < self.min_review_bounty {
            panic!(
                "Bounty is less than minimum allowed amount {}",
                self.min_review_bounty
            );
        }
        if stake < self.min_jury_stake as u128 {
            panic!("Stake is less than minimum allowed amount")
        }
        self.increase_juror_that_staked_count(review_id.clone());
        // self.check_number_of_staked_jury_exceeds_max_number_of_jury_can_stake(review_id.clone());
        let account_id = env::predecessor_account_id();
        let singer_juror_user = self.get_user_id(&account_id);
        self.user_juror_stakes_store(
            account_id.clone(),
            singer_juror_user.clone(),
            review_id.clone(),
            stake.clone(),
        );
        self.user_juror_stakes_clone_store(
            singer_juror_user.clone(),
            review_id.clone(),
            stake.clone(),
        );
    }

    pub fn number_of_staked_jury(&self, review_id: U128) -> U64 {
        let user_juror_stake_count_option = self.user_juror_stake_count.get(&review_id.into());
        match user_juror_stake_count_option {
            Some(count) => count.into(),
            None => 0.into(),
        }
    }

    // fn number_of_staked_jury_internal(&self, review_id: u128) -> u64 {
    //     let user_juror_stake_count_option = self.user_juror_stake_count.get(&review_id);
    //     match user_juror_stake_count_option {
    //         Some(count) => count,
    //         None => 0,
    //     }
    // }

    // fn check_number_of_staked_jury_exceeds_max_number_of_jury_can_stake(&self, review_id: u128) {
    //     let user_juror_stake_count_option = self.user_juror_stake_count.get(&review_id);
    //     match user_juror_stake_count_option {
    //         Some(count) => {
    //             if count > self.max_number_of_jury_can_stake {
    //                 panic!("Staked juror are more than allowed juror");
    //             }
    //         }
    //         None => {}
    //     }
    // }

    fn increase_juror_that_staked_count(&mut self, review_id: u128) {
        let user_juror_stake_count_option = self.user_juror_stake_count.get(&review_id);
        match user_juror_stake_count_option {
            Some(mut count) => {
                count = count.checked_add(1).expect("overflow");
                self.user_juror_stake_count.insert(&review_id, &count);
            }
            None => {
                self.user_juror_stake_count.insert(&review_id, &1);
            }
        }
    }
    /// Receives account id, needed to call burn function for burning the stake
    /// signer_juror_user is account id number
    /// If stake entries exists for review id
    ///             If stake exists -> If stake is greater than zero, panic.
    ///                                Else (is zero) -> ** burn the stake and append stake and account id number to their reviewer id
    ///             None -> **
    /// None (If stake entries don't exist) -> Set the id for stake entries then **

    fn user_juror_stakes_store(
        &mut self,
        account_id: String,
        singer_juror_user: u128,
        review_id: u128,
        stake: u128,
    ) {
        let user_juror_stakes_option = self.user_juror_stakes.get(&review_id);
        match user_juror_stakes_option {
            Some(mut stake_entries) => {
                let stake_entries_option = stake_entries.get(&singer_juror_user);
                match stake_entries_option {
                    Some(stake) => {
                        if stake > 0 {
                            panic!("You have already staked")
                        } else {
                            stake_entries.insert(&singer_juror_user, &stake);
                            self.burn(&account_id, stake);
                            self.user_juror_stakes.insert(&review_id, &stake_entries);
                        }
                    }
                    None => {
                        stake_entries.insert(&singer_juror_user, &stake);
                        self.burn(&account_id, stake);
                        self.user_juror_stakes.insert(&review_id, &stake_entries);
                    }
                }
            }
            None => {
                let stakeidstring = format!(
                    "stakevoterid{}uniqueid{}",
                    review_id, self.juror_stake_unique_id
                );
                let stakeid = stakeidstring.to_string().into_bytes();
                let mut stake_entries = LookupMap::new(stakeid);
                stake_entries.insert(&singer_juror_user, &stake);
                self.burn(&account_id, stake);
                self.user_juror_stakes.insert(&review_id, &stake_entries);
            }
        }
    }

    ///  clone of user_juror_stakes_store
    fn user_juror_stakes_clone_store(
        &mut self,
        singer_juror_user: u128,
        review_id: u128,
        stake: u128,
    ) {
        let user_juror_stakes_option = self.user_juror_stakes_clone.get(&review_id);
        match user_juror_stakes_option {
            Some(mut stake_entries) => {
                let stake_entries_option = stake_entries.get(&singer_juror_user);
                match stake_entries_option {
                    Some(stake) => {
                        if stake > 0 {
                            panic!("You have already staked")
                        } else {
                            stake_entries.insert(&singer_juror_user, &stake);
                            self.user_juror_stakes_clone
                                .insert(&review_id, &stake_entries);
                        }
                    }
                    None => {
                        stake_entries.insert(&singer_juror_user, &stake);
                        self.user_juror_stakes_clone
                            .insert(&review_id, &stake_entries);
                    }
                }
            }
            None => {
                let stakeidstring = format!(
                    "stakevoteridclone{}uniqueid{}",
                    review_id, self.juror_stake_unique_id
                );
                self.juror_stake_unique_id += 1;
                let stakeid = stakeidstring.to_string().into_bytes();
                let mut stake_entries = TreeMap::new(stakeid);
                stake_entries.insert(&singer_juror_user, &stake);
                self.user_juror_stakes_clone
                    .insert(&review_id, &stake_entries);
            }
        }
    }

    fn get_jury_application_start_time(&self, review_id: u128) -> u64 {
        let timestamp_jury_application_start_time_option =
            self.jury_application_start_time.get(&review_id);
        match timestamp_jury_application_start_time_option {
            Some(timestamp) => timestamp,
            None => {
                panic!("No application time for review id");
            }
        }
    }

    pub fn get_jury_application_start_time_js(&self, review_id: U128) -> U64 {
        self.get_jury_application_start_time(review_id.into())
            .into()
    }

    fn assert_draw_jurors_time_possible(&self, review_id: u128) {
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp((timestamp / 1000000000) as i64, 0);
        // println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>naive now:   {}", naive_now);
        let timestamp_jury_application_start_time = self.get_jury_application_start_time(review_id);
        let native_timestamp_jury_application_start_time = NaiveDateTime::from_timestamp(
            (timestamp_jury_application_start_time / 1000000000) as i64,
            0,
        );
        let seconds = Duration::seconds(self.jury_application_phase_time as i64);
        let endtime = native_timestamp_jury_application_start_time + seconds;
        // println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>endtime draw juror: {}", endtime);
        if naive_now < endtime {
            panic!("Juror application time has not yet ended");
        }
    }

    /// Main function: Draw Jurors
    /// Check whether juror application time is over, if not through error
    /// Check there are mininum number of juror applied. ❌
    /// If juror is drawn, its get appended to selected_juror that contain review id nand juror id set
    /// draw_jurors don't require predecessor id
    pub fn draw_jurors(&mut self, review_id: U128) {
        let length: usize = 3;
        let review_id = review_id.into();
        self.assert_draw_jurors_time_possible(review_id);
        let selected_juror_option = self.selected_juror.get(&review_id);
        self.check_minimum_number_juror_staked(review_id);
        match selected_juror_option {
            Some(jurysetentries) => {
                self.draw_jurors_function(review_id, jurysetentries, length);
            }
            None => {
                let jurysetidstring = format!("jurysetid{}", review_id);
                let jurysetid = jurysetidstring.to_string().into_bytes();
                let jurysetentries = LookupSet::new(jurysetid);
                self.draw_jurors_function(review_id, jurysetentries, length);
            }
        }
    }

    pub fn get_selected_juror_count(&self, review_id: U128) -> U64 {
        let review_id: u128 = review_id.into();
        let selected_juror_count_option = self.selected_juror_count.get(&review_id);
        match selected_juror_count_option {
            Some(count) => count.into(),
            None => 0.into(),
        }
    }
    // Check minium number of juror have staked
    fn check_minimum_number_juror_staked(&self, review_id: u128) {
        let user_juror_stake_count_option = self.user_juror_stake_count.get(&review_id);
        match user_juror_stake_count_option {
            Some(count) => {
                if count < self.jury_count {
                    panic!("Staked juror are less than jury count");
                }
            }
            None => {}
        }
    }
    /// Recieves review id, selected jury id set of selected_juror for review id, and length (how many jury can be selected)
    /// Get the juries stake items (contains juror account id numbers and stakes) from user_juror_stakes_clone for review id
    /// Get the count of selected juror for the review id from selected_juror_count
    /// If count is greater than equal to self.jury_count you can't select more jurors panic
    /// After number of jurors selected gets equals to self.jury_count, timestamp is added to juror_selection_time with review id (it can be improved here, what if you don't get enough juror)
    fn draw_jurors_function(
        &mut self,
        review_id: u128,
        mut jurysetentries: LookupSet<u128>,
        length: usize,
    ) {
        let slicelength: u128 = 20;
        let user_juror_stakes_clone_option = self.user_juror_stakes_clone.get(&review_id);
        match user_juror_stakes_clone_option {
            Some(mut juries_stakes) => {
                let juries_stakes_len = juries_stakes.len() as u128;
                let random_vec = env::random_seed();
                let mut rng = self.get_rng(random_vec);
                // println!("juries stake len {:?}", juries_stakes_len);
                // println!("jury stakes {:?}", juries_stakes.to_vec());
                let mut end;
                let rand_number;
                if juries_stakes_len < slicelength {
                    end = juries_stakes_len;
                    rand_number = 0;
                } else {
                    rand_number = rng.gen_range(0, juries_stakes_len);
                    // println!("random number {}", rand_number);
                    end = rand_number + slicelength;
                    if end > juries_stakes_len {
                        end = juries_stakes_len;
                    }
                }
                // println!("start {} end {}", rand_number, end);
                let mut items = Vec::new();
                {
                    juries_stakes
                        .iter()
                        .skip(rand_number as usize)
                        .take(end as usize)
                        .for_each(|(key, value)| items.push((key, value)));
                    // ; println!("key value {}-{}", key, value)
                }
                // println!("items {:?}", items);
                let mut dist2 =
                    WeightedIndex::new(items.iter().map(|item| item.1)).expect("No items");
                let mut countvalue;
                let selected_juror_count_option = self.selected_juror_count.get(&review_id);
                match selected_juror_count_option {
                    Some(count) => {
                        countvalue = count;
                        let selectiontime_option = self.juror_selection_time.get(&review_id);
                        match selectiontime_option {
                            Some(_selection) => {
                                panic!("Jury selection time has already beend added")
                            }
                            None => {}
                        }
                    }
                    None => countvalue = 0,
                }
                for _ in 0..length {
                    let index = dist2.sample(&mut rng);
                    // println!("{}", index);
                    let drawindex = items[index].0;
                    // println!("draw_index {:?}", drawindex);
                    juries_stakes.remove(&drawindex);
                    jurysetentries.insert(&drawindex);
                    let d = dist2.update_weights(&[(index, &0)]);
                    // println!("{:?}",d);
                    countvalue = countvalue + 1;
                    if countvalue >= self.jury_count {
                        // println!("set time stamp");
                        let timestamp = env::block_timestamp();
                        self.juror_selection_time.insert(&review_id, &timestamp);
                        break;
                    }
                    match d {
                        Ok(_v) => {}
                        Err(_e) => {
                            break;
                        }
                    }
                }
                self.selected_juror_count.insert(&review_id, &countvalue);
                // println!("countvalue{}", countvalue);
                self.user_juror_stakes_clone
                    .insert(&review_id, &juries_stakes);
                self.selected_juror.insert(&review_id, &jurysetentries);
            }
            None => {
                panic!("There are no juries");
            }
        }
    }

    // pub fn get_selected_jurors(&self, review_id: u128) -> LookupSet<u128> {
    //     let selected_juror_option = self.selected_juror.get(&review_id);
    //     match selected_juror_option {
    //         Some(jurysetentries) => jurysetentries,
    //         None => {
    //             panic!("No selected jurors");
    //         }
    //     }
    // }
    fn get_juror_stakes(&self, review_id: u128, juror_user_id: u128) -> u128 {
        let juror_list_option = self.user_juror_stakes.get(&review_id);
        match juror_list_option {
            Some(juror_list) => {
                let juror_stake = juror_list.get(&juror_user_id).unwrap();
                juror_stake
            }
            None => panic!("No stakes for the review"),
        }
    }

    pub fn get_juror_stakes_js(&self, review_id: U128, juror_user_id: U128) -> U128 {
        let review_id: u128 = review_id.into();
        let juror_user_id: u128 = juror_user_id.into();
        let juror_stake = self.get_juror_stakes(review_id, juror_user_id);
        juror_stake.into()
    }

    fn get_juror_selection_time(&self, review_id: &u128) -> u64 {
        let timestamp_juror_selection_time_option = self.juror_selection_time.get(&review_id);
        match timestamp_juror_selection_time_option {
            Some(timestamp) => timestamp,
            None => {
                panic!("Jurors are not selected yet");
            }
        }
    }

    pub fn get_juror_selection_time_js(&self, review_id: U128) -> U64 {
        let timestamp = self.get_juror_selection_time(&review_id.into());
        timestamp.into()
    }

    // Logic for unstaking who are not selected as jury
    // Time passed juror_selection_time ✔️
    // check juror can not vote ✔️
    // check jury staker has already unstaked ✔️
    // unstake ✔️

    pub fn check_juror_selection_time_ended(&self, review_id: U128) {
        let review_id: u128 = review_id.into();
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp((timestamp / 1000000000) as i64, 0);
        let timestamp_juror_selection_time = self.get_juror_selection_time(&review_id);
        let native_juror_selection_time =
            NaiveDateTime::from_timestamp((timestamp_juror_selection_time / 1000000000) as i64, 0);
        let onehour = Duration::seconds(3600);
        let endtime = native_juror_selection_time + onehour; // One hour extra so that unstaking time is not manipulated
        if naive_now < endtime {
            panic!("Juror selection time has not yet ended");
        }
    }

    pub fn can_juror_unstake_bool(&self, review_id: U128, user_id: U128) -> bool {
        let review_id: u128 = review_id.into();
        let user_id: u128 = user_id.into();
        let juror_unstake_bool = self.has_juror_unstake_bool(review_id, user_id);
        let juror_staked_bool = self.has_juror_staked_bool(review_id, user_id);
        let juror_selected_bool = self.can_juror_selected_bool(review_id, user_id);
        if juror_unstake_bool == false && juror_staked_bool == true && juror_selected_bool == false
        {
            return true;
        } else {
            return false;
        }
    }

    fn has_juror_unstake_bool(&self, review_id: u128, user_id: u128) -> bool {
        let juror_unstaked_option = self.juror_unstaked.get(&review_id);
        match juror_unstaked_option {
            Some(juryentries) => {
                let juryexists = juryentries.contains(&user_id);
                if juryexists == true {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    fn can_juror_selected_bool(&self, review_id: u128, user_id: u128) -> bool {
        let selected_juror_option = self.selected_juror.get(&review_id);
        match selected_juror_option {
            Some(juryentries) => {
                let juryexists = juryentries.contains(&user_id);
                if juryexists == true {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    fn has_juror_staked_bool(&self, review_id: u128, user_id: u128) -> bool {
        let user_juror_stakes_option = self.user_juror_stakes.get(&review_id);
        match user_juror_stakes_option {
            Some(jurystakes) => {
                let jurystakes_option = jurystakes.get(&user_id);
                match jurystakes_option {
                    Some(_stake) => {
                        return true;
                    }
                    None => {
                        return false;
                    }
                }
            }
            None => {
                return false;
            }
        }
    }

    // pub fn can_juror_unstake_bool(&self, review_id: U128, user_id: U128) -> bool {
    //     let review_id: u128 = review_id.into();
    //     let user_id: u128 = user_id.into();
    //     let selected_juror_option = self.selected_juror.get(&review_id);
    //     match selected_juror_option {
    //         Some(juryentries) => {
    //             let juryexists = juryentries.contains(&user_id);
    //             if juryexists == true {
    //                 return false;
    //                 //panic!("You are selected as juror to vote, can't unstake");
    //             } else if juryexists == false {
    //                 let user_juror_stakes_option = self.user_juror_stakes.get(&review_id);
    //                 match user_juror_stakes_option {
    //                     Some(jurystakes) => {
    //                         let jurystakes_option = jurystakes.get(&user_id);
    //                         match jurystakes_option {
    //                             Some(_stake) => {
    //                                 let juror_unstaked_option = self.juror_unstaked.get(&review_id);
    //                                 match juror_unstaked_option {
    //                                     Some(mut juryentries) => {
    //                                         let juryexists = juryentries.contains(&user_id);
    //                                         if juryexists == true {
    //                                             panic!("You have alread unstaked, can not unstake again.");
    //                                         } else {
    //                                             juryentries.insert(&user_id);
    //                                             // self.juror_unstaked.insert(&review_id, &juryentries);
    //                                         }
    //                                     }
    //                                     None => {}
    //                                 }
    //                                 return true;
    //                             }
    //                             None => {
    //                                 return false;
    //                                 // panic!("You have not staked for the review");
    //                             }
    //                         }
    //                     }
    //                     None => {
    //                         return false;
    //                         // panic!("There are no stakes for the review");
    //                     }
    //                 }
    //             } else {
    //                 return false;
    //                 // panic!("You are selected as juror to vote, can't unstake");
    //             }
    //         }
    //         None => {
    //             return false;
    //             //panic!("Jury selection not done");
    //         }
    //     }
    // }

    // user_juror_stakes: LookupMap<u128, LookupMap<u128, u128>>, // <reviewer_id, <jurorid, stakes>>
    // selected_juror: LookupMap<u128, LookupSet<u128>>, // <reviewer_id, jurorid>

    fn juror_can_not_vote_return_stake(&self, review_id: u128, user_id: u128) -> u128 {
        let selected_juror_option = self.selected_juror.get(&review_id);
        match selected_juror_option {
            Some(juryentries) => {
                let juryexists = juryentries.contains(&user_id);
                if juryexists == true {
                    panic!("You are selected as juror to vote, can't unstake");
                } else if juryexists == false {
                    let user_juror_stakes_option = self.user_juror_stakes.get(&review_id);
                    match user_juror_stakes_option {
                        Some(jurystakes) => {
                            let jurystakes_option = jurystakes.get(&user_id);
                            match jurystakes_option {
                                Some(stake) => {
                                    return stake;
                                }
                                None => {
                                    panic!("You have not staked for the review");
                                }
                            }
                        }
                        None => {
                            panic!("There are no stakes for the review");
                        }
                    }
                } else {
                    panic!("You are selected as juror to vote, can't unstake");
                }
            }
            None => {
                panic!("Jury selection not done");
            }
        }
    }

    // juror_unstaked: LookupMap<u128, LookupSet<u128>>, // <review_id, jurorid>
    fn check_and_add_jury_unstaked(&mut self, review_id: u128, user_id: u128) {
        let juror_unstaked_option = self.juror_unstaked.get(&review_id);
        match juror_unstaked_option {
            Some(mut juryentries) => {
                let juryexists = juryentries.contains(&user_id);
                if juryexists == true {
                    panic!("You have alread unstaked, can not unstake again.");
                } else {
                    juryentries.insert(&user_id);
                    self.juror_unstaked.insert(&review_id, &juryentries);
                }
            }
            None => {
                // Add unstaked
                let jurysetidstring = format!("juryunstakeid{}", review_id);
                let jurysetid = jurysetidstring.to_string().into_bytes();
                let mut juryentries = LookupSet::new(jurysetid);
                juryentries.insert(&user_id);
                self.juror_unstaked.insert(&review_id, &juryentries);
            }
        }
    }

    pub fn unstaking_non_selected_juror(&mut self, review_id: U128, user_id: U128) {
        self.check_juror_selection_time_ended(review_id);
        let review_id: u128 = review_id.into();
        let user_id: u128 = user_id.into();
        let stake = self.juror_can_not_vote_return_stake(review_id, user_id);
        self.check_and_add_jury_unstaked(review_id, user_id);
        let user_profile = self.get_user_profile(user_id);
        let user_address = user_profile.username;
        self.mint(&user_address, stake);
    }

    /// Fetch the juror selection time from review id, get the commit phase time, add the both and get the endtime, if its less than now, panic
    /// Commit vote example 1thenUniqueString or 0thenUniqueString, than convert it to Keccak256 and produce the vote_commit string
    pub fn commit_vote(&mut self, review_id: U128, vote_commit: String) {
        let review_id: u128 = review_id.into();
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp((timestamp / 1000000000) as i64, 0);
        // println!("{}, now2", naive_now);
        let timestamp_juror_selection_time = self.get_juror_selection_time(&review_id);
        let native_juror_selection_time =
            NaiveDateTime::from_timestamp((timestamp_juror_selection_time / 1000000000) as i64, 0);
        let seconds = Duration::seconds(self.commit_phase_time as i64);
        let endtime = native_juror_selection_time + seconds;
        if naive_now > endtime {
            panic!("Commiting time has ended");
        }
        // println!(">>>>nativenow{} native_juror_selection_time{}<<<<", naive_now,native_juror_selection_time );
        if naive_now < native_juror_selection_time {
            panic!("Juror selection time has not ended");
        }
        self.can_juror_vote(review_id, user_id);
        self.add_juror_voting_status_commit(review_id, user_id);
        let mut vote_commit_all = self.get_voter_commits_lookup(review_id);
        let votecommit = vote_commit_all.get(&vote_commit);
        match votecommit {
            Some(_commit) => panic!("This vote is already commited"),
            None => {
                vote_commit_all.insert(&vote_commit, &1);
                self.voter_commit.insert(&review_id, &vote_commit_all);
            }
        }
    }
    fn add_juror_voting_status_commit(&mut self, review_id: u128, user_id: u128) {
        let juror_voting_status_option = self.juror_voting_status.get(&review_id);
        match juror_voting_status_option {
            Some(mut juror_voting_status_lookup) => {
                let juror_voting_status_lookup_option = juror_voting_status_lookup.get(&user_id);
                match juror_voting_status_lookup_option {
                    Some(value) => {
                        if value == 1 || value == 2 {
                            panic!("Voter has already commited");
                        } else {
                            panic!("Not at valid voter status");
                        }
                    }
                    None => {
                        juror_voting_status_lookup.insert(&user_id, &1);
                        self.juror_voting_status
                            .insert(&review_id, &juror_voting_status_lookup);
                    }
                }
            }
            None => {
                let votestatusstring = format!("juror_voting_status{}uniqueid", review_id);
                let votestatusid = votestatusstring.to_string().into_bytes();
                let mut votestatus_lookup = LookupMap::new(votestatusid);
                votestatus_lookup.insert(&user_id, &1);
                self.juror_voting_status
                    .insert(&review_id, &votestatus_lookup);
            }
        }
    }
    fn get_voter_commits_lookup(&self, review_id: u128) -> LookupMap<String, u8> {
        let vote_status_option = self.voter_commit.get(&review_id);
        match vote_status_option {
            Some(votecommits) => votecommits,
            None => {
                let votestatusstring = format!("votecommit{}uniqueid", review_id);
                let votestatusid = votestatusstring.to_string().into_bytes();
                let votecommits = LookupMap::new(votestatusid);
                votecommits
            }
        }
    }

    fn if_juror_selected(&self, review_id: u128, user_id: u128) -> bool {
        let selected_juror_option = self.selected_juror.get(&review_id);
        match selected_juror_option {
            Some(juryentries) => {
                let juryexists = juryentries.contains(&user_id);
                juryexists
            }
            None => false,
        }
    }

    pub fn can_juror_vote_bool(&self, review_id: U128, user_id: U128) -> bool {
        let review_id: u128 = review_id.into();
        let user_id: u128 = user_id.into();
        let selected = self.if_juror_selected(review_id, user_id);
        let commited = self.if_vote_commited(review_id, user_id);
        if selected == true && commited == false {
            true
        } else {
            false
        }
    }

    fn if_vote_commited(&self, review_id: u128, user_id: u128) -> bool {
        let juror_voting_status_option = self.juror_voting_status.get(&review_id);
        match juror_voting_status_option {
            Some(juror_voting_status_lookup) => {
                let juror_voting_status_lookup_option = juror_voting_status_lookup.get(&user_id);
                match juror_voting_status_lookup_option {
                    Some(value) => {
                        if value == 1 || value == 2 {
                            true
                        } else {
                            false
                        }
                    }
                    None => false,
                }
            }
            None => false,
        }
    }

    pub fn can_juror_vote_js(&self, review_id: U128, user_id: U128) {
        let review_id: u128 = review_id.into();
        let user_id: u128 = user_id.into();
        self.can_juror_vote(review_id, user_id);
    }
    fn can_juror_vote(&self, review_id: u128, user_id: u128) {
        let selected_juror_option = self.selected_juror.get(&review_id);
        match selected_juror_option {
            Some(jurysetentries) => {
                let juryexists = jurysetentries.contains(&user_id);
                // if user id exists in selected_juror (<review_id, juror_id_set>) it will
                // return true else false, look at the contains
                if juryexists == false {
                    panic!("You are not juror of the review");
                }
            }
            None => {
                panic!("No selected jurors");
            }
        }
    }

    /// Reveal end time is juror_selection_time + commit_phase_time + reveal_phase_time
    // Vote can be 0 or 1, 0=> Review is not good, 1=> Review is good as per guidelines
    pub fn reveal_vote(&mut self, review_id: U128, vote: String, vote_commit: String) {
        let review_id: u128 = review_id.into();
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp((timestamp / 1000000000) as i64, 0);
        let timestamp_juror_selection_time = self.get_juror_selection_time(&review_id);
        let native_juror_selection_time =
            NaiveDateTime::from_timestamp((timestamp_juror_selection_time / 1000000000) as i64, 0);
        let seconds = Duration::seconds(self.commit_phase_time as i64);
        let endtime = native_juror_selection_time + seconds;
        let reveal_end_seconds = Duration::seconds(self.reveal_phase_time as i64);
        let reveal_endtime = native_juror_selection_time + seconds + reveal_end_seconds;
        if naive_now < endtime {
            panic!("Commiting time has not ended");
        }
        if naive_now > reveal_endtime {
            panic!("Reveal time has ended"); // reveal phase time, when the reveal time ends
        }
        self.can_juror_vote(review_id, user_id);
        self.add_juror_voting_status_reveal(review_id, user_id);
        let mut vote_commit_all = self.get_voter_commits_in_reveal(review_id);
        let votecommit = vote_commit_all.get(&vote_commit);

        match votecommit {
            Some(commitstatus) => {
                if commitstatus == 2 {
                    panic!("The vote has be already revealed and added.");
                } else if commitstatus == 1 {
                    vote_commit_all.insert(&vote_commit, &2);
                    self.voter_commit.insert(&review_id, &vote_commit_all);
                }
            }
            None => {
                panic!("Vote with this commit is not present");
            }
        }
        let mut hasher = Keccak256::new();
        hasher.update(vote.as_bytes());
        let result = hasher.finalize();
        let vote_hex = format!("{:x}", result);
        if vote_commit == vote_hex {
            println!("commit and vote matches"); // comment out this step, only for debugging
        }
        if vote_commit != vote_hex {
            panic!("Vote hash doesn't match the vote commit");
        }

        let answer_id_string = format!("{}", &vote[0..1]);
        match answer_id_string.parse::<u8>() {
            Ok(n) => {
                if n > 1 {
                    panic!("Vote can be only 0 or 1");
                } else {
                    let schelling_decisions_juror_option =
                        self.schelling_decisions_juror.get(&review_id);
                    match schelling_decisions_juror_option {
                        Some(mut jurorsdecisionsall) => {
                            let jurydecisionoption = jurorsdecisionsall.get(&user_id);
                            match jurydecisionoption {
                                Some(value) => {
                                    panic!("You have already given the decision {}", value);
                                }
                                None => {
                                    jurorsdecisionsall.insert(&user_id, &n);
                                    self.schelling_decisions_juror
                                        .insert(&review_id, &jurorsdecisionsall);
                                    self.add_true_or_false_count(review_id, n);
                                }
                            }
                        }
                        None => {
                            let decisionstring =
                                format!("decisionstring{}uniqueid{}", review_id, self.user_id);
                            let decisionid = decisionstring.to_string().into_bytes();
                            let mut jurorsdecisionsallmap = LookupMap::new(decisionid);
                            jurorsdecisionsallmap.insert(&user_id, &n);
                            self.schelling_decisions_juror
                                .insert(&review_id, &jurorsdecisionsallmap);
                            self.add_true_or_false_count(review_id, n);
                        }
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    fn get_voter_commits_in_reveal(&self, review_id: u128) -> LookupMap<String, u8> {
        let vote_status_option = self.voter_commit.get(&review_id);
        match vote_status_option {
            Some(votecommits) => votecommits,
            None => {
                panic!("Voter commit doesnot exist for review_id");
            }
        }
    }

    pub fn can_juror_reveal(&self, review_id: U128, user_id: U128) -> bool {
        let review_id: u128 = review_id.into();
        let user_id: u128 = user_id.into();
        let juror_voting_status_option = self.juror_voting_status.get(&review_id);
        match juror_voting_status_option {
            Some(juror_voting_status_lookup) => {
                let juror_voting_status_lookup_option = juror_voting_status_lookup.get(&user_id);
                match juror_voting_status_lookup_option {
                    Some(value) => {
                        if value == 2 {
                            return false;
                        } else if value == 1 {
                            return true;
                        } else {
                            return false;
                        }
                    }
                    None => {
                        return false;
                        // panic!("Voting status doesnot exists, commit the vote first.");
                    }
                }
            }
            None => {
                return false;
                // panic!("Voting status lookup doesnot exists, commit the vote first.");
            }
        }
    }

    fn add_juror_voting_status_reveal(&mut self, review_id: u128, user_id: u128) {
        let juror_voting_status_option = self.juror_voting_status.get(&review_id);
        match juror_voting_status_option {
            Some(mut juror_voting_status_lookup) => {
                let juror_voting_status_lookup_option = juror_voting_status_lookup.get(&user_id);
                match juror_voting_status_lookup_option {
                    Some(value) => {
                        if value == 2 {
                            panic!("The juror has already been revealed a vote.");
                        } else if value == 1 {
                            juror_voting_status_lookup.insert(&user_id, &2);
                            self.juror_voting_status
                                .insert(&review_id, &juror_voting_status_lookup);
                        } else {
                            panic!("Not at valid voter status");
                        }
                    }
                    None => {
                        panic!("Voting status doesnot exists, commit the vote first.");
                    }
                }
            }
            None => {
                panic!("Voting status lookup doesnot exists, commit the vote first.");
            }
        }
    }

    fn add_true_or_false_count(&mut self, review_id: u128, value: u8) {
        if value == 0 {
            let schelling_decision_false_count_option =
                self.schelling_decision_false_count.get(&review_id);
            match schelling_decision_false_count_option {
                Some(mut count) => {
                    count += 1;
                    self.schelling_decision_false_count
                        .insert(&review_id, &count);
                }
                None => {
                    self.schelling_decision_false_count.insert(&review_id, &1);
                }
            }
        }
        if value == 1 {
            let schelling_decision_true_count_option =
                self.schelling_decision_true_count.get(&review_id);
            match schelling_decision_true_count_option {
                Some(mut count) => {
                    count += 1;
                    self.schelling_decision_true_count
                        .insert(&review_id, &count);
                }
                None => {
                    self.schelling_decision_true_count.insert(&review_id, &1);
                }
            }
        }
    }

    fn get_true_count(&self, review_id: u128) -> u128 {
        let count_option = self.schelling_decision_true_count.get(&review_id);
        match count_option {
            Some(count) => count,
            None => 0,
        }
    }

    pub fn get_true_count_js(&self, review_id: U128) -> U128 {
        let review_id: u128 = review_id.into();
        let count = self.get_true_count(review_id);
        count.into()
    }

    fn get_false_count(&self, review_id: u128) -> u128 {
        let count_option = self.schelling_decision_false_count.get(&review_id);
        match count_option {
            Some(count) => count,
            None => 0,
        }
    }

    pub fn get_false_count_js(&self, review_id: U128) -> U128 {
        let review_id: u128 = review_id.into();
        let count = self.get_false_count(review_id);
        count.into()
    }
    pub fn get_winning_decision(&self, review_id: U128) -> u8 {
        let review_id: u128 = review_id.into();
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp((timestamp / 1000000000) as i64, 0);
        let timestamp_juror_selection_time = self.get_juror_selection_time(&review_id);
        let native_juror_selection_time =
            NaiveDateTime::from_timestamp((timestamp_juror_selection_time / 1000000000) as i64, 0);
        let seconds = Duration::seconds(self.commit_phase_time as i64);
        let reveal_end_seconds = Duration::seconds(self.reveal_phase_time as i64);
        let onehour = Duration::seconds(3600);
        let reveal_endtime = native_juror_selection_time + seconds + reveal_end_seconds + onehour; // One hour extra so that incentive time is not manipulated
        if naive_now < reveal_endtime {
            panic!("Reveal time has not yet ended."); // when the reveal time ends
        }
        let truecount = self.get_true_count(review_id);
        let falsecount = self.get_false_count(review_id);
        if truecount > falsecount {
            1
        } else if falsecount > truecount {
            0
        } else if falsecount == truecount {
            2
        } else {
            3
        }
    }

    pub fn incentives_distribution(&mut self, review_id: U128) {
        let review_id: u128 = review_id.into();
        let account_id = env::predecessor_account_id();
        // println!(">>>>>>>>>>>>>>>>>>>>>>>>>accountid {}<<<<<<<<<<<<<<<<<<<<<", account_id);
        let user_id = self.get_user_id(&account_id);
        self.can_juror_vote(review_id, user_id);
        let winning_decision = self.get_winning_decision(review_id.into());
        let juror_stake = self.get_juror_stakes(review_id, user_id);
        let schelling_decisions_juror_option = self.schelling_decisions_juror.get(&review_id);
        match schelling_decisions_juror_option {
            Some(decisionlookup) => {
                let decisionlookup_option = decisionlookup.get(&user_id);
                match decisionlookup_option {
                    Some(decision) => {
                        if decision == winning_decision {
                            let mint_value = juror_stake + self.jury_incentives;
                            self.add_juror_voting_status_got_incentives(review_id, user_id);
                            self.mint(&account_id, mint_value);
                        }
                        // else if winning_decision == 2{   }
                        else if decision != winning_decision && winning_decision != 3 {
                            self.add_juror_voting_status_got_incentives(review_id, user_id);
                            let mint_value = (juror_stake as f64).powf(0.8) as u128;
                            // println!(">>>>>>>>>>>>>mintvalue{}<<<<<<<<<<<<<<<<<<<", mint_value);
                            if mint_value > self.jury_incentives {
                                self.mint(&account_id, mint_value);
                            }
                        }
                    }
                    None => {
                        panic!("Decision doesnot exists for the user id");
                    }
                }
            }
            None => {
                panic!("Juror decisions don't exist for this review id.");
            }
        }
    }

    pub fn if_juror_will_get_incentives(&self, review_id: U128, user_id: U128) -> bool {
        let review_id = review_id.into();
        let user_id = user_id.into();
        let juror_voting_status_option = self.juror_voting_status.get(&review_id);
        match juror_voting_status_option {
            Some(juror_voting_status_lookup) => {
                let juror_voting_status_lookup_option = juror_voting_status_lookup.get(&user_id);
                match juror_voting_status_lookup_option {
                    Some(value) => {
                        if value == 3 {
                            return false;
                            // panic!("Juror already got the incentives");
                        } else if value == 2 {
                            return true;
                        } else if value == 1 {
                            return false;
                            // panic!("You have not yet revealed the vote");
                        } else {
                            return false;
                            // panic!("Not at valid voter status");
                        }
                    }
                    None => {
                        return false;
                        // panic!("Voting status doesnot exists, commit the vote first.");
                    }
                }
            }
            None => {
                return false;
                // panic!("Voting status lookup doesnot exists, commit the vote first.");
            }
        }
    }

    fn add_juror_voting_status_got_incentives(&mut self, review_id: u128, user_id: u128) {
        let juror_voting_status_option = self.juror_voting_status.get(&review_id);
        match juror_voting_status_option {
            Some(mut juror_voting_status_lookup) => {
                let juror_voting_status_lookup_option = juror_voting_status_lookup.get(&user_id);
                match juror_voting_status_lookup_option {
                    Some(value) => {
                        if value == 3 {
                            panic!("Juror already got the incentives");
                        } else if value == 2 {
                            juror_voting_status_lookup.insert(&user_id, &3);
                            self.juror_voting_status
                                .insert(&review_id, &juror_voting_status_lookup);
                        } else if value == 1 {
                            panic!("You have not yet revealed the vote");
                        } else {
                            panic!("Not at valid voter status");
                        }
                    }
                    None => {
                        panic!("Voting status doesnot exists, commit the vote first.");
                    }
                }
            }
            None => {
                panic!("Voting status lookup doesnot exists, commit the vote first.");
            }
        }
    }

    fn check_review_got_incentives(&mut self, review_id: u128) {
        let review_got_incentives_option = self.review_got_incentives.get(&review_id);
        match review_got_incentives_option {
            Some(value) => {
                if value == 1 {
                    panic!("Incentives is already given")
                } else {
                    self.review_got_incentives.insert(&review_id, &1);
                }
            }
            None => {
                self.review_got_incentives.insert(&review_id, &1);
            }
        }
    }

    pub fn if_review_get_incentives_bool(&self, review_id: U128) -> bool {
        let review_id = review_id.into();
        let review_got_incentives_option = self.review_got_incentives.get(&review_id);
        match review_got_incentives_option {
            Some(value) => {
                if value == 1 {
                    return false;
                    // panic!("Incentives is already given")
                } else {
                    return true;
                }
            }
            None => {
                return true;
            }
        }
    }

    pub fn incentive_distribution_reviewer(&mut self, review_id: U128) {
        let winning_decision = self.get_winning_decision(review_id);
        let review_id: u128 = review_id.into();
        let bountyvalue = self.get_review_bounty(review_id);
        let review = self.get_review(review_id);
        let review_user_id = review.user_id;
        let user = self.get_user_profile(review_user_id);
        let review_incentives = self.review_incentives;
        let user_address = user.username;
        if winning_decision == 1 {
            self.check_review_got_incentives(review_id);
            self.mint(&user_address, review_incentives + bountyvalue as u128);
        }
    }

    /// Incentive for Product
    /// Check review is done for the product ✔️
    /// Check product is given bounty ✔️

    // product_oa_incentives: u128, // Extra incentives for each review for open access content
    // product_evidence_incentives: u128, // Extra incentives for each review for evidence of learning
    // product_got_incentives: LookupMap<u128, LookupMap<u128, u8>>, // product_id <review_id, 1 if got incentives>
    // product_incentives_count: LookupMap<u128, u128>, // product_id, product_incentives_count
    // max_allowed_product_oa_incentives_count: u128,
    // max_allowed_product_evidence_incentives_count: u128,

    /// Check the product is already incentivised for the review  ✔️
    /// Increment the product incentives count (number of incentives product gets) ✔️
    /// Check the products don't exceed number of allowed review for incentives ✔️
    /// Check the product is evidence of learning or open access  ✔️
    /// provide incentives only for this two category ✔️
    //  Incentives should be given when review is good, that is winning decision is 1 and review has rated the content >= 4 stars ✔️

    pub fn check_product_will_get_incentives_bool(
        &self,
        product_id: U128,
        review_id: U128,
    ) -> bool {
        let product_id = product_id.into();
        let review_id = review_id.into();
        let product_evidence_incentives_option = self.product_got_incentives.get(&product_id);
        match product_evidence_incentives_option {
            Some(review_product_incentives_lookup) => {
                let review_product_incentives_lookup_option =
                    review_product_incentives_lookup.get(&review_id);
                match review_product_incentives_lookup_option {
                    Some(value) => {
                        if value == 1 {
                            return false;
                            // panic!("Incentives is already given");
                        } else {
                            return true;
                        }
                    }
                    None => {
                        return true;
                    }
                }
            }
            None => {
                return true;
            }
        }
    }
    fn check_if_product_will_get_incentives(&mut self, product_id: u128, review_id: u128) {
        let product_evidence_incentives_option = self.product_got_incentives.get(&product_id);
        match product_evidence_incentives_option {
            Some(mut review_product_incentives_lookup) => {
                let review_product_incentives_lookup_option =
                    review_product_incentives_lookup.get(&review_id);
                match review_product_incentives_lookup_option {
                    Some(value) => {
                        if value == 1 {
                            panic!("Incentives is already given");
                        } else {
                            review_product_incentives_lookup.insert(&review_id, &1);
                            self.product_got_incentives
                                .insert(&product_id, &review_product_incentives_lookup);
                        }
                    }
                    None => {
                        review_product_incentives_lookup.insert(&review_id, &1);
                        self.product_got_incentives
                            .insert(&product_id, &review_product_incentives_lookup);
                    }
                }
            }
            None => {
                let uniqueidstring = format!("product_got_incentives{}", product_id);
                let uniqueid = uniqueidstring.to_string().into_bytes();
                let mut productgotincentives = LookupMap::new(uniqueid);
                productgotincentives.insert(&review_id, &1);
                self.product_got_incentives
                    .insert(&product_id, &productgotincentives);
            }
        }
    }

    fn increment_product_incentive_count_check_allowed_limit(
        &mut self,
        product_type: String,
        product_id: u128,
    ) -> u128 {
        let product_incentive_count_option = self.product_incentives_count.get(&product_id);
        assert!(
            product_type == "oa" || product_type == "ev",
            "Only evidence of learning and open access gets incentives"
        );
        match product_incentive_count_option {
            Some(product_incentive_count) => {
                let mut incentives = 0;
                let count = product_incentive_count + 1;
                self.product_incentives_count.insert(&product_id, &count);
                if product_type == "oa" {
                    assert!(
                        product_incentive_count <= self.max_allowed_product_oa_incentives_count,
                        "Exceeds the number of allowed reviews"
                    );
                    if count <= 2 {
                        incentives = self.product_oa_incentives/2;
                    } else {
                        incentives = self.product_oa_incentives / self.max_allowed_product_oa_incentives_count;
                    }
                    
                } else if product_type == "ev" {
                    assert!(
                        product_incentive_count
                            <= self.max_allowed_product_evidence_incentives_count,
                        "Exceeds the number of allowed reviews"
                    );
                    if count <= 2 {
                        incentives = self.product_evidence_incentives/2;
                    } else {
                        incentives = self.product_evidence_incentives / self.max_allowed_product_evidence_incentives_count;
                    }
                    
                }

                incentives
                
                
            }
            None => {
                self.product_incentives_count.insert(&product_id, &1);
                let mut incentives = 0;
                if product_type == "oa" {
                    incentives = self.product_oa_incentives/2;
                    
                } else if product_type == "ev" {
                    incentives = self.product_evidence_incentives/2;
                    
                }
                incentives
            }
        }
    }

    pub fn incentive_distribution_product(&mut self, product_id: U128, review_id: U128) {
        let product_id: u128 = product_id.into();
        let review_id: u128 = review_id.into();
        let review = self.get_review(review_id);
        let product_review_id = review.product_id;
        assert!(
            product_id == product_review_id,
            "Review and product do not match"
        );
        let product = self.get_product(product_id);
        let product_type = product.product_type;
        let product_user_id = product.user_id;
        let user = self.get_user_profile(product_user_id);
        let user_address = user.username;
        let _bounty = self.get_product_bounty(product_id);
        self.check_if_product_will_get_incentives(product_id, review_id);
        let product_incentives =
            self.increment_product_incentive_count_check_allowed_limit(product_type, product_id);
        assert!(
            product_incentives > 0,
            "Incentives should be greater than 0"
        );
        let winning_decision = self.get_winning_decision(review_id.into());
        if winning_decision == 1 && review.rating >= 3 {
            self.mint(&user_address, product_incentives);
        } else {
            panic!("You are not eligible for incentives");
        }
    }
}

// To Do:
// Limit the number of allowed reviews that will get incentives per product. ✔️
// Same user cannot give multiple reviews to same product ❌
// Give back stake of non juror after jury selection ✔️
// Review can't be updated after bounty/stake is given for it ✔️
// Product incentives even after no reviews after one month.
// Future improvements: Random selection of 10 staked reviews for incentives.
// Set kyc variable by admin

// Future To Dos (Not required now):
// Set the threshold limit of incentives (for jury, review and product) that admin can't exceed
// Separate threshold admin and setting incentive admin
// Ability to remove admin

// Time Travel
// juror_selection_time
// commit votes end time = juror_selection_time + commit_phase_time
// reveal vote end time = juror_selection_time + comit_phase_time + reveal_phase_time
