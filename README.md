% Rust AVL Trees
This is an implementation of AVL trees in rust. It currently features insert, delete, look-up and iteration over ranges of arbitrary keys and arbitrary
values. It is implemented entirely without unsafe code (this means any update will result in an additional unecessary amount of log(n) memory writes du to `foo.left = some_op(foo.left.take())`. You should not be using this unless you know what you are doing, as I only wrote it to get to know programming in rust. However, it is decently tested and contains no unsafe code so feel free to have a look at it. Also since it is a learning project, I would be very glad to hear your ideas of how to improve the code (even simple things like reformating).

#Examples
```rust
#![feature(collections_bound)]
extern crate avl_tree;
use std::collections::Bound;
let mut t=avl_tree::AVLTree::<u64,i32>::new();
for (key,val) in t.range(Bound::Excluded(32), Bound::Excluded(38)) {
  println!("{} -> {}",key,val)
}
```
