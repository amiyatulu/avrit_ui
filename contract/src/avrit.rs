use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, TreeMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, StorageUsage};
use chrono::{Duration, NaiveDateTime};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use sha3::{Digest, Keccak256};

pub mod account;
pub use self::account::Account;
pub mod avritstructs;
pub use self::avritstructs::{Product, Review, User};

/// Price per 1 byte of storage from mainnet genesis config.
pub const STORAGE_PRICE_PER_BYTE: Balance = 100000000000000000000;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
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
    user_id: u128,
    product_id: u128,
    review_id: u128,
    user_map: TreeMap<String, u128>,       // (username, user_id)
    user_profile_map: TreeMap<u128, User>, // (user_id, User)
    product_map: TreeMap<u128, Product>,   // (product_id, Product)
    review_map: TreeMap<u128, Review>,     // (review_id, Review)
    user_products_map: TreeMap<u128, UnorderedSet<u128>>, // (user_id, set<product_id>)
    product_reviews_map: TreeMap<u128, UnorderedSet<u128>>, // (product_id, set<review_id>)
    product_check_bounty: LookupMap<u128, Vector<u64>>, // (product_id, bounty -> 0 index +  0_bountyperiodover 1_bountyperiodexists -> 1 index)
    review_check_bounty: LookupMap<u128, Vector<u64>>, // (review_id, bounty -> 0 index +  0_bountyperiodover 1_bountyperiodexists -> 1 index)
    user_juror_stakes: LookupMap<u128, LookupMap<u128, u128>>, // <reviewer_id, <jurorid, stakes>>
    user_juror_stakes_clone: LookupMap<u128, TreeMap<u128, u128>>,
    juror_stake_unique_id: u128,
    selected_juror_count: LookupMap<u128, u64>, // <review_id, selected_juror_count>
    selected_juror: LookupMap<u128, LookupSet<u128>>, // <reviewer_id, jurorid>
    juror_selection_time: LookupMap<u128, u64>,
    product_id_set_ucount: u128,
    review_id_set_ucount: u128,
    product_check_bounty_vector_ucount: u128,
    review_check_bounty_vector_ucount: u128,
    jury_count: u64,
    commit_phase_time: u64, // Commit phase time in seconds
    reveal_phase_time: u64, // Reveal phase time in seconds
    voter_commit: LookupMap<u128, LookupMap<String, u8>>, // review_id, vote_commits, 1 if commited, 2 if revealed
    juror_voting_status: LookupMap<u128, LookupMap<u128, u8>>, // review_id, <juror id, 0 or null =not commited, 1=commited, 2=revealed, 3=got the incentives>
    schelling_decisions_juror: LookupMap<u128, LookupMap<u128, u8>>, // <reviewer_id, <jurorid, 1=true 0=false>>
    schelling_decision_true_count: LookupMap<u128, u128>,            // <reviewer_id, true_count>
    schelling_decision_false_count: LookupMap<u128, u128>,           // <reviewer_id, false_count>
    jury_incentives: u128,
    // Fungible Token
    /// sha256(AccountID) -> Account details.
    accounts: UnorderedMap<Vec<u8>, Account>,

    /// Total supply of the all token.
    total_supply: Balance,
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
    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }
    pub fn set_commit_phase_time(&mut self, time_in_secs: u64) {
        self.assert_owner();
        self.commit_phase_time = time_in_secs;
    }
    pub fn set_reveal_phase_time(&mut self, time_in_secs: u64) {
        self.assert_owner();
        self.reveal_phase_time = time_in_secs;
    }
    pub fn set_jury_count(&mut self, jury_count: u64) {
        self.assert_owner();
        self.jury_count = jury_count;
    }
    pub fn set_jury_incentives(&mut self, incentives: u128) {
        self.assert_owner();
        self.jury_incentives = incentives;
    }
}

#[near_bindgen]
impl Avrit {
    pub fn get_user_id(&self, account_id: &AccountId) -> u128 {
        let user_id_option = self.user_map.get(&account_id);
        match user_id_option {
            Some(user_id) => user_id,
            None => {
                panic!("User id doesnot exist for AccountId {}", account_id);
            }
        }
    }
    pub fn get_user_details(&self, user_id: u128) -> User {
        let user_profile_option = self.user_profile_map.get(&user_id);
        let user = user_profile_option.unwrap();
        user
    }
    pub fn create_profile(&mut self, profile_hash: String) {
        let account_id = env::predecessor_account_id();
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

    pub fn create_product(&mut self, product_details_hash: String) {
        let account_id = env::predecessor_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        match account_id_exists_option {
            Some(user_id) => {
                self.product_id += 1;
                let prod = Product {
                    user_id,
                    product_details_hash,
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

    pub fn get_products_of_user(&self) -> Vec<u128> {
        let account_id = env::predecessor_account_id();
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

    pub fn update_product(&mut self, product_id: u128, product_details_hash: String) {
        let account_id = env::predecessor_account_id();
        let mut product = self.product_map.get(&product_id).unwrap();
        // println!("{:?} user_id", product.user_id);
        let user_id = self.user_map.get(&account_id).unwrap();
        // println!("{:?} user_id from account", user_id);
        if user_id == product.user_id {
            product.product_details_hash = product_details_hash;
        }
        // println!("{:?} product", product);
        self.product_map.insert(&product_id, &product);
    }

    pub fn get_product(&self, product_id: u128) -> Product {
        let product = self.product_map.get(&product_id).unwrap();
        product
    }

    pub fn create_review(&mut self, product_id: u128, review_hash: String) {
        let account_id = env::predecessor_account_id();
        let account_id_exists_option = self.user_map.get(&account_id);
        let _product_exist = self.product_map.get(&product_id).unwrap();

        match account_id_exists_option {
            Some(user_id) => {
                let rev = Review {
                    product_id,
                    user_id,
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

    pub fn get_review(&self, review_id: u128) -> Review {
        let review = self.review_map.get(&review_id).unwrap();
        review
    }

    pub fn add_product_bounty(&mut self, bounty: u64, product_id: u128) {
        let account_id = env::predecessor_account_id();
        // println!(">>>>add product bounty{}<<<<<<<<<<", account_id);
        let product_bounty_exists_option = self.product_check_bounty.get(&product_id);
        match product_bounty_exists_option {
            Some(mut bountyvector) => {
                let bountyperiod = bountyvector.get(1).unwrap();
                if bountyperiod == 1 {
                    let bountyvalue = bountyvector.get(0).unwrap();
                    if bounty > bountyvalue {
                        self.burn(&account_id, (bounty - bountyvalue) as u128);
                        bountyvector.replace(0, &bounty);
                        self.product_check_bounty.insert(&product_id, &bountyvector);
                    } else {
                        panic!("Bounty period has not ended, please enter amount of higher value");
                    }
                }
                if bountyperiod == 0 {
                    self.burn(&account_id, bounty as u128);
                    bountyvector.replace(0, &bounty);
                    bountyvector.replace(1, &1);
                    self.product_check_bounty.insert(&product_id, &bountyvector);
                }
            }
            None => {
                let bountyvectorstring =
                    format!("bountyproductid{}", self.product_check_bounty_vector_ucount);
                let boutyvectorstrindid = bountyvectorstring.to_string().into_bytes();
                let mut bountyvector: Vector<u64> = Vector::new(boutyvectorstrindid);
                self.product_check_bounty_vector_ucount += 1;
                self.burn(&account_id, bounty as u128);
                bountyvector.push(&bounty);
                bountyvector.push(&1);
                self.product_check_bounty.insert(&product_id, &bountyvector);
            }
        }
    }

    // pub fn get_product_bounty(&mut self, product_id: u128) -> Vector<u64> {
    //     let bounty_option = self.product_check_bounty.get(&product_id);
    //     match bounty_option {
    //         Some(bountyvector) => bountyvector,
    //         None => {
    //             panic!("Bounty doesn't exists");
    //         }
    //     }
    // }
    pub fn add_review_bounty(&mut self, bounty: u64, review_id: u128) {
        let account_id = env::predecessor_account_id();
        let review_bounty_exists_option = self.review_check_bounty.get(&review_id);
        match review_bounty_exists_option {
            Some(mut bountyvector) => {
                let bountyperiod = bountyvector.get(1).unwrap();
                if bountyperiod == 1 {
                    let bountyvalue = bountyvector.get(0).unwrap();
                    if bounty > bountyvalue {
                        self.burn(&account_id, (bounty - bountyvalue) as u128);
                        bountyvector.replace(0, &bounty);
                        self.review_check_bounty.insert(&review_id, &bountyvector);
                    } else {
                        panic!("Bounty period has not ended, please enter amount of higher value");
                    }
                }
                if bountyperiod == 0 {
                    self.burn(&account_id, bounty as u128);
                    bountyvector.replace(0, &bounty);
                    bountyvector.replace(1, &1);
                    self.review_check_bounty.insert(&review_id, &bountyvector);
                }
            }
            None => {
                let bountyvectorstring =
                    format!("bountyreviewid{}", self.review_check_bounty_vector_ucount);
                let boutyvectorstrindid = bountyvectorstring.to_string().into_bytes();
                let mut bountyvector: Vector<u64> = Vector::new(boutyvectorstrindid);
                self.review_check_bounty_vector_ucount += 1;
                self.burn(&account_id, bounty as u128);
                bountyvector.push(&bounty);
                bountyvector.push(&1);
                self.review_check_bounty.insert(&review_id, &bountyvector);
            }
        }
    }
}

impl Default for Avrit {
    fn default() -> Self {
        panic!("Fun token should be initialized before usage")
    }
}

#[near_bindgen]
impl Avrit {
    /// Initializes the contract with the given total supply owned by the given `owner_id`.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128) -> Self {
        let total_supply = total_supply.into();
        assert!(!env::state_exists(), "Already initialized");
        let mut ft = Self {
            accounts: UnorderedMap::new(b"2d965fc1-874a-4240-a328-9c3c4b00be2a".to_vec()),
            total_supply,
            owner_id: owner_id.clone(),
            user_id: 0,
            product_id: 0,
            review_id: 0,
            user_map: TreeMap::new(b"061af613-4e63-4fc8-9b16-a30a7aa3a8b9".to_vec()),
            user_profile_map: TreeMap::new(b"589d167f-fc96-4299-89d0-b6d57fb41803".to_vec()),
            product_map: TreeMap::new(b"cf27d94f-6066-4aa0-90fd-a9bf7ac9dc3b".to_vec()),
            review_map: TreeMap::new(b"5fc2c77f-c84e-4da8-b8ab-ea0524995549".to_vec()),
            user_products_map: TreeMap::new(b"e7b6e8a6-ccee-4887-9eff-21bb49c5c257".to_vec()),
            product_reviews_map: TreeMap::new(b"ea4ee217-662f-43f0-8ef0-cf96d411afe7".to_vec()),
            product_check_bounty: LookupMap::new(b"0566cfb4-19e1-4895-b50c-68f6c0b90e40".to_vec()),
            review_check_bounty: LookupMap::new(b"00423f89-b178-4697-9ea7-316d08504b0a".to_vec()),
            product_id_set_ucount: 0,
            review_id_set_ucount: 0,
            product_check_bounty_vector_ucount: 0,
            review_check_bounty_vector_ucount: 0,
            user_juror_stakes: LookupMap::new(b"e56291ef-2806-4298-8646-054d5d116a70".to_vec()),
            user_juror_stakes_clone: LookupMap::new(
                b"4e74c845-0608-4c55-af1e-86eb8bf01687".to_vec(),
            ),
            juror_stake_unique_id: 0,
            selected_juror: LookupMap::new(b"89390257-80c2-446a-bc21-fac4885250ee".to_vec()),
            jury_count: 20,
            commit_phase_time: 2592000, // 30 days in secs
            reveal_phase_time: 1296000, // 15 days in secs
            jury_incentives: 10,
            selected_juror_count: LookupMap::new(b"532caf99-c5e5-4be5-8e23-802388aa86d5".to_vec()),
            juror_selection_time: LookupMap::new(b"5942be3d-b37f-4cb0-afaa-9ec8a831df00".to_vec()),
            voter_commit: LookupMap::new(b"a11fe88d-be47-4709-8a54-58da79218c3e".to_vec()),
            juror_voting_status: LookupMap::new(b"4c4879f8-096b-4201-8ce3-64141c2eebf6".to_vec()),
            schelling_decisions_juror: LookupMap::new(
                b"8c7b8f85-1ba6-4a2a-83e8-3cfc07d7355e".to_vec(),
            ),
            schelling_decision_true_count: LookupMap::new(
                b"4bf8d29d-aadc-4c62-89ce-fe2382197ae2".to_vec(),
            ),
            schelling_decision_false_count: LookupMap::new(
                b"98396f41-606d-4cf0-b06f-2668db6f6238".to_vec(),
            ),
        };
        let mut account = ft.get_account(&owner_id);
        account.balance = total_supply;
        ft.set_account(&owner_id, &account);
        ft
    }

    /// Increments the `allowance` for `escrow_account_id` by `amount` on the account of the caller of this contract
    /// (`predecessor_id`) who is the balance owner.
    /// Requirements:
    /// * Caller of the method has to attach deposit enough to cover storage difference at the
    ///   fixed storage price defined in the contract.
    #[payable]
    pub fn inc_allowance(&mut self, escrow_account_id: AccountId, amount: U128) {
        let initial_storage = env::storage_usage();
        assert!(
            env::is_valid_account_id(escrow_account_id.as_bytes()),
            "Escrow account ID is invalid"
        );
        let owner_id = env::predecessor_account_id();
        if escrow_account_id == owner_id {
            env::panic(b"Can not increment allowance for yourself");
        }
        let mut account = self.get_account(&owner_id);
        let current_allowance = account.get_allowance(&escrow_account_id);
        account.set_allowance(
            &escrow_account_id,
            current_allowance.saturating_add(amount.0),
        );
        self.set_account(&owner_id, &account);
        self.refund_storage(initial_storage);
    }

    /// Decrements the `allowance` for `escrow_account_id` by `amount` on the account of the caller of this contract
    /// (`predecessor_id`) who is the balance owner.
    /// Requirements:
    /// * Caller of the method has to attach deposit enough to cover storage difference at the
    ///   fixed storage price defined in the contract.
    #[payable]
    pub fn dec_allowance(&mut self, escrow_account_id: AccountId, amount: U128) {
        let initial_storage = env::storage_usage();
        assert!(
            env::is_valid_account_id(escrow_account_id.as_bytes()),
            "Escrow account ID is invalid"
        );
        let owner_id = env::predecessor_account_id();
        if escrow_account_id == owner_id {
            env::panic(b"Can not decrement allowance for yourself");
        }
        let mut account = self.get_account(&owner_id);
        let current_allowance = account.get_allowance(&escrow_account_id);
        account.set_allowance(
            &escrow_account_id,
            current_allowance.saturating_sub(amount.0),
        );
        self.set_account(&owner_id, &account);
        self.refund_storage(initial_storage);
    }

    /// Transfers the `amount` of tokens from `owner_id` to the `new_owner_id`.
    /// Requirements:
    /// * `amount` should be a positive integer.
    /// * `owner_id` should have balance on the account greater or equal than the transfer `amount`.
    /// * If this function is called by an escrow account (`owner_id != predecessor_account_id`),
    ///   then the allowance of the caller of the function (`predecessor_account_id`) on
    ///   the account of `owner_id` should be greater or equal than the transfer `amount`.
    /// * Caller of the method has to attach deposit enough to cover storage difference at the
    ///   fixed storage price defined in the contract.
    #[payable]
    pub fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, amount: U128) {
        let initial_storage = env::storage_usage();
        assert!(
            env::is_valid_account_id(new_owner_id.as_bytes()),
            "New owner's account ID is invalid"
        );
        let amount = amount.into();
        if amount == 0 {
            env::panic(b"Can't transfer 0 tokens");
        }
        assert_ne!(
            owner_id, new_owner_id,
            "The new owner should be different from the current owner"
        );
        // Retrieving the account from the state.
        let mut account = self.get_account(&owner_id);

        // Checking and updating unlocked balance
        if account.balance < amount {
            env::panic(b"Not enough balance");
        }
        account.balance -= amount;

        // If transferring by escrow, need to check and update allowance.
        let escrow_account_id = env::predecessor_account_id();
        if escrow_account_id != owner_id {
            let allowance = account.get_allowance(&escrow_account_id);
            if allowance < amount {
                env::panic(b"Not enough allowance");
            }
            account.set_allowance(&escrow_account_id, allowance - amount);
        }

        // Saving the account back to the state.
        self.set_account(&owner_id, &account);

        // Deposit amount to the new owner and save the new account to the state.
        let mut new_account = self.get_account(&new_owner_id);
        new_account.balance += amount;
        self.set_account(&new_owner_id, &new_account);
        self.refund_storage(initial_storage);
    }

    /// Transfer `amount` of tokens from the caller of the contract (`predecessor_id`) to
    /// `new_owner_id`.
    /// Act the same was as `transfer_from` with `owner_id` equal to the caller of the contract
    /// (`predecessor_id`).
    /// Requirements:
    /// * Caller of the method has to attach deposit enough to cover storage difference at the
    ///   fixed storage price defined in the contract.
    #[payable]
    pub fn transfer(&mut self, new_owner_id: AccountId, amount: U128) {
        // NOTE: New owner's Account ID checked in transfer_from.
        // Storage fees are also refunded in transfer_from.
        self.transfer_from(env::predecessor_account_id(), new_owner_id, amount);
    }

    /// Returns total supply of tokens.
    pub fn get_total_supply(&self) -> U128 {
        self.total_supply.into()
    }

    /// Returns balance of the `owner_id` account.
    pub fn get_balance(&self, owner_id: AccountId) -> U128 {
        self.get_account(&owner_id).balance.into()
    }

    /// Returns current allowance of `escrow_account_id` for the account of `owner_id`.
    ///
    /// NOTE: Other contracts should not rely on this information, because by the moment a contract
    /// receives this information, the allowance may already be changed by the owner.
    /// So this method should only be used on the front-end to see the current allowance.
    pub fn get_allowance(&self, owner_id: AccountId, escrow_account_id: AccountId) -> U128 {
        assert!(
            env::is_valid_account_id(escrow_account_id.as_bytes()),
            "Escrow account ID is invalid"
        );
        self.get_account(&owner_id)
            .get_allowance(&escrow_account_id)
            .into()
    }
}

#[near_bindgen]
impl Avrit {
    /// Helper method to get the account details for `owner_id`.
    fn get_account(&self, owner_id: &AccountId) -> Account {
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Owner's account ID is invalid"
        );
        let account_hash = env::sha256(owner_id.as_bytes());
        self.accounts
            .get(&account_hash)
            .unwrap_or_else(|| Account::new(account_hash))
    }

    /// Helper method to set the account details for `owner_id` to the state.
    fn set_account(&mut self, owner_id: &AccountId, account: &Account) {
        let account_hash = env::sha256(owner_id.as_bytes());
        if account.balance > 0 || !account.allowances.is_empty() {
            self.accounts.insert(&account_hash, &account);
        } else {
            self.accounts.remove(&account_hash);
        }
    }

    fn refund_storage(&self, initial_storage: StorageUsage) {
        let current_storage = env::storage_usage();
        let attached_deposit = env::attached_deposit();
        let refund_amount = if current_storage > initial_storage {
            let required_deposit =
                Balance::from(current_storage - initial_storage) * STORAGE_PRICE_PER_BYTE;
            assert!(
                required_deposit <= attached_deposit,
                "The required attached deposit is {}, but the given attached deposit is is {}",
                required_deposit,
                attached_deposit,
            );
            attached_deposit - required_deposit
        } else {
            attached_deposit
                + Balance::from(initial_storage - current_storage) * STORAGE_PRICE_PER_BYTE
        };
        if refund_amount > 0 {
            env::log(format!("Refunding {} tokens for storage", refund_amount).as_bytes());
            Promise::new(env::predecessor_account_id()).transfer(refund_amount);
        }
    }
}

// Burn and mint
#[near_bindgen]
impl Avrit {
    fn mint(&mut self, owner_id: &AccountId, amount: u128) {
            let initial_storage = env::storage_usage();
            if amount == 0 {
                env::panic(b"Can't transfer 0 tokens");
            }
            assert!(
                env::is_valid_account_id(owner_id.as_bytes()),
                "New owner's account ID is invalid"
            );
            let mut account = self.get_account(&owner_id);
            account.balance += amount;
            self.set_account(&owner_id, &account);
            self.total_supply = self.total_supply + amount;
            self.refund_storage(initial_storage);
    }

    fn burn(&mut self, owner_id: &AccountId, amount: u128) {
            let initial_storage = env::storage_usage();
            if amount == 0 {
                env::panic(b"Can't transfer 0 tokens");
            }
            assert!(
                env::is_valid_account_id(owner_id.as_bytes()),
                "Owner's account ID is invalid"
            );
            let mut account = self.get_account(&owner_id);

            account.balance -= amount;
            self.set_account(&owner_id, &account);
            self.total_supply = self.total_supply - amount;
            self.refund_storage(initial_storage);
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
    /// Apply Jurors with stake
    pub fn apply_jurors(&mut self, review_id: u128, stake: u128) {
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

    pub fn draw_jurors(&mut self, review_id: u128, length: usize) {
        let selected_juror_option = self.selected_juror.get(&review_id);
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
    fn draw_jurors_function(
        &mut self,
        review_id: u128,
        mut jurysetentries: LookupSet<u128>,
        length: usize,
    ) {
        let user_juror_stakes_clone_option = self.user_juror_stakes_clone.get(&review_id);
        match user_juror_stakes_clone_option {
            Some(mut juries_stakes) => {
                let items = juries_stakes.to_vec();
                // println!(">>>>>>>>Juries{:?}<<<<<<<<<<<", items);
                let random_vec = env::random_seed();
                let mut rng = self.get_rng(random_vec);
                let mut dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
                let selected_juror_count_option = self.selected_juror_count.get(&review_id);
                let mut countvalue = 0;
                match selected_juror_count_option {
                    Some(count) => {
                        if count >= self.jury_count {
                            panic!("Jury selection done");
                        } else {
                            countvalue = count;
                        }
                    }
                    None => {}
                }

                for _ in 0..length {
                    let index = dist2.sample(&mut rng);
                    // println!("{}", index);
                    let drawindex = items[index].0;
                    println!("{:?}", drawindex);
                    juries_stakes.remove(&drawindex);
                    jurysetentries.insert(&drawindex);
                    let d = dist2.update_weights(&[(index, &0)]);
                    // println!("{:?}",d);
                    match d {
                        Ok(_v) => {}
                        Err(_e) => {
                            let timestamp = env::block_timestamp();
                            self.juror_selection_time.insert(&review_id, &timestamp);
                            break;
                        }
                    }
                    countvalue += 1;
                    if countvalue >= self.jury_count {
                        let timestamp = env::block_timestamp();
                        self.juror_selection_time.insert(&review_id, &timestamp);
                        break;
                    }
                }
                self.selected_juror_count.insert(&review_id, &countvalue);
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
    pub fn get_juror_stakes(&self, review_id: u128, juror_user_id: u128) -> u128 {
        let juror_list_option = self.user_juror_stakes.get(&review_id);
        match juror_list_option {
            Some(juror_list) => {
                let juror_stake = juror_list.get(&juror_user_id).unwrap();
                juror_stake
            }
            None => panic!("No one has staked for the voter"),
        }
    }

    pub fn get_juror_selection_time(&self, review_id: &u128) -> u64 {
        let timestamp_juror_selection_time_option = self.juror_selection_time.get(&review_id);
        match timestamp_juror_selection_time_option {
            Some(timestamp) => timestamp,
            None => {
                panic!("Jurors are not selected yet");
            }
        }
    }

    pub fn commit_vote(&mut self, review_id: u128, vote_commit: String) {
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        // println!("{}, now2", naive_now);
        let timestamp_juror_selection_time = self.get_juror_selection_time(&review_id);
        let native_juror_selection_time =
            NaiveDateTime::from_timestamp(timestamp_juror_selection_time as i64, 0);
        let seconds = Duration::seconds(self.commit_phase_time as i64);
        let endtime = native_juror_selection_time + seconds;
        if naive_now > endtime {
            panic!("Commiting time has ended");
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
    pub fn can_juror_vote(&self, review_id: u128, user_id: u128) {
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
    pub fn reveal_vote(&mut self, review_id: u128, vote: String, vote_commit: String) {
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        let timestamp_juror_selection_time = self.get_juror_selection_time(&review_id);
        let native_juror_selection_time =
            NaiveDateTime::from_timestamp(timestamp_juror_selection_time as i64, 0);
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

    pub fn get_true_count(&self, review_id: u128) -> u128 {
        let count_option = self.schelling_decision_true_count.get(&review_id);
        match count_option {
            Some(count) => count,
            None => {
                panic!("Count is not set");
            }
        }
    }

    pub fn get_false_count(&self, review_id: u128) -> u128 {
        let count_option = self.schelling_decision_false_count.get(&review_id);
        match count_option {
            Some(count) => count,
            None => {
                panic!("Count is not set");
            }
        }
    }
    pub fn get_winning_decision(&self, review_id: u128) -> u8 {
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        let timestamp_juror_selection_time = self.get_juror_selection_time(&review_id);
        let native_juror_selection_time =
            NaiveDateTime::from_timestamp(timestamp_juror_selection_time as i64, 0);
        let seconds = Duration::seconds(self.commit_phase_time as i64);
        let reveal_end_seconds = Duration::seconds(self.reveal_phase_time as i64);
        let reveal_endtime = native_juror_selection_time + seconds + reveal_end_seconds;
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

    pub fn incentives_distribution(&mut self, review_id: u128) {
        let account_id = env::predecessor_account_id();
        // println!(">>>>>>>>>>>>>>>>>>>>>>>>>accountid {}<<<<<<<<<<<<<<<<<<<<<", account_id);
        let user_id = self.get_user_id(&account_id);
        self.can_juror_vote(review_id, user_id);
        let winning_decision = self.get_winning_decision(review_id);
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
                            let mint_value = (juror_stake as f64).powf(0.8) as u128 + 1;
                            println!(">>>>>>>>>>>>>mintvalue{}<<<<<<<<<<<<<<<<<<<", mint_value);
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
}

