use rand::Rng;
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
    fn new(key: T, height: u16) -> Node<T> {
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

    fn next(&self, height: u16) -> Link<T> {
        assert!(height >= 0);
        self.next[height as usize]
    }

    fn set_next(&mut self, height: u16, n: Link<T>) -> () {
        assert!(height >= 0);
        self.next[height as usize] = n;
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
    inverse_branching: f64,
}

impl<T> SkipList<T>
where
    T: Clone,
    T: Ord,
{
    pub fn contains() {}
    pub fn new(max_height: usize, branching_factor: u16) -> SkipList<T> {
        SkipList {
            max_height: 1,
            k_max_height: max_height as u16,
            branching_factor: branching_factor,
            head: Some(Box::new(Node::new_head(max_height))),
            inverse_branching: 1.0 / branching_factor as f64,
        }
    }

    fn get_max_height(&self) -> u16 {
        self.max_height
    }

    fn random_height(&self) -> u16 {
        let mut lvl = 1;
        while lvl < self.max_height && rand::thread_rng().gen::<f64>() < self.inverse_branching {
            lvl += 1;
        }
        lvl
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

    pub fn insert(&mut self, key: T) {
        let mut update: Vec<Link<T>> = iter::repeat(None)
            .take(self.k_max_height as usize)
            .collect();
        let mut x = self.head;
        for i in (0..self.max_height).rev() {
            loop {
                if x.is_none() {
                    break;
                }
                let node = *x.unwrap();
                if SkipList::key_is_after_node(&key, node.next(i)) {
                    break;
                }
                x = x.unwrap().next(i);
            }
            update[i as usize] = x;
        }
        // Here we will know if key is in list or not as we keep values in a HashMap
        let height = self.random_height();
        if height > self.get_max_height() {
            self.max_height = height;
        }
    }

    // fn find_greater_or_equal(k: &T) -> Node<T> {}

    //     fn find_less_than(&self, k: &T) -> Node<T> {
    //         let x = self.head;
    //         let mut lvl = self.get_max_height();
    //         let last_not_after = None;
    //         while x.is_some() {
    //             let next = x.unwrap();
    //         }
    //         Node::new(T::default())
    //     }

    // fn find_last() -> Node<T> {}

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: Some(&self.head.unwrap()),
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
