struct Block {
    index: u64,
    timestamp: u64,
    txns: Vec<SignedTxn>,
    previous_hash: String,
    nonce: u64,
}

impl Block {
    pub fn new(index: u64, timestamp: u64, txns: Vec<SignedTxn>, previous_hash: String, nonce: u64) -> Self {
        Self { index, timestamp, txns, previous_hash, nonce }
    }
}