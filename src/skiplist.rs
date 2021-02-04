use std::iter;

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    key: Option<T>,
    next: Vec<Link<T>>,
}

impl<T> Node<T>
where
    T: Clone,
{
    fn new(key: T) -> Node<T> {
        Node {
            key: Some(key),
            next: vec![None],
        }
    }

    fn new_head(height: usize) -> Node<T> {
        Node {
            key: None,
            next: iter::repeat(None).take(height).collect(),
        }
    }

    fn next(&self, level: usize) -> Option<Link<T>> {
        assert!(level >= 0);
        Some(self.next[level])
    }

    fn set_next(&mut self, n: Node<T>) -> () {
        self.next.push(Some(Box::new(n)));
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        Node {
            key: self.key.clone(),
            next: self.next.clone(),
        }
    }
}

struct SkipList<T> {
    head: Link<T>,
    k_max_height: u16,
    max_height: u16,
    branching_factor: u16,
}

impl<T> SkipList<T>
where
    T: Clone,
    T: Ord,
{
    pub fn insert() {}
    pub fn contains() {}
    pub fn new(max_height: usize, branching_factor: u16) -> SkipList<T> {
        SkipList {
            max_height: 1,
            k_max_height: max_height as u16,
            branching_factor: branching_factor,
            head: Some(Box::new(Node::new_head(max_height))),
        }
    }

    fn get_max_height(&self) -> u16 {
        self.max_height
    }

    fn random_height() -> u16 {
        0
    }

    fn equal(a: &T, b: &T) -> bool {
        a == b
    }

    fn less_than(a: &T, b: &T) -> bool {
        a < b
    }
    // fn key_is_after_node(k: &T, n: Node<T>) -> bool {}
    // fn find_greater_or_equal(k: &T) -> Node<T> {}
    // fn find_less_than(k: &T) -> Node<T> {}
    // fn find_last() -> Node<T> {}

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next[0].as_deref();
//             &node.key
//         })
//     }
// }
