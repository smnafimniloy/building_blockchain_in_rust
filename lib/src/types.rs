use crate::U256;
use uuid:: Uuid;
use chrono:: {DateTime, Utc};

pub struct Blockchain {
    pub blocks:Vec<Block>,
}

pub struct Block{
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

pub struct BlockHeader{
    //Timestamp of the block
    pub timestamp: u64,
    // Nonce used to mine the block
    pub nonce: u64, /* Number only used once, we increment it to mine the block */
    // Hash of the previous block
    pub prev_block_hash: [u8;32],
    // Merkle root of the block's transactions
    pub merkle_root: [u8; 32],
    // Target
    pub target: U256, /* A number which has to be higher than the hash of this block for it to be considered valid */
}

pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

pub struct TransactionInput{
    pub prev_transaction_output_hash: [u8; 32],
    pub signature: [u8; 64], // dummy types, will be replaced later
}

pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: [u8; 33], // dummy types, will be replaced later
}


impl Blockchain {
    pub fn new () -> Self {
        Blockchain {
            blocks: vec![]
        }
    }
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Block {
    pub fn new (
        header: BlockHeader,
        transactions: Vec<Transaction>,
    ) -> Self {
        Block {
            header: header,
            transactions : transactions,
        }
    }
    pub fn hash(&self) -> ! {
        unimplemented!() /* also TODO!() can be used */
    }
}

impl BlockHeader {
    pub fn new (
        timestamp: u64,
        nonce: u64,
        prev_block_hash: [u8; 32],
        merkle_root: [u8; 32],
        target: U256,
    ) -> Self {
        BlockHeader { 
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

impl Transaction {
    pub fn new (
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>,
    ) -> Self {
        Transaction { 
            inputs:inputs, 
            outputs: outputs, 
        }
    }

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

