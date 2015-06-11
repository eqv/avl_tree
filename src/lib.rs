use std::cmp;

struct Node<T> {
    key: u64,
    data: T,
    height: u32,
    left: Option<Box<Node<T>>>,
    right:Option<Box<Node<T>>>,
}

fn height<T>(node: &Option<Box<Node<T>>>) -> u32  {
    return match *node {None => 0, Some(ref succ) => succ.height}
}

impl<T:ToString> ToString for Node<T> {
    fn to_string(&self) -> String{
        return format!("N {}(h: {} l: {}, r: {})", self.key.to_string(), self.height, to_string::<T>(&self.left), to_string::<T>(&self.right));
    }
}

fn to_string<T:ToString>(opt_box_node: &Option<Box<Node<T>>>) -> String {
    return match *opt_box_node {
        Some(ref box_node) => (*box_node).to_string(),
        None => "Ø".to_string()
    }
}


/// Perform a single right rotation on this (sub) tree
fn rotate_right<T>(mut root: Box<Node<T>>) -> Box<Node<T>>{
    let mut new_root_box = root.left.take().expect("AVL broken");
    root.left = new_root_box.right.take();
    update_height(&mut root);
    new_root_box.right = Some(root);
    update_height(&mut new_root_box);
    return new_root_box
}

/// Perform a single left rotation on this (sub) tree
fn rotate_left<T>(mut root: Box<Node<T>>) -> Box<Node<T>>{
    let mut new_root_box = root.right.take().expect("AVL broken");
    root.right = new_root_box.left.take();
    update_height(&mut root);
    new_root_box.left = Some(root);
    update_height(&mut new_root_box);
    return new_root_box
}

/// Performs a rotation that counteracts the fact that the left successor is too high
fn rotate_left_successor<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    let left = root.left.take().expect("AVL broken");
    if height(&left.left) < height(&left.right) {
        let mut rotated = rotate_left(left);
        root.left = Some(rotated);
        update_height(&mut root);
    }
    else{
        root.left = Some(left);
    }
    root = rotate_right(root);
    return root;
}

/// Performs a rotation that counteracts the fact that the right successor is too high
fn rotate_right_successor<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    let right = root.right.take().expect("AVL broken");
    if height(&right.left) > height(&right.right) {
        let mut rotated = rotate_right(right);
        root.right = Some(rotated);
        update_height(&mut root);
    }
    else {
        root.right = Some(right)
    }
    root = rotate_left(root);
    return root
}

fn diff_of_successors_height<T>(root: &Box<Node<T>>) -> i32 {
    let l = height(&root.left);
    let r = height(&root.right);
    //Since AVL trees are balanced this should never happen, also prevents integer overflows
    //But honestly this is mostly for debugging purposes
    assert!(l < 128); 
    assert!(r < 128);
    return (l as i32) - (r as i32)
}


/// Apply all necessary rotations on root. 
fn rotate_if_necessary<T>(root: Box<Node<T>>) -> Box<Node<T>> {
    let diff  = diff_of_successors_height(&root);
    if -1 <= diff && diff <= 1 {return root}
    match diff{
        2 =>return rotate_left_successor::<T>(root),
        -2 => return rotate_right_successor::<T>(root),
        _ => unreachable!()
    };
}

/// update the cached height of root. To call this function make sure that the cached values of
/// both children of root ar up to date.
fn update_height<T>(root: &mut Node<T>){
    root.height = cmp::max( height(&root.left), height(&root.right) )+1;
}

/// Inserts the given data under the key in the tree root. It will replace old data stored
/// under this key if it was allready used in the tree. The resulting tree will be returned (its
/// root may now differ due to rotations, thus the old root is moved into the function)
fn insert<T: Copy>(key: u64, data: T, mut root: Box<Node<T>>) -> Box<Node<T>>{
    if root.key == key { root.data = data; return root }
    if root.key < key {
        if let Some(succ) = root.right.take() {
            root.right = Some(insert(key, data, succ));
        } else {
            root.right = Some(Box::new(Node::<T>{key: key, data: data, height: 1, left: None, right: None}));
        }
    }
    if root.key > key {
        if let Some(succ) = root.left.take() {
            root.left = Some(insert(key, data, succ));
        } else {
            root.left = Some(Box::new(Node::<T>{key: key, data: data, height: 1, left: None, right: None}));
        }
    }
    update_height(&mut *root);
    return rotate_if_necessary(root);
    unreachable!();
}

/// returns a read only reference to the data stored under key in the tree given by root
fn search<T>(key: u64, root: &Box<Node<T>>) -> Option<&T>{
    if root.key == key { return Some(&root.data) }
    if root.key < key {
        if let Some(ref succ) = root.right {
            return search(key, &succ)
        }
    }
    if root.key > key {
        if let Some(ref succ) = root.left {
            return search(key, &succ)
        }
    }
    return None
}

/// returns true iff key is stored in the tree given by root
fn contains<T>(key: u64, root: &Box<Node<T>> ) -> bool  {
    match search(key,root) {
        None => return false,
        Some(_) => return true
    }
}

#[test]
fn simple_tree_operations() {
    let mut t = Box::new(Node::<i32>{key: 3, data: 4, height: 0,
        left: Some(Box::new(Node::<i32>{key: 2, data: 5, height:0, left: None, right: None})), 
        right: None});
    assert!( contains::<i32>(3,&t) );
    assert!( contains::<i32>(2,&t) );
    assert!( !contains::<i32>(6,&t) );
    assert!( !contains::<i32>(4,&t) );
    t = insert::<i32>(4,7, t);
    t = insert::<i32>(5,7, t);
    t = insert::<i32>(6,8, t);
    assert!( contains::<i32>(4,&t) );
    assert!( contains::<i32>(6,&t) );
    assert!( !contains::<i32>(7,&t) );
}

#[test]
fn rotations_on_tree(){ 
    let mut t = Box::new(Node::<i32>{key: 1, data: 1337, height: 1, left: None, right: None});
    for i in 2..255 {
        t = insert::<i32>(i,7, t);
    }
    //check that the tree is indeed balanced
    assert!(height(&Some(t)) <= 8);
}
