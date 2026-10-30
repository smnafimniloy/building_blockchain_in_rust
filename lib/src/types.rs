pub struct Blockchain {
    pub blocks:Vec<Block>,
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
pub struct Block;
pub struct BlockHeader;
pub struct Transaction;

