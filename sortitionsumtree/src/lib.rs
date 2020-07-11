use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{TreeMap, Vector};
use near_sdk::{env, near_bindgen};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SortitionSumTree {
    k: u128,
    stack: Vector<u128>,
    nodes: Vector<u128>,
    ids_to_node_indexes: TreeMap<String, u128>,
    node_indexes_to_ids: TreeMap<u128, String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SortitionSumTrees {
    sortition_sum_trees: TreeMap<String, SortitionSumTree>,
    uniquecount: u128,
}

#[near_bindgen]
impl SortitionSumTrees {
    pub fn new() -> SortitionSumTrees {
        let id = "68dbf390-0b13-4db1-bb7d-9bf6ac5d23ab".to_string().into_bytes();
        SortitionSumTrees {
            sortition_sum_trees: TreeMap::new(id),
            uniquecount: 0
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
                let s = "SortitionSumTree";
                self.uniquecount = self.uniquecount + 1;
                let t = format!("{}{}", s, self.uniquecount);
                println!("{:?}", t);
                let node_id = t.to_string().into_bytes();
                self.uniquecount = self.uniquecount + 1;
                let t = format!("{}{}", s, self.uniquecount);
                println!("{:?}", t);
                let stack_id = t.to_string().into_bytes();
                self.uniquecount = self.uniquecount+ 1;
                let t = format!("{}{}", s, self.uniquecount);
                println!("{:?}", t);
                let ids_to_node_indexes_id = t.to_string().into_bytes();
                self.uniquecount = self.uniquecount+ 1;
                let t = format!("{}{}", s, self.uniquecount);
                println!("{:?}", t);
                let node_indexes_to_ids_id = t.to_string().into_bytes();
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

    pub fn set(&mut self, _key: String, _value: u128, _id: String) {
        let tree_option = self.sortition_sum_trees.get(&_key);

        match tree_option {
            Some(mut tree) => {
                let ids_to_node_option = tree.ids_to_node_indexes.get(&_id);
                println!("ids_to_node_option {:?}", ids_to_node_option);
                match ids_to_node_option {
                    Some(tree_index) => {
                        if _value == 0 {
                            let value = tree.nodes.get(tree_index as u64).unwrap();
                            tree.nodes.replace(tree_index as u64, &0);
                            tree.stack.push(&tree_index);
                            tree.ids_to_node_indexes.remove(&_id);
                            tree.node_indexes_to_ids.remove(&tree_index);
                            self.sortition_sum_trees.insert(&_key, &tree);
                            self.update_parents(_key, tree_index, false, value);
                        } else if _value != tree.nodes.get(tree_index as u64).unwrap() {
                            let plus_or_minus =
                                tree.nodes.get(tree_index as u64).unwrap() <= _value;
                            let plus_or_minus_value = if plus_or_minus {
                                _value - tree.nodes.get(tree_index as u64).unwrap()
                            } else {
                                tree.nodes.get(tree_index as u64).unwrap() - _value
                            };
                            tree.nodes.replace(tree_index as u64, &_value);

                            self.sortition_sum_trees.insert(&_key, &tree);
                            self.update_parents(
                                _key,
                                tree_index,
                                plus_or_minus,
                                plus_or_minus_value,
                            );
                        }
                    }
                    None => {
                        if _value != 0 {
                            println!("{:?}", tree.stack.len());
                            let mut tree_index: u128 = 0;
                            if tree.stack.len() == 0 {
                                tree_index = tree.nodes.len() as u128;
                                println!("Node length {:?}", tree_index);
                                tree.nodes.push(&_value);
                                println!("{:?}: Nodes", tree.nodes.to_vec());

                                if tree_index != 1 && (tree_index - 1) % tree.k == 0 {
                                    println!("Inside a long test");
                                    println!("Tree index {:?}", tree_index);
                                    println!("K value {:?}", tree.k);
                                    let parent_index = tree_index / tree.k;
                                    println!("{:?}: parent_index", parent_index);
                                    println!(
                                        "nodes_indexes_to_ids: {:?}",
                                        tree.node_indexes_to_ids.to_vec()
                                    );
                                    let parent_id =
                                        tree.node_indexes_to_ids.get(&parent_index).unwrap();
                                    println!("{:?}: parent_id", parent_id);
                                    let new_index = tree_index + 1;
                                    tree.nodes
                                        .push(&tree.nodes.get(parent_index as u64).unwrap());
                                    tree.node_indexes_to_ids.remove(&parent_index);
                                    tree.ids_to_node_indexes.insert(&parent_id, &new_index);
                                    tree.node_indexes_to_ids.insert(&new_index, &parent_id);
                                    self.sortition_sum_trees.insert(&_key, &tree);
                                }
                            } else {
                                println!("Inside else block long test");

                                let tree_index = tree.stack.get(tree.stack.len() - 1);
                                tree.stack.pop();
                                tree.nodes.replace(tree_index.unwrap() as u64, &_value);
                                self.sortition_sum_trees.insert(&_key, &tree);
                            }
                            println!("Before appending 0 and id");
                            println!("Tree index {:?}", tree_index);
                            tree.ids_to_node_indexes.insert(&_id, &tree_index);
                            tree.node_indexes_to_ids.insert(&tree_index, &_id);
                            println!(
                                "node_indexes_to_ids {:?}",
                                tree.node_indexes_to_ids.to_vec()
                            );
                            println!(
                                "ids_to_node_indexes {:?}",
                                tree.ids_to_node_indexes.to_vec()
                            );
                            self.sortition_sum_trees.insert(&_key, &tree);
                            self.update_parents(_key, tree_index, true, _value);
                        }
                    }
                }
            }

            None => {
                println!("Null");
            }
        }
    }

    fn update_parents(
        &mut self,
        _key: String,
        _tree_index: u128,
        _plus_or_minus: bool,
        _value: u128,
    ) {
        let mut tree = self.sortition_sum_trees.get(&_key).unwrap();

        println!("{:?} hello", tree.ids_to_node_indexes.to_vec());

        let mut parent_index = _tree_index;
        println!("{:?} parent index", parent_index);

        while parent_index != 0 {
            parent_index = (parent_index - 1) / tree.k;
            let nodes = tree.nodes.get(parent_index as u64).unwrap();
            println!("{:?}", nodes);
            let tree_node_value = if _plus_or_minus {
                tree.nodes.get(parent_index as u64).unwrap() + _value
            } else {
                tree.nodes.get(parent_index as u64).unwrap() - _value
            };

            tree.nodes.replace(parent_index as u64, &tree_node_value);
            println!("Final K: {:?}", tree.k);
            println!("Final stack: {:?}", tree.stack.to_vec());
            println!("Final nodes: {:?}", tree.nodes.to_vec());
            println!(
                "Final ids_to_node_indexes: {:?}",
                tree.ids_to_node_indexes.to_vec()
            );
            println!(
                "Final node_indexes_to_ids: {:?}",
                tree.node_indexes_to_ids.to_vec()
            );

            self.sortition_sum_trees.insert(&_key, &tree);
        }
    }

    pub fn draw(&mut self, _key: String, _draw_number: u128) -> String {
        let tree = self.sortition_sum_trees.get(&_key).unwrap();
        let mut tree_index = 0;
        let mut current_draw_number = _draw_number % tree.nodes.get(0).unwrap();

        while (tree.k * tree_index) + 1 < (tree.nodes.len() as u128) {
            for i in 1..tree.k + 1 {
                let node_index = (tree.k * tree_index) + i;
                let node_value = tree.nodes.get(node_index as u64).unwrap();
                if current_draw_number >= node_value {
                    current_draw_number -= node_value;
                } else {
                    tree_index = node_index;
                    break;
                }
            }
        }

        tree.node_indexes_to_ids.get(&tree_index).unwrap()
    }
    pub fn stake_of(&mut self, _key: String, _id: String) -> u128 {
        let tree = self.sortition_sum_trees.get(&_key).unwrap();
        let tree_index = tree.ids_to_node_indexes.get(&_id).unwrap();
        let  value:u128;
        if tree_index == 0 {
            value = 0;
        } else {
            value = tree.nodes.get(tree_index as u64).unwrap();
        }
        value

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
        // for x in 0..1000{
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SortitionSumTrees::new();
        contract.create_tree("Python".to_owned(), 2);
        // data.create_tree("Python".to_owned(), 1);
        contract.set("Python".to_owned(), 15, "Code1".to_owned());
        println!("---------------------------------------------");
        contract.set("Python".to_owned(), 5, "Code2".to_owned());
        println!("---------------------------------------------");
        contract.set("Python".to_owned(), 10, "Code3".to_owned());
        println!("---------------------------------------------");
        contract.set("Python".to_owned(), 20, "Code4".to_owned());

        let draw_value = contract.draw("Python".to_owned(), 13);
        println!("{:?}", draw_value);
        let draw_value = contract.draw("Python".to_owned(), 27);
        println!("{:?}", draw_value);

        let draw_value = contract.draw("Python".to_owned(), 3);
        println!("{:?}", draw_value);
        let draw_value = contract.draw("Python".to_owned(), 19);
        println!("{:?}", draw_value);
        let draw_value = contract.draw("Python".to_owned(), 49);
        println!("{:?}", draw_value);
        let draw_value = contract.draw("Python".to_owned(), 0);
        println!("{:?}", draw_value);
        let value = contract.stake_of("Python".to_owned(), "Code4".to_owned());
        println!("{:?}", value);

        contract.create_tree("Python2".to_owned(), 2);
        contract.set("Python2".to_owned(), 15, "Code1".to_owned());


        // }
        
    }
}
