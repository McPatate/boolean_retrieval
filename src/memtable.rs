use serde::{Deserialize, Serialize};

struct MemTable<T> {
    idx: HashMap<String, T>,
}
