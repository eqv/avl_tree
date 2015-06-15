use std::cmp;

struct Node<T> {
    key: u64,
    data: T,
    height: u32,
    left: Option<Box<Node<T>>>,
    right:Option<Box<Node<T>>>,
}

fn height<T>(node: &Option<Box<Node<T>>>) -> u32  {
    return node.as_ref().map_or(0, |succ| succ.height)
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
        let rotated = rotate_left(left);
        root.left = Some(rotated);
        update_height(&mut root);
    }
    else{
        root.left = Some(left);
    }
    rotate_right(root)
}

/// Performs a rotation that counteracts the fact that the right successor is too high
fn rotate_right_successor<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    let right = root.right.take().expect("AVL broken");
    if height(&right.left) > height(&right.right) {
        let rotated = rotate_right(right);
        root.right = Some(rotated);
        update_height(&mut root);
    }
    else {
        root.right = Some(right)
    }
    rotate_left(root)
}

fn diff_of_successors_height<T>(root: &Box<Node<T>>) -> i32 {
    let l = height(&root.left);
    let r = height(&root.right);

    //Since AVL trees are balanced this should never happen, also prevents integer overflows
    debug_assert!(l < 128); 
    debug_assert!(r < 128);
    (l as i32) - (r as i32)
}


/// Apply all necessary rotations on root. 
fn rotate_if_necessary<T>(root: Box<Node<T>>) -> Box<Node<T>> {
    let diff  = diff_of_successors_height(&root);
    if -1 <= diff && diff <= 1 {return root}
    match diff{
        2 => rotate_left_successor::<T>(root),
        -2 => rotate_right_successor::<T>(root),
        _ => unreachable!()
    }
}

/// update the cached height of root. To call this function make sure that the cached values of
/// both children of root ar up to date.
fn update_height<T>(root: &mut Node<T>){
    root.height = cmp::max( height(&root.left), height(&root.right) )+1;
}

/// Inserts the given data under the key in the tree root. It will replace old data stored
/// under this key if it was allready used in the tree. The resulting tree will be returned (its
/// root may now differ due to rotations, thus the old root is moved into the function)
fn insert<T>(key: u64, data: T, mut root: Box<Node<T>>) -> Box<Node<T>>{
    if root.key == key { root.data = data; return root }
    else 
    if root.key < key {
        if let Some(succ) = root.right.take() {
            root.right = Some(insert(key, data, succ));
        } else {
            root.right = Some(Box::new(Node::<T>{key: key, data: data, height: 1, left: None, right: None}));
        }
    } else 
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
    search(key,root).is_some()
}


//Performs recursive `drop_and_get_min` if a left  since a successor is available
fn drop_and_get_min_from_left<T>(mut root : Box<Node<T>>, left: Box<Node<T>>) -> (Option<Box<Node<T>>>,Box<Node<T>>) {
    let (new_left, min) =  drop_and_get_min(left);
    root.left = new_left;
    update_height(&mut root);
    (Some(rotate_if_necessary(root)),min)
}

//Finds the minimal value below root and returns a new (optional) tree where the minimal value has been
//removed and the (optional) minimal node as tuple (new_tree, min);
fn drop_and_get_min<T>(mut root: Box<Node<T>>) -> (Option<Box<Node<T>>>, Box<Node<T>>) {
    match root.left.take() {
        Some(left) => drop_and_get_min_from_left(root, left),
        None => (root.right.take(), root)
    }
}

//Return a new AVL tree, as the combination of two subtrees with max(l) <= min(r)
fn combine_two_subtrees<T>(l: Box<Node<T>>, r: Box<Node<T>>) -> Box<Node<T>>{
    let (remaining_tree, min) = drop_and_get_min(r);
    let mut new_root = min;
    new_root.left = Some(l);
    new_root.right = remaining_tree;
    update_height(&mut new_root);
    rotate_if_necessary(new_root)
}

//Return a new AVL tree, where the root has been removed
fn drop_root<T>(mut root: Box<Node<T>>) -> Option<Box<Node<T>>> {
    match ( root.left.take(), root.right.take() ) {
        ( None,     None)    => None,
        ( Some(l),  None)    => Some(l),
        ( None,     Some(r)) => Some(r),
        ( Some(l),  Some(r)) => Some(combine_two_subtrees(l,r))
    }
}
// will delete `key` from the tree `root`. Returns either `Some` tree or if the resilting tree is
// empty: None.
fn delete<T>(key: u64, mut root: Box<Node<T>>) -> Option<Box<Node<T>>>{
    if root.key == key { return drop_root(root); }
    if root.key < key {
        if let Some(succ) = root.right.take() {
            root.right = delete(key, succ);
            update_height(&mut root);
            return Some(rotate_if_necessary(root))
        }
    }
    if root.key > key {
        if let Some(succ) = root.left.take() {
            root.left = delete(key, succ);
            update_height(&mut root);
            return Some(rotate_if_necessary(root))
        }
    }
    unreachable!()
}

// functions only used for testing
fn simple_tree(size: i32) -> Box<Node<i32>> {
    let mut t = Box::new(Node::<i32>{key: 1, data: 1337, height: 0, left:None, right: None});
    for x in 2..size+1 {
        t = insert((x as u64),1337,t)
    }
    t
}

#[test]
fn simple_tree_operations() {
    let mut t = Box::new(Node::<i32>{key: 3, data: 4, height: 2,
        left: Some(Box::new(Node::<i32>{key: 2, data: 5, height:1, left: None, right: None})), 
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
        t = insert::<i32>(i,1337, t);
    }
    //check that the tree is indeed balanced
    assert!(height(&Some(t)) <= 8);
}

#[test]
fn test_drop_min(){
    let mut t = simple_tree(3);
    let (maybe_tree,min) = drop_and_get_min(t);
    t = maybe_tree.expect("failure to get tree for first min delete");
    assert!( min.key == 1);
    assert!(!contains::<i32>(1,&t));
    assert!(contains::<i32>(2,&t));
    assert!(contains::<i32>(3,&t));

    let (maybe_tree,min) = drop_and_get_min(t);
    t = maybe_tree.expect("failure to get tree for second min delete");
    assert!( min.key == 2);
    assert!(!contains::<i32>(1,&t));
    assert!(!contains::<i32>(2,&t));
    assert!(contains::<i32>(3,&t));

    let (maybe_tree,min) = drop_and_get_min(t);
    assert!( maybe_tree.is_none() );
    assert!( min.key == 3);
}

#[test]
fn test_drop_root(){
    let mut t = simple_tree(3);
    let maybe_tree = drop_root(t);
    t = maybe_tree.expect("failure to get tree for first root drop");
    println!("{}",t.to_string());
    assert!( t.height == 2);
    assert!(contains::<i32>(1,&t));
    assert!(!contains::<i32>(2,&t));
    assert!(contains::<i32>(3,&t));

    let maybe_tree = drop_root(t);
    t = maybe_tree.expect("failure to get tree for second root drop");
    assert!(contains::<i32>(1,&t));
    assert!(!contains::<i32>(2,&t));
    assert!(!contains::<i32>(3,&t));

    let maybe_tree = drop_root(t);
    assert!( maybe_tree.is_none() );
}

#[test]
fn test_delete(){
    let mut t = simple_tree(10);
    for i in 1..10 {
        assert!(contains::<i32>(i,&t));
        let maybe_tree = delete(i,t);
        t = maybe_tree.expect("failure to get tree for delete");
        assert!(!contains::<i32>(i,&t));
    }
    assert!(contains::<i32>(10,&t));
    let maybe_tree = delete(10,t);
    assert!(maybe_tree.is_none());
}
