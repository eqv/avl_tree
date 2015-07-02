# Rust AVL Trees
This is an implementation of AVL trees in rust. It currently features insert, delete, look-up and iteration over ranges of arbitrary keys and arbitrary
values. It is implemented entirely without unsafe code (this means any update will result in an additional unecessary amount of log(n) memory writes du to `foo.left = some_op(foo.left.take())`. You should not be using this unless you know what you are doing, as I only wrote it to get to know programming in rust. However, it is decently tested and contains no unsafe code so feel free to have a look at it. Also since it is a learning project, I would be very glad to hear your ideas of how to improve the code (even simple things like reformating).

#Examples
```rust
#![feature(collections_bound)]
extern crate avl_tree;
use std::collections::Bound;
let mut t=avl_tree::AVLTree::<u64,i32>::new();

t.insert(2,25);
assert_eq!(t.get(2), Some(&25));
t.insert(2,30);
assert_eq!(t.get(2), Some(&30));
t.delete(2);
assert!(t.empty());

t.insert(2,25);
assert_eq!(t.get(2), Some(&25));
assert_eq!(t.get(3), None);
assert_eq!(t.get_or(2,&2000), &25);
assert_eq!(t.get_or(3,&2000), &2000);
assert!(!t.contains(3));
assert!(t.contains(2));

t.insert(3,50);
assert_eq!(t.max().unwrap().0, &3);
assert_eq!(t.max().unwrap().1, &50);

for (key,val) in t.iter() {
    println!("{} -> {}",key,val)
}

for (key,val) in t.range(Bound::Excluded(32), Bound::Excluded(38)) {
  println!("{} -> {}",key,val)
}

```
