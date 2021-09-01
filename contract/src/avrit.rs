use chrono::{Duration, NaiveDateTime};
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, LookupSet, TreeMap, UnorderedSet};
use near_sdk::json_types::{ValidAccountId, U64};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue,
};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use sha3::{Digest, Keccak256};
use std::convert::TryInto;

pub mod avritstructs;
pub use self::avritstructs::{CommentProduct, CommentReview, Communication, Product, Review, User};

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
    communication_id: u128,
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
    communication_map: TreeMap<u128, Communication>, // (communication_id, Communication)
    comment_product_map: LookupMap<u128, CommentProduct>, // (comment_product_id, CommentProduct)
    comment_review_map: LookupMap<u128, CommentReview>, // (comment_review_id, CommentReview)
    user_products_map: TreeMap<u128, UnorderedSet<u128>>, // (user_id, set<product_id>)
    product_reviews_map: TreeMap<u128, UnorderedSet<u128>>, // (product_id, set<review_id>)
    product_commentproduct_map: LookupMap<u128, UnorderedSet<u128>>, // (product_id, set<commentproduct_id>)
    review_commentreview_map: LookupMap<u128, UnorderedSet<u128>>, // (review_id, set<commentreview_id>)
    product_crowdfunding: LookupMap<u128, u128>,                   // (product_id, bounty)
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
    // Crowdsale
    token_price: u128,
    token_sold: u128,
    total_available_tokens: u128,
    phase_available_tokens: u128,
    on_crowdsale: bool,
    // NFT
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    nft_token_count: LookupMap<u128, u128>, // <NFT token id, nft count>
    nft_token_mint_count: LookupMap<u128, u128>, // <NFT token id, nft mint count>
    nft_token_price: LookupMap<u128, u128>, // <NFT token id, price in yocto near>
    nft_owner_incentives: LookupMap<u128, u128>, // <Profile id, incentives>
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
    pub fn change_token_price(&mut self, token_price: U128) {
        self.assert_owner();
        let token_price: u128 = token_price.into();
        self.token_price = token_price;
    }
    pub fn change_on_crowdsale(&mut self, on_crowdsale: bool) {
        self.assert_owner();
        self.on_crowdsale = on_crowdsale;
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

    pub fn get_final_product_id(&self) -> U128 {
        self.product_id.into()
    }

    pub fn get_final_review_id(&self) -> U128 {
        self.review_id.into()
    }

    pub fn get_final_communcation_id(&self) -> U128 {
        self.communication_id.into()
    }

    pub fn set_burn_percentage(&mut self, value: U128) {
        self.assert_owner();
        self.ft.change_burn_percentage(value.into());
    }

    pub fn get_burn_percentage(&self) -> u128 {
        self.ft.burn_percentage
    }
    pub fn get_total_available_tokens(&self) -> U128 {
        self.total_available_tokens.into()
    }
    pub fn get_phase_available_tokens(&self) -> U128 {
        self.phase_available_tokens.into()
    }

    pub fn set_phase_available_token(&mut self, value: U128) {
        self.assert_owner();
        let value: u128 = value.into();
        assert!(
            value <= self.total_available_tokens,
            "Value must be lower than total_available tokens"
        );
        self.phase_available_tokens = value;
    }

    pub fn get_token_sold(&self) -> U128 {
        self.token_sold.into()
    }

    // Expire product, review, communication

    pub fn expire_product(&mut self, product_id: U128) {
        self.assert_owner();
        let product_id: u128 = product_id.into();
        let mut product = self.get_product(product_id);
        product.product_expired = true;
        self.product_map.insert(&product_id, &product);
    }

    pub fn expire_review(&mut self, review_id: U128) {
        self.assert_owner();
        let review_id: u128 = review_id.into();
        let mut review = self.get_review(review_id);
        review.review_expired = true;
        self.review_map.insert(&review_id, &review);
    }
    pub fn expire_communication(&mut self, communication_id: U128) {
        self.assert_owner();
        let communication_id: u128 = communication_id.into();
        let mut communication = self.get_communication(communication_id);
        communication.communication_expired = true;
        self.communication_map
            .insert(&communication_id, &communication);
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
    pub fn get_username(&self, user_id: U128) -> String {
        let user_id: u128 = user_id.into();
        let user_profile_option = self.user_profile_map.get(&user_id);
        let user = user_profile_option.unwrap();
        user.username
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

    pub fn create_communication(&mut self, communication_details_hash: String) {
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let com = Communication {
            user_id,
            communication_details_hash,
            communication_expired: false,
            communication_id: self.communication_id,
        };
        self.communication_map.insert(&self.communication_id, &com);
        self.communication_id += 1;
    }

    pub fn get_communication_js(&self, communication_id: U128) -> Communication {
        let communication_id: u128 = communication_id.into();
        self.get_communication(communication_id)
    }

    fn get_communication(&self, communication_id: u128) -> Communication {
        let communication_option = self.communication_map.get(&communication_id);
        match communication_option {
            Some(communication) => communication,
            None => {
                panic!("No communiction for the id");
            }
        }
    }

    pub fn update_communication(
        &mut self,
        communication_id: U128,
        communication_details_hash: String,
        communication_expired: bool,
    ) {
        let communication_id: u128 = communication_id.into();
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let mut communication = self.get_communication(communication_id);
        if user_id == communication.user_id {
            communication.communication_details_hash = communication_details_hash;
            communication.communication_expired = communication_expired;
        } else {
            panic!("You are not owner of communication");
        }
        self.communication_map
            .insert(&communication_id, &communication);
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
                    review_expired: false,
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
    fn _can_not_update_review_if_staked(&self, review_id: u128) {
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
        // self.can_not_update_review_if_staked(review_id);
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
        product_id: U128,
        start: usize,
        end: usize,
    ) -> Vec<U128> {
        let product_id: u128 = product_id.into();

        let product_commentproduct_option = self.product_commentproduct_map.get(&product_id);
        match product_commentproduct_option {
            Some(commentproduct_set) => commentproduct_set
                .iter()
                .skip(start)
                .take(end)
                .map(|x| x.into())
                .collect(),
            None => {
                panic!("No comments on product");
            }
        }
    }

    pub fn get_commentreview_by_review_id(
        &self,
        review_id: U128,
        start: usize,
        end: usize,
    ) -> Vec<U128> {
        let review_id: u128 = review_id.into();
        let review_commentreview_option = self.review_commentreview_map.get(&review_id);
        match review_commentreview_option {
            Some(commentreview_set) => commentreview_set
                .iter()
                .skip(start)
                .take(end)
                .map(|x| x.into())
                .collect(),
            None => {
                panic!("No comments on review");
            }
        }
    }

    pub fn add_product_crowdfunding(&mut self, bounty: U128, product_id: U128) {
        let product_id: u128 = product_id.into();
        let bounty: u128 = bounty.into();
        assert!(bounty > 0, "Bounty can not be zero");
        let account_id = env::predecessor_account_id();
        let product = self.get_product(product_id);
        let product_user_id = product.user_id;
        let product_user_profile = self.get_user_profile(product_user_id);
        let product_user_id_name = product_user_profile.username;
        assert!(
            account_id != product_user_id_name,
            "You can't fund your won product"
        );
        let product_bounty_crowdfunding_exists_option = self.product_crowdfunding.get(&product_id);
        match product_bounty_crowdfunding_exists_option {
            Some(bountyvalue) => {
                self.burn(&account_id, bounty);
                self.mint_myft(&product_user_id_name, bounty);
                let new_bounty_value = bounty.checked_add(bountyvalue).expect("overflow");
                self.product_crowdfunding
                    .insert(&product_id, &new_bounty_value);
            }
            None => {
                self.burn(&account_id, bounty);
                self.mint_myft(&product_user_id_name, bounty);
                self.product_crowdfunding.insert(&product_id, &bounty);
            }
        }
    }

    pub fn get_product_crowdfunding_js(&self, product_id: U128) -> U128 {
        let product_id: u128 = product_id.into();
        let bounty = self.get_product_crowdfunding(product_id);
        bounty.into()
    }

    fn get_product_crowdfunding(&self, product_id: u128) -> u128 {
        let bounty_option = self.product_crowdfunding.get(&product_id);
        match bounty_option {
            Some(bounty) => bounty,
            None => 0,
        }
    }

    pub fn add_product_bounty(&mut self, bounty: U64, product_id: U128) {
        let bounty: u64 = bounty.into();
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
    pub fn add_review_bounty(&mut self, bounty: U64, review_id: U128) {
        let bounty: u64 = bounty.into();
        let review_id: u128 = review_id.into();
        self.check_products_reviewer_crossed(review_id);
        // let review = self.get_review(review_id);
        // let product_review_id = review.product_id;
        // let _bounty = self.get_product_bounty(product_review_id);
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
        let contract_id = env::current_account_id();
        assert!(!env::state_exists(), "Already initialized");
        let metadata: NFTContractMetadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Example NEAR non-fungible token".to_string(),
            symbol: "EXAMPLE".to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        metadata.assert_valid();
        let total_supply_u128: u128 = total_supply.into();
        let admin_balance_x = (total_supply_u128 * 20).checked_div(100).expect("oveflow");
        let admin_balance = admin_balance_x.checked_div(10).expect("overflow");
        let token_sale_balance = (total_supply_u128 * 80).checked_div(100).expect("oveflow");
        let phase_sale_balance = token_sale_balance.checked_div(100).expect("overflow");
        let mut this = Self {
            ft: FungibleToken::new(b"a".to_vec(), 0),
            owner_id: owner_id.clone(),
            saving_id: owner_id.clone(),
            user_id: 0,
            product_id: 0,
            review_id: 0,
            communication_id: 0,
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
            communication_map: TreeMap::new(b"67ca9fd8".to_vec()),
            comment_product_map: LookupMap::new(b"fc337b34".to_vec()),
            comment_review_map: LookupMap::new(b"7859e655".to_vec()),
            user_products_map: TreeMap::new(b"e7b6e8a6".to_vec()),
            product_reviews_map: TreeMap::new(b"ea4ee217".to_vec()),
            product_commentproduct_map: LookupMap::new(b"fadfdeca".to_vec()),
            review_commentreview_map: LookupMap::new(b"00e72970".to_vec()),
            product_crowdfunding: LookupMap::new(b"b3556b34".to_vec()),
            product_bounty: LookupMap::new(b"0566cfb4".to_vec()),
            review_bounty: LookupMap::new(b"00423f89".to_vec()),
            min_review_bounty: 1000000000000000000, // 10^18
            min_product_bounty: 156250000000000000, //1.5625 × 10^17 , half of the first product incentives ie. (x/2)/2
            min_jury_stake: 1000000000000000000,    // 10^18
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
            jury_incentives: 33000000000000000,   // 3.3*10^16, 30 times less than 1 avrit
            review_incentives: 66000000000000000, // 6.6*10^16, 15 times less than 1 avrit
            product_oa_incentives: 625000000000000000, // 6.25 × 10^17 =>  x/2 + x/2 + 3 * x/5 = 1000000000000000000  (1 avrit)
            product_evidence_incentives: 625000000000000000, // 6.25 × 10^17 =>  x/2 + x/2 + 3 * x/5 = 1000000000000000000  (1 avrit)
            // Here is the gist of incentive
            // 30 times judge, 1 avrit
            // 15 times review, 1 avrit
            // 1 time product, 1 avrit
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
            token_price: 100000,
            token_sold: 0,
            total_available_tokens: token_sale_balance,
            phase_available_tokens: phase_sale_balance,
            on_crowdsale: true,
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                contract_id.clone().try_into().unwrap(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            nft_token_count: LookupMap::new(b"d0903ca3".to_vec()),
            nft_token_mint_count: LookupMap::new(b"a9ec8b8d".to_vec()),
            nft_owner_incentives: LookupMap::new(b"8f574fbd".to_vec()),
            nft_token_price: LookupMap::new(b"42d54eac".to_vec()),
        };
        this.ft.internal_register_account(&owner_id);
        this.ft.internal_deposit(&owner_id, admin_balance);
        this
    }
}
near_contract_standards::impl_fungible_token_core!(Avrit, ft);
near_contract_standards::impl_fungible_token_storage!(Avrit, ft);

const AVRIT_SVG: &str = "data:image/svg+xml,%3c%3fxml version='1.0' encoding='UTF-8' standalone='no'%3f%3e %3csvg xmlns:dc='http://purl.org/dc/elements/1.1/' xmlns:cc='http://creativecommons.org/ns%23' xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns%23' xmlns:svg='http://www.w3.org/2000/svg' xmlns='http://www.w3.org/2000/svg' xmlns:sodipodi='http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd' xmlns:inkscape='http://www.inkscape.org/namespaces/inkscape' inkscape:version='1.0 (4035a4fb49%2c 2020-05-01)' sodipodi:docname='splash_small.svg' viewBox='0 0 512 512' height='512' width='512' id='svg64' version='1.1'%3e %3cmetadata id='metadata70'%3e %3crdf:RDF%3e %3ccc:Work rdf:about=''%3e %3cdc:format%3eimage/svg%2bxml%3c/dc:format%3e %3cdc:type rdf:resource='http://purl.org/dc/dcmitype/StillImage' /%3e %3cdc:title%3e%3c/dc:title%3e %3c/cc:Work%3e %3c/rdf:RDF%3e %3c/metadata%3e %3cdefs id='defs68' /%3e %3csodipodi:namedview inkscape:document-rotation='0' inkscape:pagecheckerboard='true' inkscape:current-layer='g4889' inkscape:window-maximized='1' inkscape:window-y='27' inkscape:window-x='40' inkscape:cy='586.54655' inkscape:cx='446.38408' inkscape:zoom='0.24134228' showgrid='false' id='namedview66' inkscape:window-height='1016' inkscape:window-width='1880' inkscape:pageshadow='2' inkscape:pageopacity='0' guidetolerance='10' gridtolerance='10' objecttolerance='10' borderopacity='1' bordercolor='%23666666' pagecolor='white' /%3e %3cg transform='translate(0%2c-3)' style='display:inline' inkscape:label='Layer 1' id='layer2' inkscape:groupmode='layer' /%3e %3cg transform='translate(0%2c-3)' style='display:inline' inkscape:label='bulb' id='layer4' inkscape:groupmode='layer'%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' transform='matrix(1.2353856%2c0%2c0%2c1.0656391%2c296.4241%2c64.717261)' id='g4889'%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4831' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4833' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4835' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4837' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4839' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4841' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4843' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4845' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4847' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4849' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4851' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4853' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4855' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4857' /%3e %3cg style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' id='g4859' /%3e %3cg id='g903' transform='matrix(0.3486723%2c0%2c0%2c0.3486723%2c-112.61033%2c16.147529)'%3e %3cpath style='display:inline%3bfill:none%3bstroke:%23e90000%3bstroke-width:10%3bstroke-linecap:butt%3bstroke-linejoin:miter%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='M 853.11575%2c438.49715 Z' id='path892' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cg id='g4917' style='display:inline' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)'%3e %3cpath inkscape:connector-curvature='0' id='path890' d='m 479.6787%2c1053.4367 -3.06994%2c-117.42538 -17.65218%2c-22.2571 -2.30246%2c-69.07373 -11.44112%2c-10.15203 -0.0678%2c-9.63283 -22.38619%2c-47.21442 -1.62808%2c-77.60531 -9.15797%2c-37.85294 -107.66479%2c-224.69375 c 0%2c0 -104.5966%2c-307.28253 179.77532%2c-387.601316 284.37192%2c-80.318786 449.35105%2c175.833016 369.03226%2c388.569256' style='fill:none%3bstroke:%23e90000%3bstroke-width:10%3bstroke-linecap:butt%3bstroke-linejoin:miter%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' /%3e %3cpath inkscape:connector-curvature='0' id='path894' d='m 853.11575%2c438.49715 -112.88046%2c251.81025 1e-5%2c79.2334 c 0%2c0 -1.89944%2c32.83301 -21.97913%2c54.26945 l -1.35674%2c12.75332 -8.41176%2c12.48197 -1.08539%2c63.49526 -20.89374%2c23.60721 -1.08539%2c119.39279' style='fill:none%3bstroke:%23e90000%3bstroke-width:10%3bstroke-linecap:butt%3bstroke-linejoin:miter%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' /%3e %3cpath sodipodi:nodetypes='cc' inkscape:connector-curvature='0' id='path898' d='m 685.42315%2c1055.5408 c -66.40573%2c35.6948 -153.27535%2c49.3817 -205.74445%2c-2.1041' style='fill:none%3bstroke:%23e90000%3bstroke-width:10%3bstroke-linecap:butt%3bstroke-linejoin:miter%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' /%3e %3c/g%3e %3cpath style='display:inline%3bfill:none%3bstroke:%23e90000%3bstroke-width:10%3bstroke-linecap:butt%3bstroke-linejoin:miter%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='M 488.50479%2c1062.6465 H 673.85264' id='path900' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='display:inline%3bfill:none%3bstroke:%23e90000%3bstroke-width:10%3bstroke-linecap:butt%3bstroke-linejoin:miter%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 476.60876%2c936.01132 209.89978%2c0.13669' id='path904' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='display:inline%3bfill:none%3bstroke:%23e90000%3bstroke-width:10%3bstroke-linecap:butt%3bstroke-linejoin:miter%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 448.80835%2c839.00569 268.09108%2c-2.44212' id='path906' inkscape:connector-curvature='0' sodipodi:nodetypes='cc' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='display:inline%3bfill:none%3bstroke:%23e90000%3bstroke-width:10%3bstroke-linecap:butt%3bstroke-linejoin:miter%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 456.65412%2c844.68049 257.53184%2c-2.41863' id='path908' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='display:inline%3bfill:none%3bstroke:%23e90000%3bstroke-width:10%3bstroke-linecap:butt%3bstroke-linejoin:miter%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 458.95658%2c913.75422 248.4457%2c-1.21342' id='path910' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cg id='g4829' style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none'%3e %3cpath id='path4819' d='M 151.245%2c222.446 C 148.054%2c237.039 135.036%2c248 119.5%2c248 c -4.142%2c0 -7.5%2c3.357 -7.5%2c7.5 0%2c4.143 3.358%2c7.5 7.5%2c7.5 23.774%2c0 43.522%2c-17.557 46.966%2c-40.386 14.556%2c-1.574 27.993%2c-8.06 38.395%2c-18.677 2.899%2c-2.959 2.85%2c-7.708 -0.109%2c-10.606 -2.958%2c-2.897 -7.707%2c-2.851 -10.606%2c0.108 C 184.947%2c202.829 172.643%2c208 159.5%2c208 132.757%2c208 111%2c186.243 111%2c159.5 c 0%2c-4.143 -3.358%2c-7.5 -7.5%2c-7.5 -4.142%2c0 -7.5%2c3.357 -7.5%2c7.5 0%2c32.215 24.119%2c58.884 55.245%2c62.946 z' inkscape:connector-curvature='0' style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' /%3e %3cpath id='path4821' d='m 183%2c287.5 c 0%2c-4.143 -3.358%2c-7.5 -7.5%2c-7.5 -35.014%2c0 -63.5%2c28.486 -63.5%2c63.5 0%2c0.362 0.013%2c0.725 0.019%2c1.088 C 109.23%2c344.212 106.39%2c344 103.5%2c344 c -4.142%2c0 -7.5%2c3.357 -7.5%2c7.5 0%2c4.143 3.358%2c7.5 7.5%2c7.5 26.743%2c0 48.5%2c21.757 48.5%2c48.5 0%2c4.143 3.358%2c7.5 7.5%2c7.5 4.142%2c0 7.5%2c-3.357 7.5%2c-7.5 0%2c-26.611 -16.462%2c-49.437 -39.731%2c-58.867 -0.178%2c-1.699 -0.269%2c-3.418 -0.269%2c-5.133 0%2c-26.743 21.757%2c-48.5 48.5%2c-48.5 4.142%2c0 7.5%2c-3.357 7.5%2c-7.5 z' inkscape:connector-curvature='0' style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' /%3e %3cpath id='path4823' d='m 439%2c223.5 c 0%2c-17.075 -6.82%2c-33.256 -18.875%2c-45.156 C 422.034%2c172.236 423%2c165.918 423%2c159.5 423%2c128.626 400.848%2c102.841 371.606%2c97.171 373.841%2c91.6 375%2c85.628 375%2c79.5 375%2c59.943 363.117%2c43.113 346.194%2c35.839 317.999%2c13.383 287.162%2c0 263.5%2c0 250.347%2c0 238.683%2c6.468 231.5%2c16.384 224.317%2c6.468 212.653%2c0 199.5%2c0 175.838%2c0 145.001%2c13.383 116.806%2c35.839 99.883%2c43.113 88%2c59.943 88%2c79.5 88%2c85.628 89.159%2c91.6 91.394%2c97.171 62.152%2c102.841 40%2c128.626 40%2c159.5 c 0%2c6.418 0.965%2c12.735 2.875%2c18.844 C 30.82%2c190.244 24%2c206.425 24%2c223.5 c 0%2c13.348 4.149%2c25.741 11.213%2c35.975 C 27.872%2c270.087 24%2c282.466 24%2c295.5 c 0%2c23.088 12.587%2c44.242 32.516%2c55.396 -0.343%2c2.852 -0.516%2c5.73 -0.516%2c8.604 0%2c31.144 20.315%2c58.679 49.79%2c68.063 12.821%2c21.942 36.175%2c35.437 61.71%2c35.437 27.995%2c0 52.269%2c-16.181 64%2c-39.674 11.731%2c23.493 36.005%2c39.674 64%2c39.674 25.535%2c0 48.889%2c-13.495 61.71%2c-35.437 29.475%2c-9.385 49.79%2c-36.92 49.79%2c-68.063 0%2c-2.874 -0.173%2c-5.752 -0.516%2c-8.604 C 426.413%2c339.742 439%2c318.588 439%2c295.5 439%2c282.466 435.128%2c270.087 427.787%2c259.475 434.851%2c249.241 439%2c236.848 439%2c223.5 Z M 167.5%2c448 c -21.029%2c0 -40.191%2c-11.594 -50.009%2c-30.256 -0.973%2c-1.849 -2.671%2c-3.208 -4.688%2c-3.751 C 88.19%2c407.369 71%2c384.961 71%2c359.5 c 0%2c-3.81 0.384%2c-7.626 1.141%2c-11.344 0.702%2c-3.447 -1.087%2c-6.92 -4.302%2c-8.35 C 50.32%2c332.018 39%2c314.626 39%2c295.5 39%2c286.801 41.256%2c278.486 45.561%2c271.121 56.757%2c280.992 71.436%2c287 87.5%2c287 91.642%2c287 95%2c283.643 95%2c279.5 95%2c275.357 91.642%2c272 87.5%2c272 60.757%2c272 39%2c250.243 39%2c223.5 c 0%2c-14.396 6.352%2c-27.964 17.428%2c-37.221 2.5%2c-2.09 3.365%2c-5.555 2.14%2c-8.574 C 56.2%2c171.869 55%2c165.744 55%2c159.5 55%2c132.757 76.757%2c111 103.5%2c111 c 26.743%2c0 48.5%2c21.757 48.5%2c48.5 0%2c4.143 3.358%2c7.5 7.5%2c7.5 4.142%2c0 7.5%2c-3.357 7.5%2c-7.5 C 167%2c125.858 140.698%2c98.257 107.579%2c96.145 104.577%2c91.127 103%2c85.421 103%2c79.5 c 0%2c-13.369 8.116%2c-24.875 19.678%2c-29.859 0.447%2c-0.133 0.885%2c-0.307 1.308%2c-0.527 C 127.568%2c47.752 131.447%2c47 135.5%2c47 c 12.557%2c0 23.767%2c7.021 29.256%2c18.325 1.81%2c3.727 6.298%2c5.281 10.023%2c3.47 3.726%2c-1.809 5.28%2c-6.296 3.47%2c-10.022 C 171.983%2c45.87 160.124%2c36.596 146.467%2c33.311 165.609%2c21.631 184.454%2c15 199.5%2c15 213.009%2c15 224%2c25.99 224%2c39.5 v 97.051 C 217.261%2c131.205 208.75%2c128 199.5%2c128 c -4.142%2c0 -7.5%2c3.357 -7.5%2c7.5 0%2c4.143 3.358%2c7.5 7.5%2c7.5 13.509%2c0 24.5%2c10.99 24.5%2c24.5 v 180.279 c -9.325%2c-12.031 -22.471%2c-21.111 -37.935%2c-25.266 -3.999%2c-1.071 -8.114%2c1.297 -9.189%2c5.297 -1.075%2c4.001 1.297%2c8.115 5.297%2c9.189 C 206.8%2c343.616 224%2c366.027 224%2c391.5 224%2c422.654 198.654%2c448 167.5%2c448 Z M 395.161%2c339.807 c -3.215%2c1.43 -5.004%2c4.902 -4.302%2c8.35 0.757%2c3.718 1.141%2c7.534 1.141%2c11.344 0%2c25.461 -17.19%2c47.869 -41.803%2c54.493 -2.017%2c0.543 -3.716%2c1.902 -4.688%2c3.751 C 335.691%2c436.406 316.529%2c448 295.5%2c448 264.346%2c448 239%2c422.654 239%2c391.5 c 0%2c-2.109 -0.098%2c-4.2 -0.281%2c-6.271 0.178%2c-0.641 0.281%2c-1.314 0.281%2c-2.012 V 135.5 c 0%2c-13.51 10.991%2c-24.5 24.5%2c-24.5 4.142%2c0 7.5%2c-3.357 7.5%2c-7.5 0%2c-4.143 -3.358%2c-7.5 -7.5%2c-7.5 -9.25%2c0 -17.761%2c3.205 -24.5%2c8.551 V 39.5 C 239%2c25.99 249.991%2c15 263.5%2c15 c 15.046%2c0 33.891%2c6.631 53.033%2c18.311 -13.657%2c3.284 -25.516%2c12.559 -31.782%2c25.462 -1.81%2c3.727 -0.256%2c8.214 3.47%2c10.022 3.726%2c1.81 8.213%2c0.257 10.023%2c-3.47 C 303.733%2c54.021 314.943%2c47 327.5%2c47 c 4.053%2c0 7.933%2c0.752 11.514%2c2.114 0.422%2c0.22 0.86%2c0.393 1.305%2c0.526 C 351.883%2c54.624 360%2c66.13 360%2c79.5 c 0%2c5.921 -1.577%2c11.627 -4.579%2c16.645 C 322.302%2c98.257 296%2c125.858 296%2c159.5 c 0%2c4.143 3.358%2c7.5 7.5%2c7.5 4.142%2c0 7.5%2c-3.357 7.5%2c-7.5 0%2c-26.743 21.757%2c-48.5 48.5%2c-48.5 26.743%2c0 48.5%2c21.757 48.5%2c48.5 0%2c6.244 -1.2%2c12.369 -3.567%2c18.205 -1.225%2c3.02 -0.36%2c6.484 2.14%2c8.574 11.075%2c9.257 17.427%2c22.825 17.427%2c37.221 0%2c26.743 -21.757%2c48.5 -48.5%2c48.5 -4.142%2c0 -7.5%2c3.357 -7.5%2c7.5 0%2c4.143 3.358%2c7.5 7.5%2c7.5 16.064%2c0 30.743%2c-6.008 41.939%2c-15.879 4.306%2c7.365 6.561%2c15.68 6.561%2c24.379 0%2c19.126 -11.32%2c36.518 -28.839%2c44.307 z' inkscape:connector-curvature='0' style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' /%3e %3cpath id='path4825' d='M 359.5%2c240 C 343.964%2c240 330.946%2c229.039 327.755%2c214.446 358.881%2c210.384 383%2c183.715 383%2c151.5 c 0%2c-4.143 -3.358%2c-7.5 -7.5%2c-7.5 -4.142%2c0 -7.5%2c3.357 -7.5%2c7.5 0%2c26.743 -21.757%2c48.5 -48.5%2c48.5 -13.143%2c0 -25.447%2c-5.171 -34.646%2c-14.561 -2.898%2c-2.958 -7.647%2c-3.007 -10.606%2c-0.108 -2.959%2c2.899 -3.008%2c7.647 -0.109%2c10.606 10.402%2c10.617 23.839%2c17.103 38.395%2c18.677 3.444%2c22.829 23.192%2c40.386 46.966%2c40.386 4.142%2c0 7.5%2c-3.357 7.5%2c-7.5 0%2c-4.143 -3.358%2c-7.5 -7.5%2c-7.5 z' inkscape:connector-curvature='0' style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' /%3e %3cpath id='path4827' d='m 335.5%2c328 c -2.89%2c0 -5.73%2c0.212 -8.519%2c0.588 0.006%2c-0.363 0.019%2c-0.726 0.019%2c-1.088 0%2c-35.014 -28.486%2c-63.5 -63.5%2c-63.5 -4.142%2c0 -7.5%2c3.357 -7.5%2c7.5 0%2c4.143 3.358%2c7.5 7.5%2c7.5 26.743%2c0 48.5%2c21.757 48.5%2c48.5 0%2c1.714 -0.091%2c3.434 -0.269%2c5.133 C 288.462%2c342.063 272%2c364.889 272%2c391.5 c 0%2c4.143 3.358%2c7.5 7.5%2c7.5 4.142%2c0 7.5%2c-3.357 7.5%2c-7.5 0%2c-26.743 21.757%2c-48.5 48.5%2c-48.5 4.142%2c0 7.5%2c-3.357 7.5%2c-7.5 0%2c-4.143 -3.358%2c-7.5 -7.5%2c-7.5 z' inkscape:connector-curvature='0' style='stroke-width:0.174311%3bstroke-miterlimit:4%3bstroke-dasharray:none' /%3e %3c/g%3e %3cpath style='opacity:1%3bfill:%23ffff00%3bfill-opacity:0.941176%3bstroke:%23fe0700%3bstroke-width:0.306994%3bstroke-linejoin:round%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 451.1187%2c832.5224 c -0.54998%2c-0.54998 -0.99996%2c-3.19803 -0.99996%2c-5.88455 0%2c-2.88034 -4.40331%2c-14.10739 -10.73138%2c-27.36164 l -10.73138%2c-22.47704 -0.87216%2c-38.00258 c -0.81169%2c-35.36744 -1.19853%2c-39.38628 -5.57888%2c-57.95722 -4.58925%2c-19.4566 -6.06346%2c-22.78961 -59.06689%2c-133.54255 -56.95472%2c-119.00936 -57.49659%2c-120.31998 -64.25076%2c-155.401 -7.01672%2c-36.44467 -9.06643%2c-91.67013 -4.63721%2c-124.94067 C 307.842%2c164.85798 367.39535%2c94.464606 468.5384%2c60.942502 c 87.87554%2c-29.124816 172.20745%2c-26.753993 242.50445%2c6.81753 32.77263%2c15.651124 67.99038%2c43.365448 90.9833%2c71.598608 61.99046%2c76.11843 81.87104%2c183.87304 51.58273%2c279.58323 -2.57023%2c8.12186 -30.51376%2c72.46459 -62.09675%2c142.98386 l -57.4236%2c128.21682 v 42.71098 c 0%2c33.58477 -0.50781%2c44.67066 -2.37657%2c51.8824 -3.19496%2c12.32981 -8.47579%2c23.77809 -14.3034%2c31.00827 -2.77575%2c3.44382 -4.80963%2c7.75143 -4.80963%2c10.18643 v 4.21921 l -41.82799%2c0.90336 c -71.15822%2c1.5368 -218.59845%2c2.52299 -219.65224%2c1.4692 z M 524.43047%2c556.508 c 20.57721%2c-4.35926 41.27159%2c-16.77903 52.5074%2c-31.51238 l 5.66688%2c-7.43092 4.09781%2c5.65688 c 6.86446%2c9.4761 19.83873%2c20.05474 31.33088%2c25.5458 37.73392%2c18.02961 83.51056%2c10.74259 111.62986%2c-17.76997 7.2439%2c-7.34521 12.68023%2c-11.37879 18.1235%2c-13.44707 17.98017%2c-6.83193 35.34935%2c-21.39866 43.43876%2c-36.43011 4.89356%2c-9.09305 8.81193%2c-24.45168 8.84407%2c-34.66577 0.0221%2c-7.02779 0.23504%2c-7.35 8.46728%2c-12.81425 29.55906%2c-19.62018 38.98612%2c-53.91572 22.89984%2c-83.3094 l -5.30819%2c-9.69943 3.1402%2c-4.41002 c 15.20194%2c-21.34915 12.55071%2c-53.96161 -6.0251%2c-74.11399 l -6.28817%2c-6.82186 1.79982%2c-11.57687 c 1.53713%2c-9.88724 1.4229%2c-13.10425 -0.78273%2c-22.04207 -6.07832%2c-24.63103 -24.17234%2c-42.61011 -51.64964%2c-51.32154 -6.60198%2c-2.0931 -8.34457%2c-3.29235 -7.99381%2c-5.50136 2.3186%2c-14.60199 1.99735%2c-23.20575 -1.18073%2c-31.62323 -3.91492%2c-10.36898 -14.61151%2c-22.15803 -24.72009%2c-27.24478 C 728.69814%2c104.0986 719.96719%2c99.155528 713.0262%2c94.991053 695.11712%2c84.245915 674.16627%2c74.625878 658.10743%2c69.773909 646.44075%2c66.248975 641.30926%2c65.577861 625.10553%2c65.457812 604.49548%2c65.305113 600.1139%2c66.567615 587.241%2c76.367995 l -4.86262%2c3.702014 -7.09834%2c-5.418277 c -9.10906%2c-6.953103 -19.91617%2c-10.110349 -34.49281%2c-10.076906 -24.96529%2c0.05727 -57.44527%2c10.945362 -86.83106%2c29.107909 -5.90964%2c3.652586 -14.11373%2c8.305925 -18.2313%2c10.340745 -10.7133%2c5.2943 -23.00869%2c17.50506 -27.17325%2c26.98622 -3.32396%2c7.56741 -4.29773%2c19.80328 -2.62517%2c32.98631 0.38906%2c3.06652 -0.82023%2c3.94286 -9.53172%2c6.90737 -21.0676%2c7.16929 -35.99778%2c19.63273 -44.70441%2c37.3184 -4.05538%2c8.23766 -5.1026%2c12.44474 -5.6383%2c22.65117 -0.36184%2c6.89397 0.0374%2c15.05256 0.88715%2c18.13018 1.4512%2c5.25583 1.2227%2c5.90734 -3.76225%2c10.72727 -2.919%2c2.82239 -7.85341%2c10.2763 -10.96535%2c16.56428 -5.39754%2c10.90627 -5.65806%2c12.14961 -5.65806%2c27.00318 0%2c15.01361 0.21047%2c15.98515 5.88462%2c27.16412 l 5.88462%2c11.59363 -3.85638%2c5.91738 c -10.34867%2c15.87937 -11.4719%2c40.19673 -2.68147%2c58.05263 5.50613%2c11.18454 16.51573%2c23.15344 26.89032%2c29.23336 l 6.75414%2c3.95818 0.0761%2c9.91801 c 0.14045%2c18.30045 5.78009%2c31.78777 19.47883%2c46.58404 9.07242%2c9.79931 21.05045%2c17.86075 33.34077%2c22.43899 5.03977%2c1.87735 10.82615%2c6.20907 18.41966%2c13.78907 22.18853%2c22.14907 55.29603%2c31.42245 87.68573%2c24.56073 z' id='path4961' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='opacity:1%3bfill:lime%3bfill-opacity:0.941176%3bstroke:%23fe0700%3bstroke-width:0.306994%3bstroke-linejoin:round%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 648.941%2c540.36726 c -26.85192%2c-4.77005 -48.13008%2c-22.34096 -54.75574%2c-45.21579 -0.74056%2c-2.55674 -1.33339%2c-69.50579 -1.3174%2c-148.77567 0.0287%2c-142.41039 0.0664%2c-144.18745 3.16348%2c-149.19863 4.22488%2c-6.83601 11.74779%2c-11.20545 22.22969%2c-12.91143 4.78591%2c-0.77892 9.66527%2c-2.21596 10.84302%2c-3.1934 3.61867%2c-3.00323 2.62763%2c-10.22173 -1.80182%2c-13.12402 -3.47583%2c-2.27744 -5.14083%2c-2.363 -14.04763%2c-0.72186 -5.55744%2c1.024 -12.03755%2c2.86146 -14.40024%2c4.08326 -2.3627%2c1.2218 -4.67548%2c2.22145 -5.13953%2c2.22145 -0.46404%2c0 -0.84371%2c-16.26663 -0.84371%2c-36.14806 0%2c-38.544431 0.3664%2c-40.752823 7.93371%2c-47.818384 8.79719%2c-8.213897 20.52784%2c-9.889073 40.44326%2c-5.77543 15.10095%2c3.119185 42.18637%2c13.383665 42.18637%2c15.987237 0%2c0.489717 -3.40631%2c2.100207 -7.56957%2c3.578867 -9.29334%2c3.3007 -21.55948%2c12.78097 -26.15267%2c20.21292 -4.09858%2c6.63163 -3.24881%2c12.42995 2.18447%2c14.90552 5.51089%2c2.51092 10.65494%2c0.48445 15.16748%2c-5.97514 6.89511%2c-9.87019 21.48784%2c-17.29646 33.98782%2c-17.29646 21.56455%2c0 40.92338%2c18.74267 38.80165%2c37.56665 -1.11441%2c9.887 -4.64248%2c14.6224 -10.8943%2c14.6224 -2.78213%2c0 -8.81015%2c1.10198 -13.39562%2c2.44886 -29.34506%2c8.61944 -51.17135%2c32.43277 -53.22971%2c58.07571 -0.96589%2c12.03295 0.62908%2c14.68905 8.82068%2c14.68905 7.28624%2c0 9.45232%2c-2.60534 10.87358%2c-13.07866 1.56894%2c-11.56161 6.89602%2c-21.32353 16.35259%2c-29.96631 12.13795%2c-11.0934 21.93106%2c-14.41931 42.61504%2c-14.47281 15.80354%2c-0.0409 17.53842%2c0.26606 27.09418%2c4.79341 23.63354%2c11.19718 36.12505%2c34.68681 29.87647%2c56.18118 -3.72715%2c12.82098 -3.34135%2c14.48616 5.11841%2c22.09222 10.55189%2c9.48705 15.84765%2c20.77961 15.95258%2c34.01682 0.20859%2c26.31709 -20.85279%2c47.00974 -51.84719%2c50.93945 -13.66622%2c1.73271 -16.21713%2c3.19925 -16.21713%2c9.32341 0%2c5.14943 4.84717%2c9.10892 11.15105%2c9.10892 12.7842%2c0 40.10409%2c-8.84204 48.27663%2c-15.62465 3.36653%2c-2.79397 8.67125%2c11.41814 8.69663%2c23.29951 0.0399%2c18.65454 -10.69816%2c34.47214 -30.56261%2c45.02014 l -9.16472%2c4.86644 v 15.34972 c 0%2c13.37725 -0.51977%2c16.45825 -4.04491%2c23.97628 -7.50922%2c16.01493 -21.40059%2c28.17138 -39.36491%2c34.44862 -8.46005%2c2.95615 -11.34921%2c5.01374 -19.10237%2c13.60415 -5.02412%2c5.56666 -12.06467%2c11.83825 -15.64566%2c13.93685 -14.95348%2c8.76332 -36.18889%2c12.8046 -52.27135%2c9.94766 z m 0.14613%2c-52.3815 c 0.94848%2c-0.75565 2.20023%2c-4.96003 2.78167%2c-9.34306 1.63549%2c-12.32899 6.45721%2c-21.71227 15.60081%2c-30.3599 10.20835%2c-9.65464 22.36194%2c-15.21853 35.75676%2c-16.36939 12.48381%2c-1.07259 17.04741%2c-3.63295 17.04741%2c-9.56424 0%2c-5.92055 -3.59629%2c-8.4953 -12.08762%2c-8.65409 l -7.09952%2c-0.13276 -1.09236%2c-8.70073 c -0.6008%2c-4.7854 -2.96877%2c-12.68568 -5.26216%2c-17.55619 -5.02321%2c-10.66789 -19.5403%2c-25.55773 -30.75416%2c-31.54384 -15.61666%2c-8.33639 -42.4379%2c-12.88936 -48.74824%2c-8.27511 -3.04225%2c2.22454 -3.74948%2c9.50982 -1.17327%2c12.08603 0.88794%2c0.88793 7.6001%2c2.57293 14.91591%2c3.74442 7.31581%2c1.17149 17.01376%2c3.88618 21.55101%2c6.03264 17.06431%2c8.07271 29.80153%2c25.72803 29.82048%2c41.33478 l 0.009%2c7.67486 -10.35539%2c5.30245 c -12.82075%2c6.56483 -26.26281%2c19.8777 -31.72169%2c31.41684 -5.28408%2c11.16961 -7.86137%2c24.70539 -5.63462%2c29.59257 2.72169%2c5.97346 11.01119%2c7.64427 16.44567%2c3.31472 z m 99.19989%2c-155.05303 c 3.35711%2c-3.96741 3.34731%2c-4.74783 -0.11313%2c-9.0213 -2.05996%2c-2.54396 -5.31028%2c-3.80044 -12.30601%2c-4.75717 -12.04137%2c-1.64675 -19.9629%2c-5.686 -26.7877%2c-13.65924 -8.58126%2c-10.02529 -8.3339%2c-11.39299 2.46387%2c-13.62324 29.28657%2c-6.04904 53.36004%2c-29.57688 57.65524%2c-56.34848 1.87253%2c-11.6713 -0.11067%2c-16.51468 -7.03726%2c-17.18616 -8.02112%2c-0.7776 -10.76046%2c1.70198 -11.94452%2c10.81185 -2.37324%2c18.259 -13.67311%2c33.2308 -31.4787%2c41.70776 -9.36035%2c4.45632 -11.06886%2c4.75005 -27.62949%2c4.75005 -16.02538%2c0 -18.47795%2c-0.38686 -26.61258%2c-4.19775 -4.92823%2c-2.30875 -11.02518%2c-5.93512 -13.54878%2c-8.0586 -2.61171%2c-2.1976 -6.52142%2c-3.86085 -9.07551%2c-3.86085 -4.88074%2c0 -10.62703%2c4.66052 -10.62703%2c8.61906 0%2c6.94953 23.22671%2c20.64751 40.5982%2c23.94285 5.53073%2c1.04916 10.06262%2c2.61347 10.07086%2c3.47623 0.0403%2c4.21385 8.10723%2c18.13324 13.81976%2c23.84576 10.7522%2c10.7522 33.92936%2c19.51546 46.02915%2c17.40357 2.11059%2c-0.36839 5.04622%2c-2.09834 6.52363%2c-3.84434 z' id='path4963' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='opacity:1%3bfill:%23ff00ff%3bfill-opacity:0.941176%3bstroke:%23fe0700%3bstroke-width:0.306994%3bstroke-linejoin:round%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 490.56016%2c540.31213 c -18.76792%2c-3.29722 -34.88411%2c-12.76363 -45.04633%2c-26.45957 -4.1857%2c-5.64119 -7.0677%2c-7.79474 -13.04726%2c-9.74941 -22.59301%2c-7.38548 -36.96504%2c-19.96866 -44.35514%2c-38.83439 -1.85785%2c-4.74278 -2.64949%2c-11.00881 -2.60398%2c-20.61129 0.0376%2c-7.92971 -0.65721%2c-14.66752 -1.63383%2c-15.84427 -0.93456%2c-1.12609 -4.69363%2c-3.372 -8.35349%2c-4.99093 -10.17676%2c-4.50166 -21.0722%2c-15.60904 -25.9306%2c-26.43501 -3.69273%2c-8.22849 -4.13019%2c-10.7819 -3.48027%2c-20.31429 0.40884%2c-5.99655 1.97106%2c-13.42703 3.47159%2c-16.51216 l 2.72825%2c-5.6093 9.3793%2c5.10947 c 15.8367%2c8.62722 38.60518%2c13.41808 47.08208%2c9.90683 5.87458%2c-2.43333 7.55629%2c-8.3399 3.5229%2c-12.37328 -1.99129%2c-1.99129 -6.64909%2c-3.39624 -14.60289%2c-4.40467 -22.70644%2c-2.8789 -40.05289%2c-14.6177 -48.10096%2c-32.55119 -3.67941%2c-8.19881 -4.13185%2c-10.8163 -3.51142%2c-20.3143 0.8864%2c-13.56953 4.67015%2c-21.30479 15.84329%2c-32.38899 4.60834%2c-4.57167 8.37881%2c-9.29393 8.37881%2c-10.49394 0%2c-1.20001 -1.03611%2c-4.66159 -2.30246%2c-7.69241 -3.04505%2c-7.28781 -2.92334%2c-21.65124 0.26307%2c-31.04649 3.6454%2c-10.74861 18.19609%2c-25.04876 30.43636%2c-29.9123 7.70882%2c-3.06302 11.96696%2c-3.66655 26.1268%2c-3.70311 16.03412%2c-0.0414 17.44689%2c0.21721 27.07126%2c4.95527 18.0739%2c8.89776 29.10383%2c23.34686 31.43128%2c41.17469 1.26912%2c9.72127 2.72145%2c11.38782 9.924%2c11.38782 4.61096%2c0 6.46921%2c-0.84349 8.42106%2c-3.82236 2.14089%2c-3.26742 2.26014%2c-5.20963 0.82138%2c-13.37759 -3.72048%2c-21.12153 -18.24981%2c-39.73498 -38.64652%2c-49.50984 -9.60083%2c-4.60107 -15.38985%2c-6.32169 -31.56931%2c-9.38304 -5.75377%2c-1.08869 -9.48641%2c-14.55982 -6.74617%2c-24.347 8.12567%2c-29.02203 51.13958%2c-35.67645 71.3312%2c-11.03522 6.76825%2c8.25975 11.48531%2c10.05641 16.9691%2c6.4633 6.221%2c-4.07616 5.11233%2c-9.99152 -3.67449%2c-19.60572 -6.93315%2c-7.58596 -13.82664%2c-12.09514 -25.50116%2c-16.68084 l -4.60492%2c-1.808789 9.20983%2c-4.179803 c 38.75737%2c-17.589701 67.61884%2c-17.991329 78.80429%2c-1.096616 l 4.08419%2c6.168848 0.43925%2c54.07385 0.43926%2c54.07385 -8.88161%2c-3.65221 c -10.66206%2c-4.38435 -24.99988%2c-5.03967 -28.72658%2c-1.31296 -3.09258%2c3.09257 -3.0978%2c8.92269 -0.0107%2c11.4851 1.32099%2c1.09632 6.21738%2c2.5111 10.88086%2c3.14395 11.0547%2c1.50016 19.85607%2c7.33655 23.64345%2c15.67852 2.76419%2c6.08834 2.91749%2c11.44295 2.94945%2c103.02877 l 0.0338%2c96.60666 -4.22117%2c-3.96001 c -7.48042%2c-7.01762 -21.33347%2c-14.87448 -33.32235%2c-18.89901 -10.20603%2c-3.42605 -12.21603%2c-3.6716 -15.85293%2c-1.93662 -2.79813%2c1.33482 -4.32733%2c3.39891 -4.65965%2c6.28953 -0.65161%2c5.6679 1.78955%2c8.33698 9.49956%2c10.38654 10.03744%2c2.66825 23.07322%2c10.26548 30.95676%2c18.0415 21.99611%2c21.69616 22.51481%2c52.81819 1.25901%2c75.54235 -4.04012%2c4.31922 -10.32979%2c9.60193 -13.97707%2c11.73938 -14.08699%2c8.25552 -36.32069%2c12.35405 -52.03848%2c9.5927 z m 9.62533%2c-35.76014 c 4.16887%2c-3.58591 4.18055%2c-3.6526 2.45999%2c-14.03586 -0.95083%2c-5.73804 -3.58746%2c-14.23184 -5.85919%2c-18.8751 -5.50126%2c-11.24417 -19.04173%2c-24.49826 -31.73179%2c-31.06065 l -10.31949%2c-5.33651 0.009%2c-7.67486 c 0.0278%2c-22.89117 22.75193%2c-43.39017 52.44608%2c-47.31058 9.88654%2c-1.30528 12.74567%2c-2.28391 14.50151%2c-4.96367 3.07452%2c-4.69231 0.4709%2c-11.00834 -5.11595%2c-12.41055 -6.31155%2c-1.58409 -26.16755%2c1.7646 -37.59945%2c6.34111 -24.37182%2c9.75668 -44.20754%2c35.12247 -44.20754%2c56.53226 v 5.5936 l -7.66009%2c-0.17257 c -8.95605%2c-0.20178 -12.29454%2c1.98556 -12.29454%2c8.05525 0%2c5.82339 4.54611%2c8.5017 16.1172%2c9.49538 14.099%2c1.21076 25.67374%2c6.29862 35.92899%2c15.79318 9.87379%2c9.1414 14.75942%2c18.40816 16.65707%2c31.59415 0.93374%2c6.4882 1.93974%2c8.5158 4.99113%2c10.05964 5.33965%2c2.70158 6.9%2c2.48454 11.67671%2c-1.62422 z M 469.23083%2c340.79801 c 12.38248%2c-4.71238 27.04217%2c-18.99256 30.93287%2c-30.1321 l 2.76627%2c-7.9201 9.13452%2c-1.7971 c 16.13977%2c-3.1753 38.07833%2c-15.04062 40.2063%2c-21.74529 1.43467%2c-4.52023 -0.60149%2c-8.49616 -5.36061%2c-10.46745 -4.85016%2c-2.00901 -5.97673%2c-1.6234 -17.37147%2c5.94609 -11.60203%2c7.70715 -20.40399%2c10.14056 -36.44076%2c10.07449 -17.49001%2c-0.0721 -28.2602%2c-3.70157 -40.43679%2c-13.62705 -9.5568%2c-7.79 -17.89214%2c-22.60381 -17.89214%2c-31.79843 0%2c-4.40394 -1.03661%2c-7.11215 -3.76765%2c-9.8432 -4.40229%2c-4.40228 -7.96909%2c-4.7447 -12.96597%2c-1.24474 -3.3098%2c2.31827 -3.5429%2c3.34544 -2.87375%2c12.66351 1.92505%2c26.80686 25.57431%2c51.48149 56.87879%2c59.34492 11.3541%2c2.85207 11.71102%2c4.09908 3.88925%2c13.58824 -5.84518%2c7.09122 -16.84722%2c12.72071 -27.51921%2c14.08094 -4.54798%2c0.57969 -9.47785%2c1.67644 -10.95526%2c2.43725 -3.46836%2c1.78607 -3.52122%2c9.4308 -0.0889%2c12.86309 3.51378%2c3.51378 19.4486%2c2.30206 31.86455%2c-2.42307 z' id='path4965' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='opacity:1%3bfill:%23ffccaa%3bfill-opacity:0.941176%3bstroke:%23fe0700%3bstroke-width:0.153497%3bstroke-linejoin:round%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 463.81639%2c897.99976 c -0.26537%2c-5.74293 -0.71183%2c-18.78791 -0.99212%2c-28.98883 l -0.50961%2c-18.54714 3.37763%2c-0.40131 c 2.89395%2c-0.34384 197.86771%2c-2.10477 226.90791%2c-2.04935 l 10.16919%2c0.0194 v 19.0658 c 0%2c10.48619 -0.22425%2c23.7415 -0.49833%2c29.45625 l -0.49831%2c10.39045 -62.24366%2c0.4669 c -34.23401%2c0.25679 -87.66563%2c0.59348 -118.73693%2c0.7482 l -56.49327%2c0.2813 z' id='path4967' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='opacity:1%3bfill:%23ffccaa%3bfill-opacity:0.941176%3bstroke:%23fe0700%3bstroke-width:0.108539%3bstroke-linejoin:round%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 474.53546%2c924.92181 -4.61317%2c-5.83397 13.68848%2c-0.12591 c 23.94391%2c-0.22026 211.76765%2c-1.13686 211.87454%2c-1.03397 0.0564%2c0.0543 -2.45924%2c2.96276 -5.59033%2c6.46326 l -5.69289%2c6.36456 H 581.67537 479.14864 Z' id='path4969' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='opacity:1%3bfill:%23ffccaa%3bfill-opacity:0.941176%3bstroke:%23fe0700%3bstroke-width:0.108539%3bstroke-linejoin:round%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 488.47128%2c1054.3544 c -2.84589%2c-2.4292 -3.37685%2c-3.169 -3.60296%2c-5.0199 -0.14586%2c-1.1939 -0.76853%2c-23.7836 -1.38371%2c-50.19922 -0.61518%2c-26.41565 -1.26394%2c-50.32168 -1.44168%2c-53.1245 l -0.32317%2c-5.09604 99.50512%2c0.40066 c 54.72781%2c0.22037 99.59414%2c0.4861 99.70293%2c0.59051 0.1088%2c0.10441 0.0156%2c25.00386 -0.20716%2c55.3321 l -0.40499%2c55.14229 -4.80191%2c2.4116 -4.80191%2c2.4117 h -89.4514 -89.4514 z' id='path4971' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3cpath style='opacity:1%3bfill:%23ffccaa%3bfill-opacity:0.941176%3bstroke:%23fe0700%3bstroke-width:0.108539%3bstroke-linejoin:round%3bstroke-miterlimit:4%3bstroke-dasharray:none%3bstroke-opacity:1' d='m 557.53835%2c1081.5715 c -13.32755%2c-1.0908 -26.86778%2c-4.1464 -37.98862%2c-8.573 -3.88026%2c-1.5445 -7.90977%2c-3.2779 -8.95446%2c-3.8519 -1.88239%2c-1.0343 -1.29329%2c-1.0436 65.66604%2c-1.0368 l 67.56546%2c0.01 -2.70096%2c1.0951 c -4.81543%2c1.9524 -22.5931%2c6.8571 -30.9461%2c8.5377 -19.18788%2c3.8606 -36.66518%2c5.1295 -52.64136%2c3.822 z' id='path4973' inkscape:connector-curvature='0' transform='matrix(0.80946386%2c0%2c0%2c0.93840401%2c-239.9446%2c-60.730937)' /%3e %3c/g%3e %3c/g%3e %3c/g%3e %3c/svg%3e";

#[near_bindgen]
impl FungibleTokenMetadataProvider for Avrit {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("Avrit fungible token"),
            symbol: String::from("AVRIT"),
            icon: Some(String::from(AVRIT_SVG)),
            reference: None,
            reference_hash: None,
            decimals: 18,
        }
    }
}

// NFT
near_contract_standards::impl_non_fungible_token_core!(Avrit, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Avrit, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Avrit, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Avrit {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
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
    fn mint_myft(&mut self, owner_id: &AccountId, amount: u128) {
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
        self.mint_myft(&user_address, stake);
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
                            self.mint_myft(&account_id, mint_value);
                        }
                        // else if winning_decision == 2{   }
                        else if decision != winning_decision && winning_decision != 3 {
                            self.add_juror_voting_status_got_incentives(review_id, user_id);
                            let mint_value = (juror_stake as f64).powf(0.8) as u128;
                            // println!(">>>>>>>>>>>>>mintvalue{}<<<<<<<<<<<<<<<<<<<", mint_value);
                            if mint_value > self.jury_incentives {
                                self.mint_myft(&account_id, mint_value);
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
            self.mint_myft(&user_address, review_incentives + bountyvalue as u128);
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
                        incentives = self.product_oa_incentives / 2;
                    } else {
                        incentives = self.product_oa_incentives
                            / self.max_allowed_product_oa_incentives_count;
                    }
                } else if product_type == "ev" {
                    assert!(
                        product_incentive_count
                            <= self.max_allowed_product_evidence_incentives_count,
                        "Exceeds the number of allowed reviews"
                    );
                    if count <= 2 {
                        incentives = self.product_evidence_incentives / 2;
                    } else {
                        incentives = self.product_evidence_incentives
                            / self.max_allowed_product_evidence_incentives_count;
                    }
                }

                incentives
            }
            None => {
                self.product_incentives_count.insert(&product_id, &1);
                let mut incentives = 0;
                if product_type == "oa" {
                    incentives = self.product_oa_incentives / 2;
                } else if product_type == "ev" {
                    incentives = self.product_evidence_incentives / 2;
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
        // let _bounty = self.get_product_bounty(product_id);
        self.check_if_product_will_get_incentives(product_id, review_id);
        let product_incentives =
            self.increment_product_incentive_count_check_allowed_limit(product_type, product_id);
        assert!(
            product_incentives > 0,
            "Incentives should be greater than 0"
        );
        let winning_decision = self.get_winning_decision(review_id.into());
        if winning_decision == 1 && review.rating >= 3 {
            self.mint_myft(&user_address, product_incentives);
        } else {
            panic!("You are not eligible for incentives");
        }
    }

    // Crowdsale

    pub fn required_deposit(&self, number_of_tokens: U128) -> U128 {
        let number_of_tokens: u128 = number_of_tokens.into();
        if number_of_tokens == 0 {
            return 0.into();
        } else {
            let required_deposit =
                (number_of_tokens * self.token_price) + self.ft.storage_balance_bounds().min.0;
            return required_deposit.into();
        }
    }

    #[payable]
    pub fn buy_tokens(&mut self, number_of_tokens: U128) {
        let number_of_tokens: u128 = number_of_tokens.into();
        assert!(self.on_crowdsale == true, "Crowdsale has stalled");
        let amount = env::attached_deposit();
        let mut required_deposit = number_of_tokens * self.token_price;
        assert!(
            amount >= required_deposit + self.ft.storage_balance_bounds().min.0,
            "Requires attached deposit {}",
            required_deposit + self.ft.storage_balance_bounds().min.0
        );
        let account_id = env::predecessor_account_id();
        if !self.ft.accounts.contains_key(&account_id) {
            // Not registered, register if enough $NEAR has been attached.
            // Subtract registration amount from the account balance.
            self.ft.internal_register_account(&account_id);
            required_deposit = required_deposit + self.ft.storage_balance_bounds().min.0;
        }
        self.token_sold = (self.token_sold)
            .checked_add(number_of_tokens)
            .expect("Overflow");
        assert!(
            self.token_sold <= self.total_available_tokens,
            "No more tokens to sale"
        );
        assert!(
            self.token_sold <= self.phase_available_tokens,
            "No more tokens to sale"
        );
        self.mint_myft(&account_id, number_of_tokens);
        if amount - required_deposit > 0 {
            Promise::new(account_id).transfer(amount - required_deposit);
        }
    }

    // Upgrade contract

    #[init(ignore_state)]
    pub fn migrate() -> Self {
        #[derive(BorshDeserialize)]
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
            communication_id: u128,
            comment_product_id: u128,
            comment_review_id: u128,
            update_user_ids: LookupMap<u128, u128>, //(incremental time number, update_user_id)
            update_user_id_time_counter: u128,
            update_product_ids: LookupMap<u128, u128>, //(incremental time number, updated_product_id)
            update_product_id_time_counter: u128,
            update_review_ids: LookupMap<u128, u128>, //(incremental time number, updated_review_id)
            update_review_id_time_counter: u128,
            user_map: TreeMap<String, u128>, // (username, user_id)
            user_profile_map: TreeMap<u128, User>, // (user_id, User)
            product_map: TreeMap<u128, Product>, // (product_id, Product)
            review_map: TreeMap<u128, Review>, // (review_id, Review)
            communication_map: TreeMap<u128, Communication>, // (communication_id, Communication)
            comment_product_map: LookupMap<u128, CommentProduct>, // (comment_product_id, CommentProduct)
            comment_review_map: LookupMap<u128, CommentReview>, // (comment_review_id, CommentReview)
            user_products_map: TreeMap<u128, UnorderedSet<u128>>, // (user_id, set<product_id>)
            product_reviews_map: TreeMap<u128, UnorderedSet<u128>>, // (product_id, set<review_id>)
            product_commentproduct_map: LookupMap<u128, UnorderedSet<u128>>, // (product_id, set<commentproduct_id>)
            review_commentreview_map: LookupMap<u128, UnorderedSet<u128>>, // (review_id, set<commentreview_id>)
            product_crowdfunding: LookupMap<u128, u128>,                   // (product_id, bounty)
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
            selected_juror_count: LookupMap<u128, u64>, // <review_id, selected_juror_count> #Delete
            selected_juror: LookupMap<u128, LookupSet<u128>>, // <reviewer_id, jurorid>  #Delete
            juror_selection_time: LookupMap<u128, u64>, // <review_id, timestamp>
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
            schelling_decision_true_count: LookupMap<u128, u128>, // <review_id, true_count>
            schelling_decision_false_count: LookupMap<u128, u128>, // <review_id, false_count>
            jury_incentives: u128,                                // Extra incentives on winning
            review_incentives: u128,                              // Extra incentives on winning
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
            // Crowdsale
            token_price: u128,
            token_sold: u128,
            total_available_tokens: u128,
            phase_available_tokens: u128,
            on_crowdsale: bool,
        }

        let state: Avrit = env::state_read().unwrap();

        assert_eq!(
            &env::predecessor_account_id(),
            &state.owner_id,
            "Can only be called by the owner"
        );

        let metadata: NFTContractMetadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Example NEAR non-fungible token".to_string(),
            symbol: "EXAMPLE".to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        };

        let contract_id = env::current_account_id();

        Self {
            ft: state.ft,
            owner_id: state.owner_id,
            saving_id: state.saving_id,
            user_id: state.user_id,
            product_id: state.product_id,
            review_id: state.review_id,
            // update
            communication_id: state.communication_id,
            comment_product_id: state.comment_product_id,
            comment_review_id: state.comment_review_id,
            update_user_ids: state.update_user_ids,
            update_user_id_time_counter: state.update_user_id_time_counter,
            update_product_ids: state.update_product_ids,
            update_product_id_time_counter: state.update_product_id_time_counter,
            update_review_ids: state.update_review_ids,
            update_review_id_time_counter: state.update_review_id_time_counter,
            user_map: state.user_map,
            user_profile_map: state.user_profile_map,
            product_map: state.product_map,
            review_map: state.review_map,
            // update
            communication_map: state.communication_map,
            comment_product_map: state.comment_product_map,
            comment_review_map: state.comment_review_map,
            user_products_map: state.user_products_map,
            product_reviews_map: state.product_reviews_map,
            product_commentproduct_map: state.product_commentproduct_map,
            review_commentreview_map: state.review_commentreview_map,
            product_crowdfunding: state.product_crowdfunding,
            product_bounty: state.product_bounty,
            review_bounty: state.review_bounty,
            min_review_bounty: state.min_review_bounty,
            min_product_bounty: state.min_product_bounty,
            min_jury_stake: state.min_jury_stake,
            product_id_set_ucount: state.product_id_set_ucount,
            review_id_set_ucount: state.review_id_set_ucount,
            user_juror_stakes: state.user_juror_stakes,
            user_juror_stakes_clone: state.user_juror_stakes_clone,
            user_juror_stake_count: state.user_juror_stake_count,
            juror_unstaked: state.juror_unstaked,
            juror_stake_unique_id: state.juror_stake_unique_id,
            selected_juror: state.selected_juror,
            jury_count: state.jury_count,
            // max_number_of_jury_can_stake:state.// max_number_of_jury_can_stake,
            jury_application_phase_time: state.jury_application_phase_time,
            commit_phase_time: state.commit_phase_time,
            reveal_phase_time: state.reveal_phase_time,
            jury_incentives: state.jury_incentives,
            review_incentives: state.review_incentives,
            product_oa_incentives: state.product_oa_incentives,
            product_evidence_incentives: state.product_evidence_incentives,
            // Here is the gist of incentivestate.// Here is the gist of incentiv,
            // 30 times judge, 1 avritstate.// 30 times judge, 1 avri,
            // 15 times review, 1 avritstate.// 15 times review, 1 avri,
            // 1 time product, 1 avritstate.// 1 time product, 1 avri,
            review_got_incentives: state.review_got_incentives,
            product_got_incentives: state.product_got_incentives,
            product_incentives_count: state.product_incentives_count,
            max_allowed_product_oa_incentives_count: state.max_allowed_product_oa_incentives_count,
            max_allowed_product_evidence_incentives_count: state
                .max_allowed_product_evidence_incentives_count,
            selected_juror_count: state.selected_juror_count,
            jury_application_start_time: state.jury_application_start_time,
            juror_selection_time: state.juror_selection_time,
            voter_commit: state.voter_commit,
            juror_voting_status: state.juror_voting_status,
            schelling_decisions_juror: state.schelling_decisions_juror,
            schelling_decision_true_count: state.schelling_decision_true_count,
            schelling_decision_false_count: state.schelling_decision_false_count,
            number_of_allowed_reviews_per_product: state.number_of_allowed_reviews_per_product,
            product_review_count: state.product_review_count,
            token_price: state.token_price,
            token_sold: state.token_sold,
            total_available_tokens: state.total_available_tokens,
            phase_available_tokens: state.phase_available_tokens,
            on_crowdsale: state.on_crowdsale,
            // New code
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                contract_id.clone().try_into().unwrap(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            nft_token_count: LookupMap::new(b"d0903ca3".to_vec()),
            nft_token_mint_count: LookupMap::new(b"a9ec8b8d".to_vec()),
            nft_owner_incentives: LookupMap::new(b"8f574fbd".to_vec()),
            nft_token_price: LookupMap::new(b"42d54eac".to_vec()),
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

// NFT Code

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Avrit {
    pub fn setup_nft_price_and_token_count(
        &mut self,
        product_id: U128,
        price: U128,
        token_count: U128,
    ) {
        let product_id: u128 = product_id.into();
        let price: u128 = price.into();
        let token_count: u128 = token_count.into();
        let account_id = env::predecessor_account_id();
        let product = self.get_product(product_id);
        let user_id = self.get_user_id(&account_id);
        if user_id == product.user_id {
            // Logic
            self.set_nft_price(product_id, price);
            self.set_total_nft_count(product_id, token_count);
        } else {
            panic!("You are not the product owner");
        }
    }

    pub fn update_nft_price(&mut self, product_id: U128, price: U128) {
        let product_id: u128 = product_id.into();
        let price: u128 = price.into();
        let account_id = env::predecessor_account_id();
        let product = self.get_product(product_id);
        let user_id = self.get_user_id(&account_id);
        if user_id == product.user_id {
            // Logic
            self.set_nft_price_update(product_id, price);
        } else {
            panic!("You are not the product owner");
        }
    }

    fn set_nft_price(&mut self, product_id: u128, price: u128) {
        match self.nft_token_price.get(&product_id) {
            Some(_price) => {
                panic!("NFT price already set");
            }
            None => {
                self.nft_token_price.insert(&product_id, &price);
            }
        }
    }

    fn set_nft_price_update(&mut self, product_id: u128, price: u128) {
        match self.nft_token_price.get(&product_id) {
            Some(_price) => {
                self.nft_token_price.insert(&product_id, &price);
            }
            None => {
                panic!("NFT price is not set yet");
            }
        }
    }

    fn set_total_nft_count(&mut self, product_id: u128, token_count: u128) {
        match self.nft_token_count.get(&product_id) {
            Some(_count) => {
                panic!("NFT count already set");
            }
            None => {
                self.nft_token_count.insert(&product_id, &token_count);
            }
        }
    }

    fn get_nft_price(&self, product_id: u128) -> u128 {
        let price_option = self.nft_token_price.get(&product_id);
        match price_option {
            Some(price) => price,
            None => {
                panic!("Price not set.");
            }
        }
    }

    pub fn get_nft_price_js(&self, product_id: U128) -> U128 {
        let product_id: u128 = product_id.into();
        let price = self.get_nft_price(product_id);
        price.into()
    }

    fn get_total_nft_count(&self, product_id: u128) -> u128 {
        let count_option = self.nft_token_count.get(&product_id);
        match count_option {
            Some(count) => count,
            None => {
                panic!("Nft count not set");
            }
        }
    }

    pub fn get_total_nft_count_js(&self, product_id: U128) -> U128 {
        let product_id: u128 = product_id.into();
        let count = self.get_total_nft_count(product_id);
        count.into()
    }

    fn get_nft_count_name(&self, product_id: u128) -> String {
        let count_option = self.nft_token_mint_count.get(&product_id);
        match count_option {
            Some(count) => count.to_string(),
            None => "1".to_string(),
        }
    }

    // To Do:
    // Check deposit is the price of token
    // Check nft mint availabe from nft token count
    // Add the amount to nft owner incentives

    fn check_and_increment_nft_token_mint_count(&mut self, product_id: u128) {
        let total_nft_count = self.get_total_nft_count(product_id);
        let token_mint_count_option = self.nft_token_mint_count.get(&product_id);
        match token_mint_count_option {
            Some(count) => {
                if count >= total_nft_count {
                    panic!("You can not mint more tokens");
                } else {
                    let new_count = count.checked_add(1).expect("overflow");
                    self.nft_token_mint_count.insert(&product_id, &new_count);
                }
            }
            None => {
                self.nft_token_mint_count.insert(&product_id, &1);
            }
        }
    }

    fn increment_owner_incentives(&mut self, product_owner_id: u128, incentives_got: u128) {
        let incentives_option = self.nft_owner_incentives.get(&product_owner_id);
        match incentives_option {
            Some(incentives) => {
                let new_incentives = incentives.checked_add(incentives_got).expect("Overflow");
                self.nft_owner_incentives
                    .insert(&product_owner_id, &new_incentives);
            }
            None => {
                self.nft_owner_incentives
                    .insert(&product_owner_id, &incentives_got);
            }
        }
    }

    #[payable]
    pub fn buy_nft(&mut self, token_id: U128) {
        let product_id: u128 = token_id.into();
        let token_owner_id = env::predecessor_account_id();
        let product = self.get_product(product_id);
        let price = self.get_nft_price(product_id);
        let amount = env::attached_deposit();
        assert!(amount == price, "Price should be equal to deposit");
        let product_owner_id: u128 = product.user_id;
        self.increment_owner_incentives(product_owner_id, amount);
        self.check_and_increment_nft_token_mint_count(product_id);
        let countname = self.get_nft_count_name(product_id);
        let names = [product_id.to_owned().to_string(), countname];
        let joined_name = names.join("_");
        log!("NFT token id {}", joined_name);
        let token_metadata = TokenMetadata {
            title: Some(product.product_details_hash.into()),
            description: None,
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };
        self.tokens.mint(
            joined_name,
            token_owner_id.try_into().unwrap(),
            Some(token_metadata),
        );
    }

    // To Do, withdraw incentives product owner

    pub fn get_owner_incentives(&self, user_id: U128) -> U128 {
        let user_id: u128 = user_id.into();
        let incentives_option = self.nft_owner_incentives.get(&user_id);
        match incentives_option {
            Some(incentives) => incentives.into(),
            None => 0.into(),
        }
    }

    pub fn withdraw_product_owner_incentives(&mut self) {
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let incentives_option = self.nft_owner_incentives.get(&user_id);
        let storage_balance = self.ft.storage_balance_bounds().min.0;
        match incentives_option {
            Some(incentives) => {
                self.nft_owner_incentives.insert(&user_id, &0);
                if incentives == 0 {
                    panic!("You have no incentives to withdraw");
                } else if incentives > storage_balance {
                    let transfer_amount =
                        incentives.checked_sub(storage_balance).expect("Overflow");
                    Promise::new(account_id).transfer(transfer_amount);
                } else {
                    panic!("Incentives are less than storage balance");
                }
            }
            None => {
                panic!("No incentives to withdraw");
            }
        }
    }

    pub fn last_ten_tokens_for_owner(
        &self,
        user_id:U128
    ) -> Vec<Token> {
        let user_id: u128 = user_id.into();
        let user = self.get_user_profile(user_id);
        let user_address = user.username;
        let tokens_per_owner = self.tokens.tokens_per_owner.as_ref().expect(
            "Could not find tokens_per_owner when calling a method on the enumeration standard.",
        );
        let token_set = if let Some(token_set) = tokens_per_owner.get(&user_address) {
            token_set
        } else {
            return vec![];
        };

        let length_of_tokens = token_set.len() as u128;
        let start_index;
        let end_index;
        if length_of_tokens > 10 {
            start_index = length_of_tokens.checked_sub(10).expect("Overflow");
            end_index = 10;
        } else {
            start_index = 0;
            end_index = length_of_tokens;
        }
        token_set
            .iter()
            .skip(start_index as usize)
            .take(end_index as usize)
            .map(|token_id| self.enum_get_token_x(user_address.clone(), token_id))
            .collect()
    }

    fn enum_get_token_x(&self, owner_id: AccountId, token_id: TokenId) -> Token {
        let metadata = self.tokens.token_metadata_by_id.as_ref().unwrap().get(&token_id);
        let approved_account_ids =
            Some(self.tokens.approvals_by_id.as_ref().unwrap().get(&token_id).unwrap_or_default());

        Token { token_id, owner_id, metadata, approved_account_ids }
    }
}
