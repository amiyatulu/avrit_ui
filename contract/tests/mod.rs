use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::Balance;
use near_sdk_sim::{
    call, deploy, init_simulator, to_yocto, view, ContractAccount, ExecutionResult, UserAccount,
    DEFAULT_GAS,
};
use avrit::avrit::AvritContract as Contract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    AVRIT_WASM_BYTES => "res/avrit.wasm"
}

const CONTRACT_ID: &str = "avrit";
const LEGACY_BYTE_COST: Balance = 10_000_000_000_000_000_000;

const STORAGE_BALANCE: Balance = 125 * LEGACY_BYTE_COST;


fn deploy_avrit() -> (UserAccount, UserAccount, ContractAccount<Contract>) {
    let root = init_simulator(None);
    let owner = root.create_user("alice".to_string(), to_yocto("100"));
    let avrit_contract = deploy!(
        contract: Contract,
        contract_id: CONTRACT_ID.to_string(),
        bytes: &AVRIT_WASM_BYTES,
        signer_account: root,
        init_method: new(owner.valid_account_id().as_ref().to_string(), 10000000000.into())
    );
    (root, owner, avrit_contract)
}



#[test]
pub fn test_ft_transfer() {
    let (root, owner, avrit_contract) = deploy_avrit();
    let  owner_balance: U128 = view!(avrit_contract.ft_balance_of(owner.valid_account_id())).unwrap_json();
    assert_eq!(owner_balance.0, 10000000000);

    let bob = root.create_user("bob".to_string(), to_yocto("100"));
    call!(bob, avrit_contract.register_account(bob.valid_account_id())).assert_success();
    call!(owner, avrit_contract.ft_transfer(bob.valid_account_id(), 5000.into(), None),
    deposit = 1).assert_success();

    let bob_balance: U128 = view!(avrit_contract.ft_balance_of(bob.valid_account_id())).unwrap_json();
    assert_eq!(bob_balance.0, 5000);
}
