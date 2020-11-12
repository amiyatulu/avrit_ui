use super::Avrit;
use chrono::{Duration, NaiveDateTime};
use near_sdk::collections::{LookupMap, LookupSet, TreeMap};
use near_sdk::{env, near_bindgen};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use sha3::{Digest, Keccak256};

pub fn get_rng(seed_vec: Vec<u8>) -> StdRng {
    let mut seed = [0u8; 32];
    let mut counter = 0;
    for v in seed_vec.iter() {
        seed[counter] = *v;
        counter += 1;
    }

    let rng: StdRng = SeedableRng::from_seed(seed);
    rng
}

#[near_bindgen]
impl Avrit {
    /// Apply Jurors with stake
    pub fn apply_jurors(&mut self, review_id: u128, stake: u128) {
        let account_id = env::predecessor_account_id();
        let singer_juror_user = self.get_user_id(&account_id);
        self.user_juror_stakes_store(
            account_id.clone(),
            singer_juror_user.clone(),
            review_id.clone(),
            stake.clone(),
        );
        self.user_juror_stakes_clone_store(
            singer_juror_user.clone(),
            review_id.clone(),
            stake.clone(),
        );
    }
    fn user_juror_stakes_store(
        &mut self,
        account_id: String,
        singer_juror_user: u128,
        review_id: u128,
        stake: u128,
    ) {
        let user_juror_stakes_option = self.user_juror_stakes.get(&review_id);
        match user_juror_stakes_option {
            Some(mut stake_entries) => {
                let stake_entries_option = stake_entries.get(&singer_juror_user);
                match stake_entries_option {
                    Some(stake) => {
                        if stake > 0 {
                            panic!("You have already staked")
                        } else {
                            stake_entries.insert(&singer_juror_user, &stake);
                            self.burn(&account_id, stake);
                            self.user_juror_stakes.insert(&review_id, &stake_entries);
                        }
                    }
                    None => {
                        stake_entries.insert(&singer_juror_user, &stake);
                        self.burn(&account_id, stake);
                        self.user_juror_stakes.insert(&review_id, &stake_entries);
                    }
                }
            }
            None => {
                let stakeidstring = format!(
                    "stakevoterid{}uniqueid{}",
                    review_id, self.juror_stake_unique_id
                );
                let stakeid = stakeidstring.to_string().into_bytes();
                let mut stake_entries = LookupMap::new(stakeid);
                stake_entries.insert(&singer_juror_user, &stake);
                self.burn(&account_id, stake);
                self.user_juror_stakes.insert(&review_id, &stake_entries);
            }
        }
    }

    fn user_juror_stakes_clone_store(
        &mut self,
        singer_juror_user: u128,
        review_id: u128,
        stake: u128,
    ) {
        let user_juror_stakes_option = self.user_juror_stakes_clone.get(&review_id);
        match user_juror_stakes_option {
            Some(mut stake_entries) => {
                let stake_entries_option = stake_entries.get(&singer_juror_user);
                match stake_entries_option {
                    Some(stake) => {
                        if stake > 0 {
                            panic!("You have already staked")
                        } else {
                            stake_entries.insert(&singer_juror_user, &stake);
                            self.user_juror_stakes_clone
                                .insert(&review_id, &stake_entries);
                        }
                    }
                    None => {
                        stake_entries.insert(&singer_juror_user, &stake);
                        self.user_juror_stakes_clone
                            .insert(&review_id, &stake_entries);
                    }
                }
            }
            None => {
                let stakeidstring = format!(
                    "stakevoteridclone{}uniqueid{}",
                    review_id, self.juror_stake_unique_id
                );
                self.juror_stake_unique_id += 1;
                let stakeid = stakeidstring.to_string().into_bytes();
                let mut stake_entries = TreeMap::new(stakeid);
                stake_entries.insert(&singer_juror_user, &stake);
                self.user_juror_stakes_clone
                    .insert(&review_id, &stake_entries);
            }
        }
    }

    pub fn draw_jurors(&mut self, review_id: u128, length: usize) {
        let selected_juror_option = self.selected_juror.get(&review_id);
        match selected_juror_option {
            Some(jurysetentries) => {
                self.draw_jurors_function(review_id, jurysetentries, length);
            }
            None => {
                let jurysetidstring = format!("jurysetid{}", review_id);
                let jurysetid = jurysetidstring.to_string().into_bytes();
                let jurysetentries = LookupSet::new(jurysetid);
                self.draw_jurors_function(review_id, jurysetentries, length);
            }
        }
    }
    fn draw_jurors_function(
        &mut self,
        review_id: u128,
        mut jurysetentries: LookupSet<u128>,
        length: usize,
    ) {
        let user_juror_stakes_clone_option = self.user_juror_stakes_clone.get(&review_id);
        match user_juror_stakes_clone_option {
            Some(mut juries_stakes) => {
                let items = juries_stakes.to_vec();
                // println!(">>>>>>>>Juries{:?}<<<<<<<<<<<", items);
                let random_vec = env::random_seed();
                let mut rng = get_rng(random_vec);
                let mut dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
                let selected_juror_count_option = self.selected_juror_count.get(&review_id);
                let mut countvalue = 0;
                match selected_juror_count_option {
                    Some(count) => {
                        if count >= self.jury_count {
                            panic!("Jury selection done");
                        } else {
                            countvalue = count;
                        }
                    }
                    None => {}
                }

                for _ in 0..length {
                    let index = dist2.sample(&mut rng);
                    // println!("{}", index);
                    let drawindex = items[index].0;
                    println!("{:?}", drawindex);
                    juries_stakes.remove(&drawindex);
                    jurysetentries.insert(&drawindex);
                    let d = dist2.update_weights(&[(index, &0)]);
                    // println!("{:?}",d);
                    match d {
                        Ok(_v) => {}
                        Err(_e) => {
                            let timestamp = env::block_timestamp();
                            self.juror_selection_time.insert(&review_id, &timestamp);
                            break;
                        }
                    }
                    countvalue += 1;
                    if countvalue >= self.jury_count {
                        let timestamp = env::block_timestamp();
                        self.juror_selection_time.insert(&review_id, &timestamp);
                        break;
                    }
                }
                self.selected_juror_count.insert(&review_id, &countvalue);
                self.user_juror_stakes_clone
                    .insert(&review_id, &juries_stakes);
                self.selected_juror.insert(&review_id, &jurysetentries);
            }
            None => {
                panic!("There are no juries");
            }
        }
    }

    pub fn get_selected_jurors(&self, review_id: u128) -> LookupSet<u128> {
        let selected_juror_option = self.selected_juror.get(&review_id);
        match selected_juror_option {
            Some(jurysetentries) => jurysetentries,
            None => {
                panic!("No selected jurors");
            }
        }
    }
    pub fn get_juror_stakes(&self, review_id: u128, juror_user_id: u128) -> u128 {
        let juror_list_option = self.user_juror_stakes.get(&review_id);
        match juror_list_option {
            Some(juror_list) => {
                let juror_stake = juror_list.get(&juror_user_id).unwrap();
                juror_stake
            }
            None => panic!("No one has staked for the voter"),
        }
    }

    pub fn get_juror_selection_time(&self, review_id: &u128) -> u64 {
        let timestamp_juror_selection_time_option = self.juror_selection_time.get(&review_id);
        match timestamp_juror_selection_time_option {
            Some(timestamp) => timestamp,
            None => {
                panic!("Jurors are not selected yet");
            }
        }
    }

    pub fn commit_vote(&mut self, review_id: u128, vote_commit: String) {
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        // println!("{}, now2", naive_now);
        let timestamp_juror_selection_time = self.get_juror_selection_time(&review_id);
        let native_juror_selection_time =
            NaiveDateTime::from_timestamp(timestamp_juror_selection_time as i64, 0);
        let seconds = Duration::seconds(self.commit_phase_time as i64);
        let endtime = native_juror_selection_time + seconds;
        if naive_now > endtime {
            panic!("Commiting time has ended");
        }
        self.can_juror_vote(review_id, user_id);
        self.add_juror_voting_status_commit(review_id, user_id);
        let mut vote_commit_all = self.get_voter_commits_lookup(review_id);
        let votecommit = vote_commit_all.get(&vote_commit);
        match votecommit {
            Some(_commit) => panic!("This vote is already commited"),
            None => {
                vote_commit_all.insert(&vote_commit, &1);
                self.voter_commit.insert(&review_id, &vote_commit_all);
            }
        }
    }
    fn add_juror_voting_status_commit(&mut self, review_id: u128, user_id: u128) {
        let juror_voting_status_option = self.juror_voting_status.get(&review_id);
        match juror_voting_status_option {
            Some(mut juror_voting_status_lookup) => {
                let juror_voting_status_lookup_option = juror_voting_status_lookup.get(&user_id);
                match juror_voting_status_lookup_option {
                    Some(value) => {
                        if value == 1 || value == 2 {
                            panic!("Voter has already commited");
                        } else {
                            panic!("Not at valid voter status");
                        }
                    }
                    None => {
                        juror_voting_status_lookup.insert(&user_id, &1);
                        self.juror_voting_status
                            .insert(&review_id, &juror_voting_status_lookup);
                    }
                }
            }
            None => {
                let votestatusstring = format!("juror_voting_status{}uniqueid", review_id);
                let votestatusid = votestatusstring.to_string().into_bytes();
                let mut votestatus_lookup = LookupMap::new(votestatusid);
                votestatus_lookup.insert(&user_id, &1);
                self.juror_voting_status
                    .insert(&review_id, &votestatus_lookup);
            }
        }
    }
    fn get_voter_commits_lookup(&self, review_id: u128) -> LookupMap<String, u8> {
        let vote_status_option = self.voter_commit.get(&review_id);
        match vote_status_option {
            Some(votecommits) => votecommits,
            None => {
                let votestatusstring = format!("votecommit{}uniqueid", review_id);
                let votestatusid = votestatusstring.to_string().into_bytes();
                let votecommits = LookupMap::new(votestatusid);
                votecommits
            }
        }
    }
    pub fn can_juror_vote(&self, review_id: u128, user_id: u128) {
        let selected_juror_option = self.selected_juror.get(&review_id);
        match selected_juror_option {
            Some(jurysetentries) => {
                let juryexists = jurysetentries.contains(&user_id);
                // if user id exists in selected_juror (<review_id, juror_id_set>) it will
                // return true else false, look at the contains
                if juryexists == false {
                    panic!("You are not juror of the review");
                }
            }
            None => {
                panic!("No selected jurors");
            }
        }
    }
    pub fn reveal_vote(&mut self, review_id: u128, vote: String, vote_commit: String) {
        let account_id = env::predecessor_account_id();
        let user_id = self.get_user_id(&account_id);
        let timestamp = env::block_timestamp();
        let naive_now = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        let timestamp_juror_selection_time = self.get_juror_selection_time(&review_id);
        let native_juror_selection_time =
            NaiveDateTime::from_timestamp(timestamp_juror_selection_time as i64, 0);
        let seconds = Duration::seconds(self.commit_phase_time as i64);
        let endtime = native_juror_selection_time + seconds;
        let reveal_end_seconds = Duration::seconds(self.reveal_phase_time as i64);
        let reveal_endtime = native_juror_selection_time + seconds + reveal_end_seconds;
        if naive_now < endtime {
            panic!("Commiting time has not ended");
        }
        if naive_now > reveal_endtime {
            panic!("Reveal time has ended"); // reveal phase time, when the reveal time ends
        }
        self.can_juror_vote(review_id, user_id);
        self.add_juror_voting_status_reveal(review_id, user_id);
        let mut vote_commit_all = self.get_voter_commits_in_reveal(review_id);
        let votecommit = vote_commit_all.get(&vote_commit);

        match votecommit {
            Some(commitstatus) => {
                if commitstatus == 2 {
                    panic!("The vote has be already revealed and added.");
                } else if commitstatus == 1 {
                    vote_commit_all.insert(&vote_commit, &2);
                    self.voter_commit.insert(&review_id, &vote_commit_all);
                }
            }
            None => {
                panic!("Vote with this commit is not present");
            }
        }
        let mut hasher = Keccak256::new();
        hasher.update(vote.as_bytes());
        let result = hasher.finalize();
        let vote_hex = format!("{:x}", result);
        if vote_commit == vote_hex {
            println!("commit and vote matches"); // comment out this step, only for debugging
        }
        if vote_commit != vote_hex {
            panic!("Vote hash doesn't match the vote commit");
        }

        let answer_id_string = format!("{}", &vote[0..1]);
        match answer_id_string.parse::<u8>() {
            Ok(n) => {
                if n > 1 {
                    panic!("Vote can be only 0 or 1");
                } else {
                    let schelling_decisions_juror_option =
                        self.schelling_decisions_juror.get(&review_id);
                    match schelling_decisions_juror_option {
                        Some(mut jurorsdecisionsall) => {
                            let jurydecisionoption = jurorsdecisionsall.get(&user_id);
                            match jurydecisionoption {
                                Some(value) => {
                                    panic!("You have already given the decision {}", value);
                                }
                                None => {
                                    jurorsdecisionsall.insert(&user_id, &n);
                                    self.schelling_decisions_juror
                                        .insert(&review_id, &jurorsdecisionsall);
                                    self.add_true_or_false_count(review_id, n);
                                }
                            }
                        }
                        None => {
                            let decisionstring =
                                format!("decisionstring{}uniqueid{}", review_id, self.user_id);
                            let decisionid = decisionstring.to_string().into_bytes();
                            let mut jurorsdecisionsallmap = LookupMap::new(decisionid);
                            jurorsdecisionsallmap.insert(&user_id, &n);
                            self.schelling_decisions_juror
                                .insert(&review_id, &jurorsdecisionsallmap);
                            self.add_true_or_false_count(review_id, n);
                        }
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    fn get_voter_commits_in_reveal(&self, review_id: u128) -> LookupMap<String, u8> {
        let vote_status_option = self.voter_commit.get(&review_id);
        match vote_status_option {
            Some(votecommits) => votecommits,
            None => {
                panic!("Voter commit doesnot exist for review_id");
            }
        }
    }

    fn add_juror_voting_status_reveal(&mut self, review_id: u128, user_id: u128) {
        let juror_voting_status_option = self.juror_voting_status.get(&review_id);
        match juror_voting_status_option {
            Some(mut juror_voting_status_lookup) => {
                let juror_voting_status_lookup_option = juror_voting_status_lookup.get(&user_id);
                match juror_voting_status_lookup_option {
                    Some(value) => {
                        if value == 2 {
                            panic!("The juror has already been revealed a vote.");
                        } else if value == 1 {
                            juror_voting_status_lookup.insert(&user_id, &2);
                            self.juror_voting_status
                                .insert(&review_id, &juror_voting_status_lookup);
                        } else {
                            panic!("Not at valid voter status");
                        }
                    }
                    None => {
                        panic!("Voting status doesnot exists, commit the vote first.");
                    }
                }
            }
            None => {
                panic!("Voting status lookup doesnot exists, commit the vote first.");
            }
        }
    }

    fn add_true_or_false_count(&mut self, review_id: u128, value: u8) {
        if value == 0 {
            let schelling_decision_false_count_option =
                self.schelling_decision_false_count.get(&review_id);
            match schelling_decision_false_count_option {
                Some(mut count) => {
                    count += 1;
                    self.schelling_decision_false_count
                        .insert(&review_id, &count);
                }
                None => {
                    self.schelling_decision_false_count.insert(&review_id, &1);
                }
            }
        }
        if value == 1 {
            let schelling_decision_true_count_option =
                self.schelling_decision_true_count.get(&review_id);
            match schelling_decision_true_count_option {
                Some(mut count) => {
                    count += 1;
                    self.schelling_decision_true_count
                        .insert(&review_id, &count);
                }
                None => {
                    self.schelling_decision_true_count.insert(&review_id, &1);
                }
            }
        }
    }

    pub fn get_true_count(&self, review_id: u128) -> u128 {
        let count_option = self.schelling_decision_true_count.get(&review_id);
        match count_option {
            Some(count) => count,
            None => {
                panic!("Count is not set");
            }
        }
    }

    pub fn get_false_count(&self, review_id: u128) -> u128 {
        let count_option = self.schelling_decision_false_count.get(&review_id);
        match count_option {
            Some(count) => count,
            None => {
                panic!("Count is not set");
            }
        }
    }
}
