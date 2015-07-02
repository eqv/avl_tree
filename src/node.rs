use std::cmp;
use std::cmp::Ordering;

pub struct Node<K:Ord,D> {
    key: K,
    data: D,
    height: u32,
    left: Option<Box<Node<K,D>>>,
    right:Option<Box<Node<K,D>>>,
}

impl<K:Ord, D> Node<K,D> {
    pub fn new(key: K, data: D) -> Node<K,D>{
        Node::<K,D>{key: key, data: data, height: 1, left: None, right: None}
    }
}

fn height<K:Ord,D>(node: &Option<Box<Node<K,D>>>) -> u32  {
    return node.as_ref().map_or(0, |succ| succ.height)
}

impl<K:ToString+Ord, D:ToString> ToString for Node<K,D> {
    fn to_string(&self) -> String{
        return format!("N {}(h: {} l: {}, r: {})", self.key.to_string(), self.height, to_string::<K,D>(&self.left), to_string::<K,D>(&self.right));
    }
}

pub fn to_string<K:ToString+Ord,D:ToString>(opt_box_node: &Option<Box<Node<K,D>>>) -> String {
    return match *opt_box_node {
        Some(ref box_node) => (*box_node).to_string(),
        None => "Ø".to_string()
    }
}

/// Perform a single right rotation on this (sub) tree
fn rotate_right<K:Ord,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>>{
    let mut new_root_box = root.left.take().expect("AVL broken");
    root.left = new_root_box.right.take();
    update_height(&mut root);
    new_root_box.right = Some(root);
    update_height(&mut new_root_box);
    return new_root_box
}

/// Perform a single left rotation on this (sub) tree
fn rotate_left<K:Ord,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>>{
    let mut new_root_box = root.right.take().expect("AVL broken");
    root.right = new_root_box.left.take();
    update_height(&mut root);
    new_root_box.left = Some(root);
    update_height(&mut new_root_box);
    return new_root_box
}

/// Performs a rotation that counteracts the fact that the left successor is too high
fn rotate_left_successor<K:Ord,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>> {
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
fn rotate_right_successor<K:Ord,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>> {
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

fn diff_of_successors_height<K:Ord,D>(root: &Box<Node<K,D>>) -> i32 {
    let l = height(&root.left);
    let r = height(&root.right);
    (l as i32) - (r as i32)
}


/// Apply all necessary rotations on root. 
fn rotate_if_necessary<K:Ord,D>(root: Box<Node<K,D>>) -> Box<Node<K,D>> {
    let diff  = diff_of_successors_height(&root);
    if -1 <= diff && diff <= 1 {return root}
    match diff{
        2 => rotate_left_successor::<K,D>(root),
        -2 => rotate_right_successor::<K,D>(root),
        _ => unreachable!()
    }
}

/// update the cached height of root. To call this function make sure that the cached values of
/// both children of root ar up to date.
fn update_height<K:Ord,D>(root: &mut Node<K,D>){
    root.height = cmp::max( height(&root.left), height(&root.right) )+1;
}

/// recursively insert the (key,data) pair into the given optional succesor and return its new
/// value
fn insert_in_successor<K:Ord,D>(key: K, data: D, successor: Option<Box<Node<K,D>>>)->Option<Box<Node<K,D>>> {
            Some(match successor {
                Some(succ) => insert(key, data, succ),
                None =>Box::new(Node::new(key, data))
            })
}

/// Inserts the given data under the key in the tree root. It will replace old data stored
/// under this key if it was allready used in the tree. The resulting tree will be returned (its
/// root may now differ due to rotations, thus the old root is moved into the function)
pub fn insert<K:Ord,D>(key: K, data: D, mut root: Box<Node<K,D>>) -> Box<Node<K,D>>{
    match root.key.cmp(&key) {
        Ordering::Equal => { root.data  = data; return root },
        Ordering::Less =>    root.right = insert_in_successor(key, data, root.right.take()),
        Ordering::Greater => root.left  = insert_in_successor(key,data, root.left.take())
    }
    update_height(&mut *root);
    return rotate_if_necessary(root)
}

/// returns a read only reference to the data stored under key in the tree given by root
pub fn search<'a, K:Ord,D>(key: &K, root: &'a Box<Node<K,D>>) -> Option<&'a D>{
    search_pair(key,root).map(|(_,v)| v )
}

/// returns a read only reference paie to the data stored under key in the tree given by root
pub fn search_pair<'a, K:Ord,D>(key: &K, root: &'a Box<Node<K,D>>) -> Option<(&'a K,&'a D)>{
    match root.key.cmp(key) {
        Ordering::Equal => Some((&root.key, &root.data)),
        Ordering::Less => root.right.as_ref().map_or(None, |succ| search_pair(key, succ)),
        Ordering::Greater => root.left.as_ref().map_or(None, |succ| search_pair(key, succ))
    }
}


/// returns true iff key is stored in the tree given by root
fn contains<K:Ord,D>(key: &K, root: &Box<Node<K,D>> ) -> bool  {
    search(key,root).is_some()
}


///returns the smallest key and value after the given key.
pub fn min_after<'a, K:Ord,D>(key: &K, root: &'a Box<Node<K,D>>) -> Option<(&'a K,&'a D)> {
    match root.key.cmp(key){
        Ordering::Equal =>  root.right.as_ref().map_or(None, |succ| Some(min_pair(succ))),
        Ordering::Less =>   root.right.as_ref().map_or(None, |succ| min_after(key, succ)),
        Ordering::Greater => {
            match root.left {
                Some(ref succ) => min_after(key, &succ).or( Some((&root.key,&root.data)) ),
                None => Some((&root.key, &root.data))
            }
        }
    }
}

///returns the minimal key,value pair within this tree
pub fn min_pair<K:Ord,D>(root: &Box<Node<K,D>>) -> (&K,&D) {
    root.left.as_ref().map_or((&root.key,&root.data), min_pair)
}

///returns the maximal key,value pair within this tree
pub fn max_pair<K:Ord,D>(root: &Box<Node<K,D>>) -> (&K,&D) {
    root.right.as_ref().map_or((&root.key,&root.data), max_pair)
}

///returns the minimal value within this tree
pub fn min<K:Ord,D>(root: &Box<Node<K,D>>) -> &D {
    root.left.as_ref().map_or(&root.data, min)
}

///returns the minimal value within this tree
pub fn max<K:Ord,D>(root: &Box<Node<K,D>>) -> &D {
    root.right.as_ref().map_or(&root.data, max)
}

//will update_heights and rotate the node if necessary, returns the rotated node
fn updated_node<K:Ord,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>> {
    update_height(&mut root);
    rotate_if_necessary(root)
}

//Performs recursive `drop_and_get_min` if a left  since a successor is available
fn drop_min_from_left<K:Ord,D>(mut root : Box<Node<K,D>>, left: Box<Node<K,D>>) -> (Option<Box<Node<K,D>>>,Box<Node<K,D>>) {
    let (new_left, min) =  drop_min(left);
    root.left = new_left;
    (Some(updated_node(root)),min)
}

//Finds the minimal value below root and returns a new (optional) tree where the minimal value has been
//removed and the (optional) minimal node as tuple (new_tree, min);
fn drop_min<K:Ord,D>(mut root: Box<Node<K,D>>) -> (Option<Box<Node<K,D>>>, Box<Node<K,D>>) {
    match root.left.take() {
        Some(left) => drop_min_from_left(root, left),
        None => (root.right.take(), root)
    }
}

//Return a new AVL tree, as the combination of two subtrees with max(l) <= min(r)
fn combine_two_subtrees<K:Ord,D>(l: Box<Node<K,D>>, r: Box<Node<K,D>>) -> Box<Node<K,D>>{
    let (remaining_tree, min) = drop_min(r);
    let mut new_root = min;
    new_root.left = Some(l);
    new_root.right = remaining_tree;
    updated_node(new_root)
}

//Return a new AVL tree, where the root has been removed
fn delete_root<K:Ord,D>(mut root: Box<Node<K,D>>) -> Option<Box<Node<K,D>>> {
    match ( root.left.take(), root.right.take() ) {
        ( None,     None)    => None,
        ( Some(l),  None)    => Some(l),
        ( None,     Some(r)) => Some(r),
        ( Some(l),  Some(r)) => Some(combine_two_subtrees(l,r))
    }
}


// will delete `key` from the tree `root`. Returns either `Some` tree or if the resilting tree is
// empty: None.
//
//
pub fn delete<K:Ord,D>(key: K, mut root: Box<Node<K,D>>) -> Option<Box<Node<K,D>>>{
    match root.key.cmp(&key){
        Ordering::Equal =>  return delete_root(root),
        Ordering::Less => {
            if let Some(succ) = root.right.take() {
                root.right = delete(key, succ);
                return Some(updated_node(root))
            }
        },
        Ordering::Greater => {
            if let Some(succ) = root.left.take() {
                root.left =  delete(key, succ);
                return Some(updated_node(root))
            }
        }
    }
    return Some(root);
}

fn simple_tree(size: i32) -> Box<Node<u64,i32>> {
    let mut t = Box::new(Node::<u64,i32>{key: 1, data: 1337, height: 0, left:None, right: None});
    for x in 2..size+1 {
        t = insert((x as u64),1337+x-1,t)
    }
    t
}

fn is_sorted_left<K:Ord,D>(node: &Box<Node<K,D>>) -> bool {
    node.left.as_ref().map_or(true, |succ| succ.key < node.key)
}

fn is_sorted_right<K:Ord,D>(node: &Box<Node<K,D>>) -> bool {
    node.right.as_ref().map_or(true, |succ| succ.key > node.key)
}

fn is_avl_node<K:Ord,D>(node: &Box<Node<K,D>>) -> bool {
    let sorted = is_sorted_left(node) && is_sorted_right(node);
    let balanced = node.height == cmp::max(height(&node.left),height(&node.right))+1;
    return sorted && balanced;
}

pub fn is_avl_tree<K:Ord,D>(root: &Option<Box<Node<K,D>>>) -> bool {
    (*root).as_ref().map_or(true, is_avl_node)
}

#[test]
fn simple_tree_operations() {
    let mut t = Box::new(Node::<u64,i32>{key: 3, data: 4, height: 2,
        left: Some(Box::new(Node::<u64,i32>{key: 2, data: 5, height:1, left: None, right: None})), 
        right: None});
    assert!(is_avl_node(&t));
    assert!( contains::<u64,i32>(&3,&t) );
    assert!( contains::<u64,i32>(&2,&t) );
    assert!( !contains::<u64,i32>(&6,&t) );
    assert!( !contains::<u64,i32>(&4,&t) );
    t = insert::<u64,i32>(4,7, t);
    t = insert::<u64,i32>(5,7, t);
    t = insert::<u64,i32>(6,8, t);
    assert!( contains::<u64,i32>(&4,&t) );
    assert!( contains::<u64,i32>(&6,&t) );
    assert!( !contains::<u64,i32>(&7,&t) );
}

#[test]
fn rotations_on_tree(){ 
    let mut t = Box::new(Node::<u64,i32>{key: 1, data: 1337, height: 1, left: None, right: None});
    for i in 2..255 {
        t = insert::<u64,i32>(i,1337, t);
        assert!(is_avl_node(&t));
    }
    //check that the tree is indeed balanced
    assert!(height(&Some(t)) <= 8);
}

#[test]
fn test_drop_min(){
    let mut t = simple_tree(3);
    let (maybe_tree,min) = drop_min(t);
    t = maybe_tree.expect("failure to get tree for first min delete");
    assert!(is_avl_node(&t));
    assert!( min.key == 1);
    assert!(!contains::<u64,i32>(&1,&t));
    assert!(contains::<u64,i32>(&2,&t));
    assert!(contains::<u64,i32>(&3,&t));

    let (maybe_tree,min) = drop_min(t);
    t = maybe_tree.expect("failure to get tree for second min delete");
    assert!(is_avl_node(&t));
    assert!( min.key == 2);
    assert!(!contains::<u64,i32>(&1,&t));
    assert!(!contains::<u64,i32>(&2,&t));
    assert!(contains::<u64,i32>(&3,&t));

    let (maybe_tree,min) = drop_min(t);
    assert!( maybe_tree.is_none() );
    assert!( min.key == 3);
}

#[test]
fn test_drop_root(){
    let mut t = simple_tree(3);
    let maybe_tree = delete_root(t);
    t = maybe_tree.expect("failure to get tree for first root drop");
    assert!(is_avl_node(&t));
    println!("{}",t.to_string());
    assert!( t.height == 2);
    assert!(contains::<u64,i32>(&1,&t));
    assert!(!contains::<u64,i32>(&2,&t));
    assert!(contains::<u64,i32>(&3,&t));

    let maybe_tree = delete_root(t);
    t = maybe_tree.expect("failure to get tree for second root drop");
    assert!(is_avl_node(&t));
    assert!(contains::<u64,i32>(&1,&t));
    assert!(!contains::<u64,i32>(&2,&t));
    assert!(!contains::<u64,i32>(&3,&t));

    let maybe_tree = delete_root(t);
    assert!( maybe_tree.is_none() );
}

#[test]
fn test_delete(){
    let mut t = simple_tree(10);
    for i in 1..10 {
        assert!(contains::<u64,i32>(&i,&t));
        let maybe_tree = delete(i,t);
        t = maybe_tree.expect("failure to get tree for delete");
        assert!(!contains::<u64,i32>(&i,&t));
        assert!(is_avl_node(&t));
    }
    assert!(contains::<u64,i32>(&10,&t));
    let maybe_tree = delete(10,t);
    assert!(maybe_tree.is_none());
}

#[test] 
fn test_min_max() {
    let mut t = simple_tree(50);
    assert_eq!(min(&t),&1337);
    assert_eq!(max(&t),&(1337+50-1));
    assert_eq!(max_pair(&t).0,&50);
    assert_eq!(min_pair(&t).0,&1);
}

#[test]
fn test_min_after(){
    let t = simple_tree(50);
    for old_key in 0..55 {
        println!("trying value: {}", old_key);
        match min_after(&old_key,&t) {
            Some((k,_d)) => assert_eq!(k, &(old_key+1)),
            None => assert!(old_key >= 50)
        }
    }
}
