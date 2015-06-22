extern crate avl_tree;
extern crate rand;
extern crate time;

use time::PreciseTime;

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
    let start = PreciseTime::now();
    for _ in 1..10000 {
        t.insert(1, data);
        t.insert(20000, data+1);
        t.delete(1);
        t.delete(20000);
    }
    let end = PreciseTime::now();
    let diff_simple = start.to(end).num_milliseconds();
    for x in 5..2000 {
        t.insert(x, data);
    }

    let start_2 = PreciseTime::now();
    for _ in 1..10000 {
        t.insert(1, data);
        t.insert(20000, data+1);
        t.delete(1);
        t.delete(20000);
    }
    let end_2 = PreciseTime::now();
    let diff_full = start_2.to(end_2).num_milliseconds();
    assert!(diff_full < diff_simple * 13); //log time 
}

#[test]
fn test_min(){
    let mut t = avl_tree::AVLTree::<u64,i32>::new();
    assert!{t.min().is_none()};
    t.insert(50,1337);
    assert_eq!{t.min().expect("get 1 min"),&1337};
    t.insert(49,1338);
    assert_eq!{t.min().expect("get 2 min"),&1338};
    t.insert(47,1339);
    assert_eq!{t.min().expect("get 2 min"),&1339};
    t.insert(48,1340);
    assert_eq!{t.min().expect("get 2 min"),&1339};
}
