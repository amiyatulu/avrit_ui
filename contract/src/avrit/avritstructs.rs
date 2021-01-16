use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};



#[derive(Default, BorshDeserialize, BorshSerialize)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub profile_hash: String, //IPFS Hash
    pub kyc_done: bool,
    pub username: String,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Review {
    pub product_id: u128,
    pub user_id: u128,
    pub rating: u8,
    pub review_hash: String, //IPFS Hash
}


#[derive(Default, BorshDeserialize, BorshSerialize)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub user_id: u128,
    pub product_details_hash: String, //IPFS Hash
    pub product_type: String,
    pub product_expired: bool,
    pub product_id: u128,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommentProduct {
    pub product_id: u128,
    pub user_id: u128,
    pub comment_hash: String,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommentReview {
    pub review_id: u128,
    pub user_id: u128,
    pub comment_hash: String,
}