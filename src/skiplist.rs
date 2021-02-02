type Link<T> = Option<Box<Node<T>>>;

struct Node<Key> {
    key: Key,
    next: Link<Key>,
}

struct SkipList {}
