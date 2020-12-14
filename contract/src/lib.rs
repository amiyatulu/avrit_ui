use near_sdk::wee_alloc;

pub mod avrit;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use avrit::{Avrit, STORAGE_PRICE_PER_BYTE};
    use chrono::{DateTime, Utc};
    use near_sdk::MockedBlockchain;
    use near_sdk::{env, testing_env, AccountId, Balance, VMContext};
    use rand::Rng;
    use sha3::{Digest, Keccak256};

    fn get_timestamp() -> u64 {
        let now: DateTime<Utc> = Utc::now();
        now.timestamp() as u64
    }

    fn get_timestamp_add(add: u64) -> u64 {
        let now: DateTime<Utc> = Utc::now();
        now.timestamp() as u64 + add
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

    fn alice() -> AccountId {
        "alice.near".to_string()
    }
    fn bob() -> AccountId {
        "bob.near".to_string()
    }
    fn carol() -> AccountId {
        "carol.near".to_string()
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: alice(),
            signer_account_id: bob(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1_000_000_000_000_000_000_000_000_000u128,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn test_initialize_new_token() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 1_000_000_000_000_000u128;
        let contract = Avrit::new(bob(), total_supply.into(), total_supply.into());
        assert_eq!(contract.get_total_supply().0, total_supply);
        assert_eq!(contract.get_balance(bob()).0, total_supply);
    }

    #[test]
    #[should_panic]
    fn test_initialize_new_token_twice_fails() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 1_000_000_000_000_000u128;
        {
            let _contract = Avrit::new(bob(), total_supply.into(), total_supply.into());
        }
        Avrit::new(bob(), total_supply.into(), total_supply.into());
    }

    #[test]
    fn test_transfer_to_a_different_account_works() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.storage_usage = env::storage_usage();

        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        testing_env!(context.clone());
        let transfer_amount = total_supply / 3;
        contract.transfer(bob(), transfer_amount.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(
            contract.get_balance(carol()).0,
            (total_supply - transfer_amount)
        );
        assert_eq!(contract.get_balance(bob()).0, transfer_amount);
    }

    #[test]
    #[should_panic(expected = "The new owner should be different from the current owner")]
    fn test_transfer_to_self_fails() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.storage_usage = env::storage_usage();

        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        testing_env!(context.clone());
        let transfer_amount = total_supply / 3;
        contract.transfer(carol(), transfer_amount.into());
    }

    #[test]
    #[should_panic(expected = "Can not increment allowance for yourself")]
    fn test_increment_allowance_to_self_fails() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.inc_allowance(carol(), (total_supply / 2).into());
    }

    #[test]
    #[should_panic(expected = "Can not decrement allowance for yourself")]
    fn test_decrement_allowance_to_self_fails() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.dec_allowance(carol(), (total_supply / 2).into());
    }

    #[test]
    fn test_decrement_allowance_after_allowance_was_saturated() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.dec_allowance(bob(), (total_supply / 2).into());
        assert_eq!(contract.get_allowance(carol(), bob()), 0.into())
    }

    #[test]
    fn test_increment_allowance_does_not_overflow() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = std::u128::MAX;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.inc_allowance(bob(), total_supply.into());
        contract.inc_allowance(bob(), total_supply.into());
        assert_eq!(
            contract.get_allowance(carol(), bob()),
            std::u128::MAX.into()
        )
    }

    #[test]
    #[should_panic(
        expected = "The required attached deposit is 33100000000000000000000, but the given attached deposit is is 0"
    )]
    fn test_increment_allowance_with_insufficient_attached_deposit() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.attached_deposit = 0;
        testing_env!(context.clone());
        contract.inc_allowance(bob(), (total_supply / 2).into());
    }

    #[test]
    fn test_carol_escrows_to_bob_transfers_to_alice() {
        // Acting as carol
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.storage_usage = env::storage_usage();

        context.is_view = true;
        testing_env!(context.clone());
        assert_eq!(contract.get_total_supply().0, total_supply);

        let allowance = total_supply / 3;
        let transfer_amount = allowance / 3;
        context.is_view = false;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.inc_allowance(bob(), allowance.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(contract.get_allowance(carol(), bob()).0, allowance);

        // Acting as bob now
        context.is_view = false;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        context.predecessor_account_id = bob();
        testing_env!(context.clone());
        contract.transfer_from(carol(), alice(), transfer_amount.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(
            contract.get_balance(carol()).0,
            total_supply - transfer_amount
        );
        assert_eq!(contract.get_balance(alice()).0, transfer_amount);
        assert_eq!(
            contract.get_allowance(carol(), bob()).0,
            allowance - transfer_amount
        );
    }

    #[test]
    fn test_carol_escrows_to_bob_locks_and_transfers_to_alice() {
        // Acting as carol
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.storage_usage = env::storage_usage();

        context.is_view = true;
        testing_env!(context.clone());
        assert_eq!(contract.get_total_supply().0, total_supply);

        let allowance = total_supply / 3;
        let transfer_amount = allowance / 3;
        context.is_view = false;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.inc_allowance(bob(), allowance.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(contract.get_allowance(carol(), bob()).0, allowance);
        assert_eq!(contract.get_balance(carol()).0, total_supply);

        // Acting as bob now
        context.is_view = false;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        context.predecessor_account_id = bob();
        testing_env!(context.clone());
        contract.transfer_from(carol(), alice(), transfer_amount.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(
            contract.get_balance(carol()).0,
            (total_supply - transfer_amount)
        );
        assert_eq!(contract.get_balance(alice()).0, transfer_amount);
        assert_eq!(
            contract.get_allowance(carol(), bob()).0,
            allowance - transfer_amount
        );
    }

    #[test]
    fn test_self_allowance_set_for_refund() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.storage_usage = env::storage_usage();

        let initial_balance = context.account_balance;
        let initial_storage = context.storage_usage;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.inc_allowance(bob(), (total_supply / 2).into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();
        assert_eq!(
            context.account_balance,
            initial_balance
                + Balance::from(context.storage_usage - initial_storage) * STORAGE_PRICE_PER_BYTE
        );

        let initial_balance = context.account_balance;
        let initial_storage = context.storage_usage;
        testing_env!(context.clone());
        context.attached_deposit = 0;
        testing_env!(context.clone());
        contract.dec_allowance(bob(), (total_supply / 2).into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();
        assert!(context.storage_usage < initial_storage);
        assert!(context.account_balance < initial_balance);
        assert_eq!(
            context.account_balance,
            initial_balance
                - Balance::from(initial_storage - context.storage_usage) * STORAGE_PRICE_PER_BYTE
        );
    }

    #[test]
    fn test_admin() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(bob(), total_supply.into(), total_supply.into());
        let owner = contract.get_owner();
        assert_eq!(owner, bob());
        context.predecessor_account_id = bob();
        testing_env!(context.clone());
        contract.assert_owner();
        contract.change_owner(alice());
        let owner2 = contract.get_owner();
        assert_eq!(owner2, alice());
    }

    #[test]
    fn profile() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(bob(), total_supply.into(), total_supply.into());
        let hash_string = "QmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();
        let hash_string2 = hash_string.clone();
        contract.create_profile(hash_string);
        let profile_hash = contract.get_profile_hash();
        assert_eq!(hash_string2, profile_hash);
        contract.create_product(
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            "OA".to_owned(),
        );

        contract.update_product(
            1,
            "ProductupdatexeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );
        let product = contract.get_product(1);
        assert_eq!(
            "ProductupdatexeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            product.product_details_hash
        );
        contract.create_product(
            "Product2xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            "OA".to_owned(),
        );
        let ids = contract.get_products_of_user();
        println!("{:?}", ids);
        contract.create_review(
            1,
            "Review1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );
        contract.create_review(
            2,
            "Review1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );
    }

    #[test]
    fn update_profiles() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(bob(), total_supply.into(), total_supply.into());
        let hash_string = "QmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();
        let hash_string2 = hash_string.clone();
        contract.create_profile(hash_string);
        let profile_hash = contract.get_profile_hash();
        assert_eq!(hash_string2, profile_hash);
        contract.update_profile("QmxeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned());
        let profile_hash = contract.get_profile_hash();
        assert_eq!(
            "QmxeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            profile_hash
        );
        let updated_id = contract.get_update_user_id_time_counter();
        assert_eq!(1, updated_id.0);
        let user_id = contract.get_update_user_ids(updated_id.0);
        assert_eq!(1, user_id.0);
        context.predecessor_account_id = bob();
        testing_env!(context.clone());
        contract.set_update_user_id_time_counter_zero();
        let updated_id = contract.get_update_user_id_time_counter();
        assert_eq!(0, updated_id.0);
    }

    #[test]
    fn product_bounty() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        testing_env!(context.clone());
        contract.transfer(alice(), 150.into());
        assert_eq!(150, contract.get_balance(alice()).0);
        context.predecessor_account_id = alice();
        testing_env!(context.clone());
        let hash_string = "QmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();
        let hash_string2 = hash_string.clone();
        contract.create_profile(hash_string);
        let profile_hash = contract.get_profile_hash();
        assert_eq!(hash_string2, profile_hash);
        contract.create_product(
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            "OA".to_owned(),
        );
        let product = contract.get_product(1);
        assert_eq!(
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            product.product_details_hash
        );
        let initialtotalsupply = contract.get_total_supply().0;
        assert_eq!(initialtotalsupply, total_supply);
        // println!(">>>>>>intial supply{}<<<<<<<<<<", initialtotalsupply);
        contract.add_product_bounty(10, 1);
        let totalsupply_after_bounty = contract.get_total_supply().0;
        assert_eq!(totalsupply_after_bounty, total_supply - 10);
        assert_eq!(150 - 10, contract.get_balance(alice()).0);
        // let get_bounty = contract.get_product_bounty(1);
        // assert_eq!(10, get_bounty.get(0).unwrap());
        contract.add_product_bounty(15, 1);
        let totalsupply_after_bounty2 = contract.get_total_supply().0;
        assert_eq!(totalsupply_after_bounty2, totalsupply_after_bounty - 5);
        assert_eq!(150 - 10 - 5, contract.get_balance(alice()).0);
    }

    #[test]
    fn review_bounty() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        testing_env!(context.clone());
        contract.transfer(alice(), 150.into());
        assert_eq!(150, contract.get_balance(alice()).0);
        context.predecessor_account_id = alice();
        testing_env!(context.clone());
        let hash_string = "QmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();
        let hash_string2 = hash_string.clone();
        contract.create_profile(hash_string);
        let profile_hash = contract.get_profile_hash();
        assert_eq!(hash_string2, profile_hash);
        contract.create_product(
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            "OA".to_owned(),
        );
        let product = contract.get_product(1);
        assert_eq!(
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            product.product_details_hash
        );
        contract.create_review(
            1,
            "Review1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );
        let review = contract.get_review(1);
        assert_eq!(
            "Review1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            review.review_hash
        );
        contract.add_review_bounty(25, 1);
        let totalsupply_after_bounty = contract.get_total_supply().0;
        assert_eq!(totalsupply_after_bounty, total_supply - 25);
        assert_eq!(150 - 25, contract.get_balance(alice()).0);
    }

    fn create_a_user(
        username: AccountId,
        predecessoraccountid: AccountId,
        profilehash: String,
        mut contract: Avrit,
        mut context: VMContext,
    ) -> (Avrit, VMContext) {
        context.predecessor_account_id = username.clone();
        testing_env!(context.clone());
        contract.create_profile(profilehash);
        context.predecessor_account_id = predecessoraccountid;
        testing_env!(context.clone());
        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        contract.transfer(username, 150.into());
        (contract, context)
    }

    fn apply_jurors_for_test_function(
        reviewerid: u128,
        predecessoraccountid: AccountId,
        stake: u128,
        mut contract: Avrit,
        mut context: VMContext,
    ) -> (Avrit, VMContext) {
        context.predecessor_account_id = predecessoraccountid;
        testing_env!(context.clone());
        contract.apply_jurors(reviewerid, stake);
        (contract, context)
    }

    fn draw_juror_function() -> (Avrit, VMContext) {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into(), total_supply.into());
        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        testing_env!(context.clone());
        contract.transfer(alice(), 150.into());
        assert_eq!(150, contract.get_balance(alice()).0);
        context.predecessor_account_id = alice();
        testing_env!(context.clone());
        let hash_string = "QmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();
        let hash_string2 = hash_string.clone();
        contract.create_profile(hash_string);
        let profile_hash = contract.get_profile_hash();
        assert_eq!(hash_string2, profile_hash);
        contract.create_product(
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            "oa".to_owned(),
        );
        let product = contract.get_product(1);
        assert_eq!(
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            product.product_details_hash
        );
        context.block_timestamp = get_timestamp();
        testing_env!(context.clone());
        contract.create_review(
            1,
            "Review1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );
        context.predecessor_account_id = carol();
        testing_env!(context.clone());
        contract.add_review_bounty(15,1);
        let (contract, context) = create_a_user(
            "juror1".to_owned(),
            carol(),
            "juror1######XXXXX".to_owned(),
            contract,
            context,
        );
        let (contract, context) = create_a_user(
            "juror2".to_owned(),
            carol(),
            "juror2######XXXXX".to_owned(),
            contract,
            context,
        );
        let (contract, context) = create_a_user(
            "juror3".to_owned(),
            carol(),
            "juror3######XXXXX".to_owned(),
            contract,
            context,
        );
        let (contract, context) = create_a_user(
            "juror4".to_owned(),
            carol(),
            "juror4######XXXXX".to_owned(),
            contract,
            context,
        );
        let (contract, context) = create_a_user(
            "juror5".to_owned(),
            carol(),
            "juror5######XXXXX".to_owned(),
            contract,
            context,
        );

        let (contract, context) =
            apply_jurors_for_test_function(1, "juror1".to_owned(), 60, contract, context.clone());
        let (contract, context) =
            apply_jurors_for_test_function(1, "juror2".to_owned(), 40, contract, context.clone());
        let (contract, context) =
            apply_jurors_for_test_function(1, "juror3".to_owned(), 30, contract, context.clone());
        let (contract, context) =
            apply_jurors_for_test_function(1, "juror4".to_owned(), 20, contract, context.clone());
        let (mut contract, mut context) =
            apply_jurors_for_test_function(1, "juror5".to_owned(), 20, contract, context.clone());

        context.random_seed = rand_vector();
        context.predecessor_account_id = carol();
        context.block_timestamp = get_timestamp_add(1296000);
        testing_env!(context.clone());
        contract.set_jury_count(5);
        contract.draw_jurors(1, 5);
        (contract, context)
    }

    #[test]
    fn test_draw_juror() {
        // contract.draw_jurors(1, 5);
        let (contract, _context) = draw_juror_function();

        let time = contract.get_juror_selection_time_js(&1);
        println!(">>>>>>time{}<<<<<<<<<", time.0);
        // let jurylist = contract.get_selected_jurors(1);
        // let four = jurylist.contains(&4);
        // println!("{:?}", four);
        // let two = jurylist.contains(&2);
        // println!("{:?}", two);
        // let three = jurylist.contains(&3);
        // println!("{:?}", three);
        // let seven = jurylist.contains(&7);
        // println!("{:?}", seven);
        // let five = jurylist.contains(&5);
        // println!("{:?}", five);
        // let six = jurylist.contains(&6);
        // println!("{:?}", six);
        // let ten = jurylist.contains(&10);
        // println!("{:?}", ten);
    }

    fn commit_votes_function(
        mut contract: Avrit,
        mut context: VMContext,
        vote: String,
        predecessor_account: AccountId,
        reviewer_id: u128,
    ) -> (Avrit, VMContext) {
        let mut hasher = Keccak256::new();
        hasher.update(vote.as_bytes());
        let result = hasher.finalize();
        let commit = format!("{:x}", result);
        context.block_timestamp = get_timestamp();
        context.predecessor_account_id = predecessor_account;
        testing_env!(context.clone());
        contract.commit_vote(reviewer_id, commit);
        (contract, context)
    }

    fn reveal_votes_function(
        mut contract: Avrit,
        mut context: VMContext,
        vote: String,
        predecessor_account: AccountId,
        reviewer_id: u128,
    ) -> (Avrit, VMContext) {
        context.block_timestamp = get_timestamp_add(1296000 + 2592000);
        context.predecessor_account_id = predecessor_account;
        testing_env!(context.clone());
        let mut hasher = Keccak256::new();
        hasher.update(vote.as_bytes());
        let result = hasher.finalize();
        let commit = format!("{:x}", result);
        contract.reveal_vote(reviewer_id, vote, commit.clone());
        (contract, context)
    }

    #[test]
    #[should_panic(expected = "You are not juror of the review")]
    fn test_not_a_juror_commit_vote() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1password".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (_contract, _context) =
            commit_votes_function(contract, context, "1password".to_owned(), alice(), 1);
    }

    #[test]
    #[should_panic(expected = "This vote is already commited")]
    fn test_two_same_commit_vote() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1password".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (_contract, _context) = commit_votes_function(
            contract,
            context,
            "1password".to_owned(),
            "juror2".to_owned(),
            1,
        );
    }

    #[test]
    #[should_panic(expected = "Voter has already commited")]
    fn test_same_juror_commit_vote() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1password".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (_contract, _context) = commit_votes_function(
            contract,
            context,
            "1password".to_owned(),
            "juror1".to_owned(),
            1,
        );
    }

    #[test]
    fn test_vote_commit() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (_contract, _context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
    }

    #[test]
    #[should_panic(expected = "Vote with this commit is not present")]
    fn test_vote_reveal_not_present() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let (_contract, _context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror".to_owned(),
            "juror1".to_owned(),
            1,
        );
    }

    #[test]
    #[should_panic(expected = "You are not juror of the review")]
    fn test_vote_reveal_not_juror() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let (_contract, _context) =
            reveal_votes_function(contract, context, "1passwordjuror1".to_owned(), alice(), 1);
    }
    #[test]
    #[should_panic(expected = "The juror has already been revealed a vote.")]
    fn test_vote_reveal_again() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (_contract, _context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror2".to_owned(),
            1,
        );
    }

    #[test]
    #[should_panic(expected = "The vote has be already revealed and added.")]
    fn test_vote_reveal_already() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (_contract, _context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror2".to_owned(),
            1,
        );
    }

    #[test]
    fn test_vote_reveal() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );

        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (contract, _context) = reveal_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let data_true = contract.get_true_count_js(1);
        assert_eq!(data_true.0, 2);
        let data_false = contract.get_false_count_js(1);
        assert_eq!(data_false.0, 3);
    }

    #[test]
    fn test_winning_decisions() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );

        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (contract, mut context) = reveal_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let data_true = contract.get_true_count_js(1);
        assert_eq!(data_true.0, 2);
        let data_false = contract.get_false_count_js(1);
        assert_eq!(data_false.0, 3);
        context.block_timestamp = get_timestamp_add(1296000 + 2592000 + 1296001);
        testing_env!(context.clone());
        let winingdecision = contract.get_winning_decision(1);
        assert_eq!(0, winingdecision);
    }

    #[test]

    fn test_incentives_distribution() {
        let (contract, context) = draw_juror_function();
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "1passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (contract, context) = commit_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror1".to_owned(),
            "juror1".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "1passwordjuror2".to_owned(),
            "juror2".to_owned(),
            1,
        );
        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "0passwordjuror3".to_owned(),
            "juror3".to_owned(),
            1,
        );

        let (contract, context) = reveal_votes_function(
            contract,
            context,
            "0passwordjuror4".to_owned(),
            "juror4".to_owned(),
            1,
        );
        let (mut contract, mut context) = reveal_votes_function(
            contract,
            context,
            "0passwordjuror5".to_owned(),
            "juror5".to_owned(),
            1,
        );
        let data_true = contract.get_true_count_js(1);
        assert_eq!(data_true.0, 2);
        let data_false = contract.get_false_count_js(1);
        assert_eq!(data_false.0, 3);
        context.block_timestamp = get_timestamp_add(1296000 + 2592000 + 1296001);
        testing_env!(context.clone());
        let winingdecision = contract.get_winning_decision(1);
        assert_eq!(0, winingdecision);
        assert_eq!(90, contract.get_balance("juror1".to_owned()).0);
        context.predecessor_account_id = "juror1".to_owned();
        testing_env!(context.clone());
        contract.incentives_distribution(1);
        // Stake was 60, so incentive became 60^0.8 = 27
        assert_eq!(90 + 27, contract.get_balance("juror1".to_owned()).0);
        // println!(">>>>>>>juror4balance={}<<<<<<<<<<<", contract.get_balance("juror4".to_owned()).0);
        assert_eq!(130, contract.get_balance("juror4".to_owned()).0);
        context.predecessor_account_id = "juror4".to_owned();
        testing_env!(context.clone());
        contract.incentives_distribution(1);
        // println!(">>>>>>>juror4balance_NEXT={}<<<<<<<<<<<", contract.get_balance("juror4".to_owned()).0);
        assert_eq!(130 + 20 + 10, contract.get_balance("juror4".to_owned()).0);
    }

    #[test]
    #[should_panic(expected = "Total supply should be less than or equal to cap.")]
    fn cap_less_than_total_supply() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 1_000_000_000_000_000u128;
        let _contract = Avrit::new(bob(), total_supply.into(), 1_000_000_000_000_0u128.into());
    }

    #[test]

    fn tax_collection_burn() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(bob(), total_supply.into(), total_supply.into());
        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        testing_env!(context.clone());
        context.predecessor_account_id = bob();
        testing_env!(context.clone());
        let transfer_amount = 500;
        contract.transfer(alice(), transfer_amount.into());
        let balance = contract.get_balance(alice());
        assert_eq!(500, balance.0);
        let balance2 = contract.get_balance(bob());
        assert_eq!(total_supply - 500, balance2.0);
        let total_supply1 = contract.get_total_supply();
        assert_eq!(total_supply, total_supply1.0);
        // println!(">>>>>>>>>>total_supply>{}<<<<<<<<<<<<<<<", total_supply.0);
        // println!(">>>>>>>>>>blanace2>{}<<<<<<<<<<<<<<<", balance2.0);
        contract.set_burn_percentage(5.5);
        let burn_percentage = contract.get_burn_percentage();
        assert_eq!(burn_percentage, 5.5);
        context.predecessor_account_id = alice();
        testing_env!(context.clone());
        contract.transfer("auro.near".to_string(), 300.into());
        let balance_after_tax = contract.get_balance("auro.near".to_string());
        assert_eq!(300 - 16, balance_after_tax.0);
        let total_supply2 = contract.get_total_supply();
        assert_eq!(total_supply - 16, total_supply2.0);
        // println!(">>>>>>>>>>total_supply>{}<<<<<<<<<<<<<<<", total_supply2.0);
    }

    #[test]
    fn tax_collection_saving_burn() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(bob(), total_supply.into(), total_supply.into());
        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        testing_env!(context.clone());
        context.predecessor_account_id = bob();
        testing_env!(context.clone());
        let transfer_amount = 500;
        contract.transfer(alice(), transfer_amount.into());
        let balance = contract.get_balance(alice());
        assert_eq!(500, balance.0);
        let balance2 = contract.get_balance(bob());
        assert_eq!(total_supply - 500, balance2.0);
        let total_supply1 = contract.get_total_supply();
        assert_eq!(total_supply, total_supply1.0);
        // println!(">>>>>>>>>>total_supply>{}<<<<<<<<<<<<<<<", total_supply.0);
        // println!(">>>>>>>>>>blanace2>{}<<<<<<<<<<<<<<<", balance2.0);
        contract.set_burn_percentage(5.5);
        contract.change_saving_id("saving.near".to_string());
        contract.set_saving_percentage(3.5);
        let burn_percentage = contract.get_burn_percentage();
        assert_eq!(burn_percentage, 5.5);
        let saving_percentage = contract.get_saving_percentage();
        assert_eq!(saving_percentage, 3.5);
        context.predecessor_account_id = alice();
        testing_env!(context.clone());
        contract.transfer("auro.near".to_string(), 300.into());
        let balance_after_tax = contract.get_balance("auro.near".to_string());
        assert_eq!(300 - 16 - 10, balance_after_tax.0);
        let total_supply2 = contract.get_total_supply();
        assert_eq!(total_supply - 16, total_supply2.0);
        let saving_balance = contract.get_balance("saving.near".to_string());
        assert_eq!(10, saving_balance.0);

    }

}
