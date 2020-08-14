/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use chrono::{Duration, NaiveDateTime};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{TreeMap, Vector};
use near_sdk::wee_alloc;
use near_sdk::{env, near_bindgen};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CommitRevealElection {
    choice1: String,
    choice2: String,
    votes_for_choice1: u128,
    votes_for_choice2: u128,
    commit_phase_end_time: String,
    number_of_votes_cast: u128,
    vote_commits: Vector<String>,
    vote_statuses: TreeMap<String, String>,
}

impl Default for CommitRevealElection {
    fn default() -> Self {
        panic!("Please intialize the contract first")
    }
}

#[near_bindgen]
impl CommitRevealElection {
    #[init]
    pub fn new(commit_phase_length_in_secs: u128, choice1: String, choice2: String) -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        if commit_phase_length_in_secs < 20 {
            panic!("Commit phase length can't be less than 20 secs");
        }
        let timestamp = env::block_timestamp();
        println!("{}, timestamp", timestamp);
        let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        println!("{}, datetime", naive);
        let seconds = Duration::seconds(commit_phase_length_in_secs as i64);
        let endtime = naive + seconds;
        println!("{}, time after addition", endtime);

        let commitreveal = Self {
            choice1: choice1,
            choice2: choice2,
            votes_for_choice1: 0,
            votes_for_choice2: 0,
            commit_phase_end_time: endtime.to_string(),
            number_of_votes_cast: 0,
            vote_commits: Vector::new(b"60545a71-8aba-49bc-b923-6cd3049ce264".to_vec()),
            vote_statuses: TreeMap::new(b"1e443a7b-e7de-4e26-b4d6-9a8f6aa92076".to_vec()),
        };
        commitreveal
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_timstamp() -> u64 {
        let now: DateTime<Utc> = Utc::now();
        now.timestamp() as u64
    }

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: get_timstamp(),
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn contract_test() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = CommitRevealElection::new(120, "choice1".to_owned(), "choice2".to_owned());
    }
}
