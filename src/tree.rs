extern crate rand;
extern crate test;

use node::Node;
use node::{insert,delete,search,min,max,is_avl_tree, to_string, min_pair, max_pair};
use iterators::RangePairIter;
use std::collections::Bound;


pub struct AVLTree<K:Ord+Copy,D> {
    pub root: Option<Box<Node<K,D>>>
}

impl <K:Ord+Copy,D> AVLTree<K,D>{

/// This function will construct a new empty AVLTree.
/// # Examples
/// ```
/// extern crate avl_tree;
/// let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// ```
    pub fn new() -> AVLTree<K,D>{
        AVLTree{root: None}
    }

/// This function will insert the key,value pair into the tree, overwriting the old data if the key is allready
/// part of the tree.
/// # Examples
/// ```
/// let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// t.insert(2,25);
/// assert_eq!(t.get(2), Some(&25));
/// t.insert(2,30);
/// assert_eq!(t.get(2), Some(&30));
/// ```
    pub fn insert(&mut self, key: K, data: D) {
        match self.root.take() {
            Some(box_to_node) => self.root = Some(insert::<K,D>(key, data, box_to_node)),
            None => self.root = Some(Box::new(Node::new(key,data))),
        }
    }

/// This function will remove the key,value pair from the tree, doing nothing if the key is not
/// part of the tree.
/// # Examples
/// ```
/// let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// t.insert(2,25);
/// t.delete(2);
/// assert!(t.empty());
/// t.delete(3);
/// assert!(t.empty());
/// ```
    pub fn delete(&mut self, key: K){
        match self.root.take() {
            Some(box_to_node) => self.root = delete(key,box_to_node),
            None => return
        }
    }

/// This function will return the Some(data) stored under the given key or None if the key is not
/// known.
/// # Examples
/// ```
/// let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// t.insert(2,25);
/// assert_eq!(t.get(2), Some(&25));
/// assert_eq!(t.get(3), None);
///
/// ```
    pub fn get(&self, key: K) -> Option<&D>{
        match self.root {
            Some(ref box_to_node) =>search(&key, box_to_node),
            None => None
        }
    }

/// This function will return the data stored under the given key or the default if the key is not
/// known.
/// # Examples
/// ```
/// let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// t.insert(2,25);
/// assert_eq!(t.get_or(2,&2000), &25);
/// assert_eq!(t.get_or(3,&2000), &2000);
///
/// ```
    pub fn get_or<'a>(&'a self, key: K, default: &'a D) -> &D{
        self.get(key).map_or(default, |data| data)
    }

/// This function will return true if the tree contains the given key, false otherwise
/// # Examples
/// ```
/// let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// t.insert(2,25);
/// assert!(!t.contains(3));
/// assert!(t.contains(2));
///
/// ```
    pub fn contains(&self, key: K) -> bool {
        self.get(key).is_some()
    }

/// This function will return true if the tree is empty, false otherwise.
/// # Examples
/// ```
/// let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// assert!(t.empty());
/// t.insert(2,25);
/// assert!(!t.empty());
///
/// ```
    pub fn empty(&self) -> bool { self.root.is_none() }

/// This function will return the key/value pair with the smallest key in the tree, or None if the
/// tree is empty.
/// # Examples
/// ```
/// let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// t.insert(2,25);
/// t.insert(3,50);
/// assert_eq!(t.min().unwrap().0, &2);
/// assert_eq!(t.min().unwrap().1, &25);
///
/// ```
    pub fn min<'a>(&'a self) -> Option<(&'a K,&'a D)> {
        match self.root {
            Some(ref root) => Some(min_pair(root)),
            None => None
        }
    }

/// This function will return the key/value pair with the biggest key in the tree, or None if the
/// tree is empty.
/// # Examples
/// ```
/// let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// t.insert(2,25);
/// t.insert(3,50);
/// assert_eq!(t.max().unwrap().0, &3);
/// assert_eq!(t.max().unwrap().1, &50);
///
/// ```
    pub fn max<'a>(&'a self) -> Option<(&'a K,&'a D)> {
        match self.root {
            Some(ref root) => Some(max_pair(root)),
            None => None
        }
    }

/// This function will return a read only iterator for all (key,value) pairs in the tree.
/// # Examples
/// ```
/// # let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// for (key,val) in t.iter() {
///     println!("{} -> {}",key,val)
/// }
///
/// ```
    pub fn iter(&self) -> RangePairIter<K,D>{
        RangePairIter::new(self, Bound::Unbounded, Bound::Unbounded)
    }

/// This function will return a read only iterator for all (key,value) pairs between the two bounds (which can
/// be inclusive, exclusive or unbounded).
/// # Examples
/// ```
/// #![feature(collections_bound)]
/// # extern crate avl_tree;
/// use std::collections::Bound;
/// //[...]
/// # let mut t=avl_tree::AVLTree::<u64,i32>::new();
/// for (key,val) in t.range(Bound::Excluded(32), Bound::Excluded(38)) {
///     println!("{} -> {}",key,val)
/// }
///
/// ```
    pub fn range(&self, min: Bound<K>, max: Bound<K>) -> RangePairIter<K,D>{
        RangePairIter::new(self, min, max)
    }

    fn test_avl_tree(&self) -> bool {
        is_avl_tree(&self.root)
    }
}

#[test]
fn test_fuzz(){
    let mut t = AVLTree::<u64,i32>::new();
    for _ in 1..5000 {
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
