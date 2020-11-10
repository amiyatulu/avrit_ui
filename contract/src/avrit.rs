use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, TreeMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, StorageUsage};

pub mod account;
pub use self::account::Account;
pub mod avritstructs;
pub use self::avritstructs::{Product, Review, User};
pub mod schelling_game;

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
    voter_commit: LookupMap<u128, LookupMap<String, u8>>, // review_id, vote_commits, 1 if commited, 2 if revealed
    juror_voting_status: LookupMap<u128, LookupMap<u128, u8>>, // review_id, <juror id, 0 or null =not commited, 1=commited, 2=revealed>
    schelling_decisions_juror: LookupMap<u128, LookupMap<u128, u8>>, // <reviewer_id, <jurorid, true_or_false>>
    schelling_decision_true_count: LookupMap<u128, u128>, // <reviewer_id, true_count>
    schelling_decision_false_count: LookupMap<u128, u128>, // <reviewer_id, false_count>

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
    pub fn set_jury_count(&mut self, jury_count: u64) {
        self.assert_owner();
        self.jury_count = jury_count;
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

    pub fn get_product_bounty(&mut self, product_id: u128) -> Vector<u64> {
        let bounty_option = self.product_check_bounty.get(&product_id);
        match bounty_option {
            Some(bountyvector) => bountyvector,
            None => {
                panic!("Bounty doesn't exists");
            }
        }
    }
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
            selected_juror_count: LookupMap::new(b"532caf99-c5e5-4be5-8e23-802388aa86d5".to_vec()),
            juror_selection_time: LookupMap::new(b"5942be3d-b37f-4cb0-afaa-9ec8a831df00".to_vec()),
            voter_commit: LookupMap::new(b"a11fe88d-be47-4709-8a54-58da79218c3e".to_vec()),
            juror_voting_status: LookupMap::new(b"4c4879f8-096b-4201-8ce3-64141c2eebf6".to_vec()),
            schelling_decisions_juror: LookupMap::new(b"8c7b8f85-1ba6-4a2a-83e8-3cfc07d7355e".to_vec()),
            schelling_decision_true_count: LookupMap::new(b"4bf8d29d-aadc-4c62-89ce-fe2382197ae2".to_vec()),
            schelling_decision_false_count: LookupMap::new(b"98396f41-606d-4cf0-b06f-2668db6f6238".to_vec()),
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
    fn _mint(&mut self, owner_id: &AccountId, amount: u128) {
        if !owner_id.is_empty() {
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
    }

    fn burn(&mut self, owner_id: &AccountId, amount: u128) {
        if !owner_id.is_empty() {
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
}
