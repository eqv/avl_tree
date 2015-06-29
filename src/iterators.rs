use ::tree;
use ::node;
use std::collections::Bound;

pub struct RangePairIter<'a, K:'a+Ord+Copy,D:'a> {
    tree: &'a tree::AVLTree<K, D>,
    from: Bound<K>,
    to: Bound<K>,
    prev: Option<&'a K>,
}

impl<'a, K:'a+Ord+Copy,D:'a> RangePairIter<'a, K, D> {

    pub fn new(tree: &'a tree::AVLTree<K,D>, lower: Bound<K>, upper: Bound<K>) -> RangePairIter<'a,K,D>{
        RangePairIter{tree: tree, from: lower, to: lower, prev:None}
    }

    fn get_next_key_under(&mut self, root: &'a Box<node::Node<K,D>>) -> Option<(&'a K,&'a D)>{
        let res = self.get_next_pair(root).and_then(|p| self.check_upper_bound(p));
        match res {
            None => return None,
            Some((key,val)) => { 
                self.prev = Some(key);
                return Some((key,val))
            }
        }
    }

    fn get_next_pair(&mut self, root: &'a Box<node::Node<K,D>>) -> Option<(&'a K, &'a D)>{
        match self.prev{
            None => self.get_lower_bound_pair(root),
            Some(key) => node::min_after::<K,D>(key, root)
        }
    }

    fn get_lower_bound_pair(&self, root: &'a Box<node::Node<K,D>>) -> Option<(&'a K, &'a D)>{
        match self.from {
            Bound::Included(ref key) => node::search_pair(key, root).or_else(|| node::min_after(key, root)),
            Bound::Excluded(ref key) => node::min_after(key, root),
            Bound::Unbounded => Some(node::min_pair(root))
        }
    }

    fn check_upper_bound(&self, current: (&'a K, &'a D)) -> Option<(&'a K, &'a D)> {
        let ok = match self.to {
            Bound::Included(ref key) => current.0 <= key,
            Bound::Excluded(ref key) => current.0 < key,
            Bound::Unbounded => true
        };
        return if ok { Some(current) } else { None };
    }
}

impl<'a, K:'a+Ord+Copy,D:'a> Iterator for RangePairIter<'a, K, D> {

    type Item = (&'a K,&'a D);

    fn next(&mut self) -> Option<(&'a K,&'a D)> {
        match self.tree.root {
            None => return None,
            Some(ref node) => {
                self.get_next_key_under(node)
            }
        }
    }


}

#[test]
fn test_iterators(){
    let mut tree = tree::AVLTree::<u64,i32>::new();
    tree.insert(18, 1337);
    tree.insert(13, 1338);
    tree.insert(17, 1339);
    tree.insert(10, 1321);
    tree.insert(1, 1321);
    tree.insert(3, 1322);
    let init_key = 0;
    let mut iter = RangePairIter::<u64,i32>{tree: &tree, prev: Some(&init_key), from: Bound::Unbounded, to: Bound::Unbounded};
    assert!(iter.next().expect("should have a few values").0 == &1);
    assert!(iter.next().expect("should have a few values").0 == &3);
    assert!(iter.next().expect("should have a few values").0 == &10);
    assert!(iter.next().expect("should have a few values").0 == &13);
    assert!(iter.next().expect("should have a few values").0 == &17);
    assert!(iter.next().expect("should have a few values").0 == &18);
    assert!(iter.next().is_none());
}
