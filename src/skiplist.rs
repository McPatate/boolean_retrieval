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

    fn key(&self) -> Option<&T> {
        self.key.as_ref()
    }

    fn next(&self, level: usize) -> Link<T> {
        assert!(level >= 0);
        self.next[level]
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
    fn key_is_after_node(k: &T, n: Link<T>) -> bool {
        let is_some = n.is_some();
        let next_smaller = if is_some {
            // will panic if n is head
            let next_key = n.unwrap().key().unwrap();
            next_key < k
        } else {
            false
        };
        next_smaller
    }

    // fn find_greater_or_equal(k: &T) -> Node<T> {}

    fn find_less_than(&self, k: &T) -> Node<T> {
        let x = self.head;
        let mut lvl = self.get_max_height();
        let last_not_after = None;
        while x.is_some() {
            let next = x.unwrap();
        }
        Node::new(T::default())
    }

    // fn find_last() -> Node<T> {}

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.map(|node| {
            self.next = node.next[0].as_deref();
            &node.key
        }) {
            Some(k) => k.as_ref(),
            None => None,
        }
    }
}
