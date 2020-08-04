use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{TreeMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance};

pub mod avritstructs;
pub use self::avritstructs::{Product, Review, User};
pub mod account;
pub use self::account::Account;
pub mod sortitionsumtree;
pub use self::sortitionsumtree::SortitionSumTrees;

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
    pub user_id: u128,
    pub product_id: u128,
    pub review_id: u128,
    pub user_map: TreeMap<String, u128>, // (username, user_id)
    pub user_profile_map: TreeMap<u128, User>, // (user_id, User)
    pub product_map: TreeMap<u128, Product>, // (product_id, Product)
    pub review_map: TreeMap<u128, Review>, // (review_id, Review)
    pub user_products_map: TreeMap<u128, UnorderedSet<u128>>, // (user_id, set<product_id>)
    pub product_reviews_map: TreeMap<u128, UnorderedSet<u128>>, // (product_id, set<review_id>)
    // Fungible Token
    pub product_id_set_ucount: u128,
    pub review_id_set_ucount: u128,
    // sha256(AccountID) -> Account details.
    pub accounts: TreeMap<Vec<u8>, Account>,

    /// Total supply of the all token.
    pub total_supply: Balance,

    pub sortition: SortitionSumTrees,
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
        let account_id = env::signer_account_id();
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

    pub fn update_product(&mut self, product_id: u128, product_details_hash: String) {
        let account_id = env::signer_account_id();
        let mut product = self.product_map.get(&product_id).unwrap();
        // println!("{:?} user_id", product.user_id);
        let user_id = self.user_map.get(&account_id).unwrap();
        // println!("{:?} user_id from account", user_id);
        if user_id == product.user_id {
            product.product_details_hash = product_details_hash
        }
        // println!("{:?} product", product);
        self.product_map.insert(&product_id, &product);
    }

    pub fn get_product(&self, product_id: u128) -> (String, bool) {
        let product = self.product_map.get(&product_id).unwrap();
        (product.product_details_hash, product.product_expired)
    }

    pub fn create_review(&mut self, product_id: u128, review_hash: String) {
        let account_id = env::signer_account_id();
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
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        let mut ft = Self {
            accounts: TreeMap::new(b"2d965fc1-874a-4240-a328-9c3c4b00be2a".to_vec()),
            total_supply,
            user_id: 0,
            product_id: 0,
            review_id: 0,
            user_map: TreeMap::new(b"061af613-4e63-4fc8-9b16-a30a7aa3a8b9".to_vec()),
            user_profile_map: TreeMap::new(b"589d167f-fc96-4299-89d0-b6d57fb41803".to_vec()),
            product_map: TreeMap::new(b"cf27d94f-6066-4aa0-90fd-a9bf7ac9dc3b".to_vec()),
            review_map: TreeMap::new(b"5fc2c77f-c84e-4da8-b8ab-ea0524995549".to_vec()),
            user_products_map: TreeMap::new(b"e7b6e8a6-ccee-4887-9eff-21bb49c5c257".to_vec()),
            product_reviews_map: TreeMap::new(b"ea4ee217-662f-43f0-8ef0-cf96d411afe7".to_vec()),
            sortition: SortitionSumTrees::new(),
            product_id_set_ucount: 0,
            review_id_set_ucount: 0,
        };
        let mut account = ft.get_account(&owner_id);
        account.balance = total_supply;
        ft.set_account(&owner_id, &account);
        ft
    }

    /// Sets the `allowance` for `escrow_account_id` on the account of the caller of this contract
    /// (`predecessor_id`) who is the balance owner.
    pub fn set_allowance(&mut self, escrow_account_id: AccountId, allowance: U128) {
        let allowance = allowance.into();
        let owner_id = env::predecessor_account_id();
        if escrow_account_id == owner_id {
            env::panic(b"Can't set allowance for yourself");
        }
        let mut account = self.get_account(&owner_id);

        account.set_allowance(&escrow_account_id, allowance);
        self.set_account(&owner_id, &account);
    }

    /// Transfers the `amount` of tokens from `owner_id` to the `new_owner_id`.
    /// Requirements:
    /// * `amount` should be a positive integer.
    /// * `owner_id` should have balance on the account greater or equal than the transfer `amount`.
    /// * If this function is called by an escrow account (`owner_id != predecessor_account_id`),
    ///   then the allowance of the caller of the function (`predecessor_account_id`) on
    ///   the account of `owner_id` should be greater or equal than the transfer `amount`.
    pub fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, amount: U128) {
        let amount = amount.into();
        if amount == 0 {
            env::panic(b"Can't transfer 0 tokens");
        }
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
    }

    /// Transfer `amount` of tokens from the caller of the contract (`predecessor_id`) to
    /// `new_owner_id`.
    /// Act the same was as `transfer_from` with `owner_id` equal to the caller of the contract
    /// (`predecessor_id`).
    pub fn transfer(&mut self, new_owner_id: AccountId, amount: U128) {
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
        self.get_account(&owner_id)
            .get_allowance(&escrow_account_id)
            .into()
    }
}

impl Avrit {
    /// Helper method to get the account details for `owner_id`.
    fn get_account(&self, owner_id: &AccountId) -> Account {
        let account_hash = env::sha256(owner_id.as_bytes());
        self.accounts
            .get(&account_hash)
            .unwrap_or_else(|| Account::new(account_hash))
    }

    /// Helper method to set the account details for `owner_id` to the state.
    fn set_account(&mut self, owner_id: &AccountId, account: &Account) {
        let account_hash = env::sha256(owner_id.as_bytes());
        self.accounts.insert(&account_hash, &account);
    }
}
