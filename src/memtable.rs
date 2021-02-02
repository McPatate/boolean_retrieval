use serde::{Deserialize, Serialize};

const MAX_CAPACITY: usize = 1024 * 1024 * 1024;

struct MemTable<Key> {
    entries: HashMap<Key, Vec<u8>>,
    skip_list: Vec<Key>,
    size: usize,
}

impl MemTable {
    fn new() -> MemTable {}

    fn write_to_wal(block: Block) {}

    fn insert(p: &mut Vec<usize>, v: usize) {
        let mut low = 0;
        let mut high = p.len() - 1;
        let ipos;

        while low <= high {
            let mid = (high + low) / 2;
            if p[mid] < v {
                low = mid + 1
            } else if p[mid] > v {
                high = mid - 1
            }
        }
        if low > high {
            ipos = high + 1;
        } else {
            ipos = low
        }
        let upper_bound = v + 1;
        p.splice(ipos..ipos, v..upper_bound);
    }
}
