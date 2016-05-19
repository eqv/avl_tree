#![feature(collections_bound)]

extern crate avl_tree;
extern crate rand;

use std::collections::Bound;
use std::time::Instant;

#[test]
fn test_getters(){
    let data = 1337;
    let mut t = avl_tree::AVLTree::<u64,i32>::new();
    t.insert(1, data);
    t.insert(2, data+1);
    t.insert(3, data+2);
    assert!(t.get_or(1, &0) == &data);
    assert!(t.get_or(2, &0) == &(data+1));
    assert!(t.get_or(3, &0) == &(data+2));
    assert!(t.get_or(4, &0) == &0);
    assert!(t.get(4) == None);
}

#[test]
fn test_contains(){
    let data = 1337;
    let mut t = avl_tree::AVLTree::<u64,i32>::new();
    t.insert(1, data);
    t.insert(2, data+1);
    t.insert(3, data+2);
    assert!(!t.contains(0));
    assert!(t.contains(1));
    assert!(t.contains(2));
    assert!(t.contains(3));
    assert!(!t.contains(4));
}

#[test]
fn test_empty(){
    let data = 1337;
    let mut t = avl_tree::AVLTree::<u64,i32>::new();
    assert!(t.empty());
    t.insert(1, data);
    t.insert(2, data+1);
    t.insert(3, data+2);
    assert!(!t.empty());
}

#[test]
fn test_delete(){
    let data = 1337;
    let mut t = avl_tree::AVLTree::<u64,i32>::new();
    t.insert(1, data);
    t.insert(2, data+1);
    t.insert(3, data+2);
    t.delete(1);
    assert!(!t.contains(1));
    assert!(t.contains(2));
    assert!(t.contains(3));
    t.delete(2);
    assert!(!t.contains(1));
    assert!(!t.contains(2));
    assert!(t.contains(3));
    t.delete(3);
    assert!(!t.contains(1));
    assert!(!t.contains(2));
    assert!(!t.contains(3));
    assert!(t.empty());
}

#[test]
fn test_perfomance(){
    let mut t = avl_tree::AVLTree::<u64,i32>::new();
    let data = 1337;
    let start = Instant::now();
    for _ in 1..10000 {
        t.insert(1, data);
        t.insert(20000, data+1);
        t.delete(1);
        t.delete(20000);
    }
    let diff_simple = start.elapsed();
    for x in 5..2000 {
        t.insert(x, data);
    }

    let start_2 = Instant::now();
    for _ in 1..10000 {
        t.insert(1, data);
        t.insert(20000, data+1);
        t.delete(1);
        t.delete(20000);
    }
    let diff_full = start_2.elapsed();
    assert!(diff_full < diff_simple * 13); //log time 
}

#[test]
fn test_min(){
    let mut t = avl_tree::AVLTree::<u64,i32>::new();
    assert!{t.min().is_none()};
    t.insert(50,1337);
    assert_eq!{t.min().expect("get 1 min"),(&50,&1337)};
    t.insert(49,1338);
    assert_eq!{t.min().expect("get 2 min"),(&49,&1338)};
    t.insert(47,1339);
    assert_eq!{t.min().expect("get 2 min"),(&47,&1339)};
    t.insert(48,1340);
    assert_eq!{t.min().expect("get 2 min"),(&47,&1339)};
}

#[test]
fn test_iter(){
    let mut t = avl_tree::AVLTree::<u64,i32>::new();
    t.insert(32,1337);
    t.insert(34,1338);
    t.insert(36,1339);
    t.insert(38,1340);
    for (i,pair) in t.iter().enumerate() {
        let (k,v) = pair;
        println!("{}, {}",k,v);
        assert_eq!(k,&((i as u64)*2 +32));
        assert_eq!(v,&((i as i32)+1337));
    }

}

#[test]
fn test_range_iter(){
    let mut t = avl_tree::AVLTree::<u64,i32>::new();
    t.insert(32,1337);
    t.insert(34,1338);
    t.insert(36,1339);
    t.insert(38,1340);
    for (i,pair) in t.range(Bound::Unbounded, Bound::Unbounded).enumerate() {
        let (k,v) = pair;
        println!("{}, {}",k,v);
        assert_eq!(k,&((i as u64)*2 +32));
        assert_eq!(v,&((i as i32)+1337));
        assert!(i<4);
    }
    println!("included");
    for (i,pair) in t.range(Bound::Included(34), Bound::Included(36)).enumerate() {
        let (k,v) = pair;
        println!("{}, {}",k,v);
        assert_eq!(k,&((i as u64)*2 +34));
        assert_eq!(v,&((i as i32)+1338));
        assert!(i<2);
    }

    println!("excluded");
    for (i,pair) in t.range(Bound::Excluded(32), Bound::Excluded(38)).enumerate() {
        let (k,v) = pair;
        println!("{}, {}",k,v);
        assert_eq!(k,&((i as u64)*2 +34));
        assert_eq!(v,&((i as i32)+1338));
        assert!(i<2);
    }
}
