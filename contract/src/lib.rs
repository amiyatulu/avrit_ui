pub mod avrit;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {

    use super::*;
    use avrit::Avrit;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use rand::Rng;
    use near_sdk::{ AccountId};

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

    fn bob() -> AccountId {
        "bob.near".to_string()
    }

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(), // The id of the account that owns the current contract
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2], // The public key of the account that did the signing.
            predecessor_account_id: "carol_near".to_string(), // The id of the account that was the previous contract in the chain of cross-contract calls. If this is the first contract, it is equal to signer_account_id.
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: rand_vector(),
            is_view,
            epoch_height: 0,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn profile() {
        let context = get_context(vec![], false);
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
        contract.create_product(
            "Product1xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
        );

        contract.update_product(1, "ProductupdatexeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned());
        let product = contract.get_product(1);
        assert_eq!("ProductupdatexeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(), product.product_details_hash);
        contract.create_product(
            "Product2xeV32S2VoyUnqJsRRCh75F1fP2AeomVq2Ury2fTt9V4p".to_owned(),
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
}
