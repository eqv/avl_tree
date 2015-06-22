//#![crate_id = "avl_tree"]
#![crate_type = "lib"]
#![feature(test)]

mod node;
pub mod tree;
mod iterators;
pub use tree::AVLTree;
