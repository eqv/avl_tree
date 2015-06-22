extern crate rand;
extern crate test;

use node::Node;
use node::{insert,delete,search,min,max,is_avl_tree, to_string};

pub struct AVLTree<K:Ord,D> {
    root: Option<Box<Node<K,D>>>
}

impl <K:Ord,D> AVLTree<K,D>{
    pub fn new() -> AVLTree<K,D>{
        AVLTree{root: None}
    }

    pub fn insert(&mut self, key: K, data: D) {
        match self.root.take() {
            Some(box_to_node) => self.root = Some(insert::<K,D>(key, data, box_to_node)),
            None => self.root = Some(Box::new(Node::new(key,data))),
        }
    }

    pub fn delete(&mut self, key: K){
        match self.root.take() {
            Some(box_to_node) => self.root = delete(key,box_to_node),
            None => return
        }
    }

    pub fn get(&self, key: K) -> Option<&D>{
        match self.root {
            Some(ref box_to_node) =>search(key, box_to_node),
            None => None
        }
    }

    pub fn get_or<'a>(&'a self, key: K, default: &'a D) -> &D{
        match self.get(key) {
            Some(data) => data,
            None => default
        }
    }

    pub fn contains(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn empty(&self) -> bool { self.root.is_none() }

    pub fn min<'a>(&'a self) -> Option<&'a D> {
        match self.root {
            Some(ref root) => Some(min(root)),
            None => None
        }
    }

    pub fn max<'a>(&'a self) -> Option<&'a D> {
        match self.root {
            Some(ref root) => Some(max(root)),
            None => None
        }
    }

    fn test_avl_tree(&self) -> bool {
        is_avl_tree(&self.root)
    }

    //pub fn size(&self) -> u32 {}
    //pub fn drop(&mut self, key: K) -> D{}
}

#[test]
fn test_fuzz(){
    let mut t = AVLTree::<u64,i32>::new();
    for _ in 1..500 {
        let decision = rand::random::<bool>();
        if  decision {
            let to_insert = rand::random::<u64>()%500;
            t.insert(to_insert, 1337);
            assert!(t.contains(to_insert));
            assert!(t.test_avl_tree());
        } else {
            let to_delete = rand::random::<u64>()%500;
            t.delete(to_delete);
            assert!(!t.contains(to_delete));
            assert!(t.test_avl_tree());
        };
    };
    return;
}
