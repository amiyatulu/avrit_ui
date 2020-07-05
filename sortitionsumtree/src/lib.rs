use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{TreeMap, Vector};
use near_sdk::{env, near_bindgen};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use uuid::{Builder, Uuid, Variant, Version};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn get_uuid(seed_vec: Vec<u8>) -> Uuid {
    let mut seed = [0u8; 32];
    let mut counter = 0;
    for v in seed_vec.iter() {
        seed[counter] = *v;
        counter += 1;
    }

    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    let uuid = Builder::from_bytes(bytes)
        .set_variant(Variant::RFC4122)
        .set_version(Version::Random)
        .build();
    return uuid;
}
#[derive(BorshDeserialize, BorshSerialize)]
struct SortitionSumTree {
    k: u128,
    stack: Vector<u128>,
    nodes: Vector<u128>,
    ids_to_node_indexes: TreeMap<String, u128>,
    node_indexes_to_ids: TreeMap<u128, String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct SortitionSumTrees {
    sortition_sum_trees: TreeMap<String, SortitionSumTree>,
}

#[near_bindgen]
impl SortitionSumTrees {
    pub fn new() -> SortitionSumTrees {
        let random_vec = env::random_seed();
        let id = get_uuid(random_vec).to_string().into_bytes();
        SortitionSumTrees {
            sortition_sum_trees: TreeMap::new(id),
        }
    }
    pub fn create_tree(&mut self, _key: String, _k: u128) {
        if _k < 1 {
            panic!("K must be greater than one");
        }
        let tree_option = self.sortition_sum_trees.get(&_key);
        match tree_option {
            Some(_tree) => {
                panic!("The tree already exists.");
            }
            None => {
                let mut random_vec = env::random_seed();
                random_vec[0] = 1;
                let rand = random_vec.clone();
                println!("{:?}", rand);
                let stack_id = get_uuid(rand).to_string().into_bytes();
                random_vec[0] = 2;
                let rand = random_vec.clone();
                println!("{:?}", rand);
                let node_id = get_uuid(rand).to_string().into_bytes();
                random_vec[0] = 3;
                let rand = random_vec.clone();
                println!("{:?}", rand);
                let ids_to_node_indexes_id = get_uuid(rand).to_string().into_bytes();
                random_vec[0] = 4;
                let rand = random_vec.clone();
                println!("{:?}", rand);
                let node_indexes_to_ids_id = get_uuid(rand).to_string().into_bytes();
                let mut firstnode = Vector::new(node_id);
                firstnode.push(&0);
                let sum_tree = SortitionSumTree {
                    k: _k,
                    stack: Vector::new(stack_id),
                    nodes: firstnode,
                    ids_to_node_indexes: TreeMap::new(ids_to_node_indexes_id),
                    node_indexes_to_ids: TreeMap::new(node_indexes_to_ids_id),
                };
                self.sortition_sum_trees.insert(&_key, &sum_tree);
            }
        }
    }

    fn set(&mut self, _key: String, _value: u128, _id: String) {
        let tree_option = self.sortition_sum_trees.get(&_key);

        match tree_option {
            Some(mut tree) => {
                let ids_to_node_option = tree.ids_to_node_indexes.get(&_id);
                println!("Data {:?}", ids_to_node_option);
                match ids_to_node_option {
                    Some(node) => {}
                    None => {
                        if _value != 0 {
                            println!("{:?}", tree.stack.len());
                            if tree.stack.len() == 0 {
                                let tree_index = tree.nodes.len() as u128;
                                println!("Node length {:?}", tree_index);
                                tree.nodes.push(&_value);
                                println!("{:?}", tree.nodes.to_vec());

                                if tree_index != 1 && (tree_index - 1) % tree.k == 0 {
                                    println!("Inside a long test");

                                    let parent_index = tree_index / tree.k;
                                    let parent_id =
                                        tree.node_indexes_to_ids.get(&parent_index).unwrap();
                                    let new_index = tree_index + 1;
                                    tree.nodes
                                        .push(&tree.nodes.get(parent_index as u64).unwrap());
                                    tree.node_indexes_to_ids.remove(&parent_index);
                                    tree.ids_to_node_indexes.insert(&parent_id, &new_index);
                                    tree.node_indexes_to_ids.insert(&new_index, &parent_id);
                                }
                            } else {
                                println!("Inside else block long test");

                                let tree_index = tree.stack.get(tree.stack.len() - 1);
                                tree.stack.pop();
                                tree.nodes.replace(tree_index.unwrap() as u64, &_value);
                            }
                            tree.ids_to_node_indexes.insert(&_id, &0);
                            tree.node_indexes_to_ids.insert(&0, &_id);
                            println!(
                                "node_indexes_to_ids {:?}",
                                tree.node_indexes_to_ids.to_vec()
                            );
                            println!(
                                "ids_to_node_indexes {:?}",
                                tree.ids_to_node_indexes.to_vec()
                            );
                        }
                    }
                }
            }

            None => {
                println!("Null");
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
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

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
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
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut data = SortitionSumTrees::new();
        data.create_tree("Python".to_owned(), 1);
        // data.create_tree("Python".to_owned(), 1);
        data.set("Python".to_owned(), 5, "Code".to_owned());
    }
}
