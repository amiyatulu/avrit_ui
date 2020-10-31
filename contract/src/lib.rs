use near_sdk::wee_alloc;

pub mod avrit;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use avrit::{Avrit, STORAGE_PRICE_PER_BYTE};
    use near_sdk::MockedBlockchain;
    use near_sdk::{env, testing_env, AccountId, Balance, VMContext};
    use rand::Rng;

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
        let contract = Avrit::new(bob(), total_supply.into());
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
            let _contract = Avrit::new(bob(), total_supply.into());
        }
        Avrit::new(bob(), total_supply.into());
    }

    #[test]
    fn test_transfer_to_a_different_account_works() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        let mut contract = Avrit::new(carol(), total_supply.into());
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.dec_allowance(carol(), (total_supply / 2).into());
    }

    #[test]
    fn test_decrement_allowance_after_allowance_was_saturated() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        let mut contract = Avrit::new(carol(), total_supply.into());
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
    fn profile() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(bob(), total_supply.into());
        let hash_string = "QmZeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4z".to_owned();
        let hash_string2 = hash_string.clone();
        contract.create_profile(hash_string);
        let profile_hash = contract.get_profile_hash();
        assert_eq!(hash_string2, profile_hash);
        contract.create_profile("QmxeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned());
        let profile_hash = contract.get_profile_hash();
        assert_eq!(
            "QmxeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            profile_hash
        );
        contract.create_product("Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned());

        contract.update_product(
            1,
            "ProductupdatexeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );
        let product = contract.get_product(1);
        assert_eq!(
            "ProductupdatexeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            product.product_details_hash
        );
        contract.create_product("Product2xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned());
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
    fn product_bounty() {
        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        contract.create_product("Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned());
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
        let get_bounty = contract.get_product_bounty(1);
        assert_eq!(10, get_bounty.get(0).unwrap());
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
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        contract.create_product("Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned());
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

    #[test]
    fn draw_juror() {

        let mut context = get_context(carol());
        testing_env!(context.clone());
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Avrit::new(carol(), total_supply.into());
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
        contract.create_product("Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned());
        let product = contract.get_product(1);
        assert_eq!(
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
            product.product_details_hash
        );
        contract.create_review(
            1,
            "Review1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );
        
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

        let (contract, context) = apply_jurors_for_test_function(
            1,
            "juror1".to_owned(),
            60,
            contract,
            context.clone(),
        );
        let (contract, context) = apply_jurors_for_test_function(
            1,
            "juror2".to_owned(),
            40,
            contract,
            context.clone(),
        );
        let (contract, context) = apply_jurors_for_test_function(
            1,
            "juror3".to_owned(),
            30,
            contract,
            context.clone(),
        );
        let (contract, context) = apply_jurors_for_test_function(
            1,
            "juror4".to_owned(),
            20,
            contract,
            context.clone(),
        );
        let (mut contract, mut context) = apply_jurors_for_test_function(
            1,
            "juror5".to_owned(),
            20,
            contract,
            context.clone(),
        );

        context.random_seed = rand_vector();
        testing_env!(context.clone());
        contract.draw_jurors(1);
        let jurylist = contract.get_selected_jurors(1);
        let four = jurylist.contains(&4);
        println!("{:?}", four);
        let two = jurylist.contains(&2);
        println!("{:?}", two);
        let three = jurylist.contains(&3);
        println!("{:?}", three);
        let seven = jurylist.contains(&7);
        println!("{:?}", seven);
        let five = jurylist.contains(&5);
        println!("{:?}", five);
        let six = jurylist.contains(&6);
        println!("{:?}", six);
        let ten = jurylist.contains(&10);
        println!("{:?}", ten);
    }
}
