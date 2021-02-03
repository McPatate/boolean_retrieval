pub struct IntoIter<T>(SkipList<T>);

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    key: T,
    next: Vec<Link<T>>,
}

impl Node<T> {
    fn next(&self, level: u8) -> Node<T> {
        assert!(level >= 0);
        self.next[level]
    }

    fn set_next(&self, level: u8, n: Node<T>) -> () {
        assert!(level >= 0);
        self.next[level] = n;
    }
}

struct SkipList<T> {
    head: Node<T>,
    max_height: u8,
}

impl<T> SkipList<T>
where
    T: Eq,
    T: Ord,
{
    pub fn insert() {}
    pub fn contains() {}

    fn get_max_height(&self) -> u8 {
        self.max_height
    }

    fn random_height() -> u8 {}

    fn equal(a: &T, b: &T) -> bool {
        a == b
    }

    fn less_than(a: &T, b: &T) -> bool {
        a < b
    }
    fn key_is_after_node(k: &T, n: Node<T>) -> bool {}
    fn find_greater_or_equal(k: &T) -> Node<T> {}
    fn find_less_than(k: &T) -> Node<T> {}
    fn find_last() -> Node<T> {}

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&self) -> Option<Self::Item> {}
}
