/// `Block` is the smallest unit of data
/// it contains a meta data about the block, the key & the value
///
/// Layout :
///
/// ```text
/// +-------------+----------------+--------------+---------------+-------------+
/// | uuid (u128) | deleted (bool) | version(u32) | key (Vec<u8>) | value (u32) |
/// +-------------+----------------+--------------+---------------+-------------+
/// ```

struct Block {
    pub id: u128,
    pub deleted: bool,
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
}
