use crate::U256;
use uuid:: Uuid;
use chrono:: {DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::crypto::{PublicKey, Signature};
use crate::error::{BtcError, Result};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub utxos: HashMap<hash, TransactionOutput>,
    pub blocks:Vec<Block>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block{
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader{
    //Timestamp of the block
    pub timestamp: DateTime<Utc>,
    // Nonce used to mine the block
    pub nonce: u64, /* Number only used once, we increment it to mine the block */

    /// Hash of the previous block
    pub prev_block_hash: Hash,
    /// Merkle root of the block's transactions
    pub merkle_root: MerkleRoot,
    /// target
    pub target: U256, /* A number which has to be higher than the hash of this block for it to be considered valid */
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput{
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: PublicKey,
}


impl Blockchain {
    pub fn new () -> Self {
        Blockchain {
            utxos: HashMap::new(),
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
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

impl BlockHeader {
    pub fn new (
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
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
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
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

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
}
}

pub fn add_block (
    &mut self, 
    block: Block,
)-> Result<()> {
    if self.blocks.is_empty() {
        if block.header.prev_block_hash != Hash::zero()
        {
            println!("zero hash");
            return Err(BtcError::InvalidBlock);
        }
    } else {
        let last_block = self.blocks.last().unwrap();
        if block.header.prev_block_hash != last_block.hash()
        {
            println!("prev hash is wrong");
            return Err(BtcError::InvalidBlock);
        }
        if !block
            .header
            .hash()
            .matches_target(block.header.target)
        {
                println!("does not match target");
                return Err(BtcError::InvalidBlock);
        }
        if !block
            .header
            .hash()
            .matches_target(block.header.target)
        {
            println!("does not match target");
            return Err(BtcError::InvalidBlock);
        }
        let calculated_merkle_root = 
            MerkleRoot::calculate(&block.transactions);
        if calculated_merkle_root
            != block.header.merkle_root
        {
            println!("invalid merkle_root");
            return Err(BtcError::InvalidMerkleRoot);
        }

        if block.header.timestamp 
            <= last_block.header.timestamp
        {
            return Err(BtcError::InvalidBlock);
        }

        block.verify_transactions(
            self.block_height(),
            &self.utxos,
        )?;
    }
    self.blocks.push(block);
    Ok(())
}