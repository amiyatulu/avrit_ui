use borsh::{BorshDeserialize, BorshSerialize};


#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct User {
    pub profile_hash: String, //IPFS Hash
    pub kyc_done: bool,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Review {
    pub product_id: u128,
    pub user_id: u128,
    pub review_hash: String, //IPFS Hash
}


#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Product {
    pub user_id: u128,
    pub product_details_hash: String, //IPFS Hash
    pub product_expired: bool,
    pub product_id: u128,
}
