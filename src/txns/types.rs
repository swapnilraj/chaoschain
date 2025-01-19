#[derive(Debug)]
pub struct Txn {
    pub sender: u64,
    pub message: String,
}

#[derive(Debug)]
pub struct SignedTxn {
    pub txn: Txn,
    pub signature: String,
}

impl Txn {
    pub fn new(sender: u64, message: String) -> Self {
        Self { 
            sender, 
            message 
        }
    }
}

impl SignedTxn {
    pub fn new(txn: Txn, signature: String) -> Self {
        // TODO: Add signature validation logic
        Self { 
            txn, 
            signature 
        }
    }
} 