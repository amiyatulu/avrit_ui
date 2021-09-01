use avrit::avrit::AvritContract as Contract;
use avrit::avrit::Product;
use chrono::{DateTime, Utc};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::Balance;
use near_sdk_sim::{
    call, deploy, init_simulator, to_yocto, view, ContractAccount, ExecutionResult, UserAccount,
    DEFAULT_GAS,
};
use rand::Rng;
use sha3::{Digest, Keccak256};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    AVRIT_WASM_BYTES => "res/avrit.wasm"
}

const CONTRACT_ID: &str = "avrit";

pub fn register_user(user: &near_sdk_sim::UserAccount) {
    user.call(
        CONTRACT_ID.to_string(),
        "storage_deposit",
        &json!({
            "account_id": user.valid_account_id()
        })
        .to_string()
        .into_bytes(),
        near_sdk_sim::DEFAULT_GAS / 2,
        near_sdk::env::storage_byte_cost() * 125, // attached deposit
    )
    .assert_success();
}

fn get_timestamp() -> u64 {
    let now: DateTime<Utc> = Utc::now();
    now.timestamp() as u64 * 1000000000
}

fn get_timestamp_add(add: u64) -> u64 {
    let now: DateTime<Utc> = Utc::now();
    now.timestamp() as u64 * 1000000000 + add
}

fn draw_time_add() -> u64 {
    1296000 * 1000000000
}

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

pub fn deploy_avrit() -> (UserAccount, UserAccount, ContractAccount<Contract>) {
    let root = init_simulator(None);
    let owner = root.create_user("alice".to_string(), to_yocto("10000000"));
    let avrit_contract = deploy!(
        contract: Contract,
        contract_id: CONTRACT_ID,
        bytes: &AVRIT_WASM_BYTES,
        signer_account: root,
        deposit: near_sdk_sim::STORAGE_AMOUNT, // Deposit required to cover contract storage.
        gas: near_sdk_sim::DEFAULT_GAS,
        init_method: new(owner.valid_account_id().as_ref().to_string(), 10000000000.into())
    );
    (root, owner, avrit_contract)
}

#[test]
pub fn test_ft_transfer() {
    let (root, owner, avrit_contract) = deploy_avrit();
    let owner_balance: U128 =
        view!(avrit_contract.ft_balance_of(owner.valid_account_id())).unwrap_json();
    assert_eq!(owner_balance.0, 200000000);
    let bob = root.create_user("bob".to_string(), to_yocto("1000000"));
    call!(
        bob,
        avrit_contract.storage_deposit(Some(bob.valid_account_id()), None),
        deposit = to_yocto("100")
    )
    .assert_success();
    call!(
        owner,
        avrit_contract.ft_transfer(bob.valid_account_id(), 5000.into(), None),
        deposit = 1
    )
    .assert_success();
    let bob_balance: U128 =
        view!(avrit_contract.ft_balance_of(bob.valid_account_id())).unwrap_json();
    assert_eq!(bob_balance.0, 5000);
}

fn create_product() -> (
    UserAccount,
    UserAccount,
    ContractAccount<Contract>,
    UserAccount,
    U128,
) {
    let (root, owner, avrit_contract) = deploy_avrit();
    let bob = root.create_user("bob".to_string(), to_yocto("100"));
    let hash_string = "QmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();
    let hash_string2 = hash_string.clone();
    call!(bob, avrit_contract.create_profile(hash_string)).assert_success();
    let user_id: U128 = view!(avrit_contract.get_user_id_js(&bob.account_id())).unwrap_json();
    let profile_hash: String = call!(bob, avrit_contract.get_profile_hash()).unwrap_json();
    let profile_hash2: String =
        view!(avrit_contract.get_profile_hash_from_id(user_id)).unwrap_json();

    assert_eq!(hash_string2, profile_hash);
    assert_eq!(hash_string2, profile_hash2);
    call!(
        bob,
        avrit_contract.create_product(
            "PPmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned(),
            "ev".to_owned()
        )
    )
    .assert_success();
    call!(
        bob,
        avrit_contract.setup_nft_price_and_token_count(1.into(), to_yocto("10").into(), 5.into())
    )
    .assert_success();

    (root, owner, avrit_contract, bob, user_id)
}

#[test]
fn create_profile_and_product() {
    let (root, owner, avrit_contract, bob, user_id) = create_product();
}

#[test]
#[should_panic(expected = "NFT price already set")]
fn nft_price_set() {
    let (root, owner, avrit_contract, bob, user_id) = create_product();
    call!(
        bob,
        avrit_contract.setup_nft_price_and_token_count(1.into(), to_yocto("10").into(), 5.into())
    )
    .assert_success();
}

#[test]
#[should_panic(expected = "Price should be equal to deposit")]
fn buy_nft() {
    let (root, owner, avrit_contract, bob, user_id) = create_product();
    let charlie = root.create_user("charlie".to_string(), to_yocto("10000"));

    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("100")
    )
    .assert_success();
}

#[test]
fn buy_nft2() {
    let (root, owner, avrit_contract, bob, user_id) = create_product();
    let charlie = root.create_user("charlie".to_string(), to_yocto("20"));
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();
}

#[test]
fn get_owner_incentives() {
    let (root, owner, avrit_contract, bob, user_id) = create_product();
    let charlie = root.create_user("charlie".to_string(), to_yocto("20"));
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();

    let owner_incentives: U128 = view!(avrit_contract.get_owner_incentives(user_id)).unwrap_json();
    let owner_incentives_int: u128 = owner_incentives.into();
    assert_eq!(to_yocto("20"), owner_incentives_int);
}

#[test]
#[should_panic(expected = "No incentives to withdraw")]
fn withdraw_owner_incentives_panic() {
    let (root, owner, avrit_contract, bob, user_id) = create_product();
    let charlie = root.create_user("charlie".to_string(), to_yocto("20"));
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();

    let owner_incentives: U128 = view!(avrit_contract.get_owner_incentives(user_id)).unwrap_json();
    let owner_incentives_int: u128 = owner_incentives.into();

    assert_eq!(to_yocto("20"), owner_incentives_int);

    let _bob_near_balance = bob.account().unwrap().amount;

    let hash_string = "charlieVoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();

    call!(charlie, avrit_contract.create_profile(hash_string)).assert_success();

    call!(charlie, avrit_contract.withdraw_product_owner_incentives()).assert_success();

    let owner_incentives: U128 = view!(avrit_contract.get_owner_incentives(user_id)).unwrap_json();
    let owner_incentives_int: u128 = owner_incentives.into();

    assert_eq!(0, owner_incentives_int);
}


#[test]
#[should_panic(expected = "You have no incentives to withdraw")]
fn withdraw_owner_incentives_panic2() {
    let (root, owner, avrit_contract, bob, user_id) = create_product();
    let charlie = root.create_user("charlie".to_string(), to_yocto("20"));
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();

    let owner_incentives: U128 = view!(avrit_contract.get_owner_incentives(user_id)).unwrap_json();
    let owner_incentives_int: u128 = owner_incentives.into();

    assert_eq!(to_yocto("20"), owner_incentives_int);

    let _bob_near_balance = bob.account().unwrap().amount;

    call!(bob, avrit_contract.withdraw_product_owner_incentives()).assert_success();

    let owner_incentives: U128 = view!(avrit_contract.get_owner_incentives(user_id)).unwrap_json();
    let owner_incentives_int: u128 = owner_incentives.into();

    assert_eq!(0, owner_incentives_int);
    call!(bob, avrit_contract.withdraw_product_owner_incentives()).assert_success();
}


#[test]
fn withdraw_owner_incentives() {
    let (root, owner, avrit_contract, bob, user_id) = create_product();
    let charlie = root.create_user("charlie".to_string(), to_yocto("20"));
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();
    call!(
        charlie,
        avrit_contract.buy_nft(1.into()),
        deposit = to_yocto("10")
    )
    .assert_success();

    let owner_incentives: U128 = view!(avrit_contract.get_owner_incentives(user_id)).unwrap_json();
    let owner_incentives_int: u128 = owner_incentives.into();

    assert_eq!(to_yocto("20"), owner_incentives_int);

    let _bob_near_balance = bob.account().unwrap().amount;

    call!(bob, avrit_contract.withdraw_product_owner_incentives()).assert_success();

    let owner_incentives: U128 = view!(avrit_contract.get_owner_incentives(user_id)).unwrap_json();
    let owner_incentives_int: u128 = owner_incentives.into();

    assert_eq!(0, owner_incentives_int);
}

// #[test]
// fn profile() {
//         let (root, _owner, avrit_contract) = deploy_avrit();
//         let bob = root.create_user("bob".to_string(), to_yocto("100"));
//         let hash_string = "QmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();
//         let hash_string2 = hash_string.clone();
//         call!(bob, avrit_contract.create_profile(hash_string)).assert_success();
//         let user_id:U128= view!(avrit_contract.get_user_id_js(&bob.account_id())).unwrap_json();
//         let profile_hash:String = call!(bob, avrit_contract.get_profile_hash()).unwrap_json();
//         let profile_hash2:String = view!(avrit_contract.get_profile_hash_from_id(user_id)).unwrap_json();

//         assert_eq!(hash_string2, profile_hash);
//         assert_eq!(hash_string2, profile_hash2);
//         call!(bob, avrit_contract.create_product(
//             "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
//             "OA".to_owned()
//         )).assert_success();
//         call!(bob, avrit_contract.update_product(
//             1.into(),
//             "ProductupdatexeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned()

//         )).assert_success();

//         // let res = view!(avrit_contract.get_product_js(1.into()));
//         // assert!(res.is_ok());

//         // // let product = contract.get_product(1);
//         // assert_eq!(
//         //     "ProductupdatexeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
//         //     product.product_details_hash
//         // );
//         // contract.create_product(
//         //     "Product2xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
//         //     "OA".to_owned(),
//         // );
//         // let ids = contract.get_products_of_user(0, 5);
//         // println!(">>>ids>{:?}<", ids);
//         // contract.create_review(
//         //     1,
//         //     "Review1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
//         //     5
//         // );
//         // contract.create_review(
//         //     2,
//         //     "Review1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
//         //     3
//         // );
//     }
