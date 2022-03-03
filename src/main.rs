use sha2::{Digest, Sha256};
use std::{fmt::Debug, time::SystemTime};

struct Block {
    height: u32,
    hash: [u8; 32],
    previous_hash: [u8; 32],
    data: String,
    timestamp: u128,
}

impl Block {
    fn new(
        height: u32,
        hash: [u8; 32],
        previous_hash: [u8; 32],
        data: String,
        timestamp: u128,
    ) -> Block {
        Block {
            height,
            hash,
            previous_hash,
            data,
            timestamp,
        }
    }
}

struct BlockChain {
    blocks: Vec<Block>,
}

impl Debug for BlockChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self
            .blocks
            .iter()
            .fold("".to_string(), |acc: String, item: &Block| {
                format!("{}->{:?}", acc, item.data)
            });

        write!(f, "{}", output)
    }
}
impl BlockChain {
    fn new() -> BlockChain {
        BlockChain { blocks: vec![] }
    }

    fn calculate_hash(
        data: &String,
        previous_hash: &[u8; 32],
        height: &u32,
        timestamp: &u128,
    ) -> [u8; 32] {
        let mut sha = Sha256::new();

        let payload = [
            previous_hash.as_slice(),
            data.as_bytes(),
            height.to_string().as_bytes(),
            timestamp.to_string().as_bytes(),
        ]
        .concat();
        sha.update(payload);
        sha.finalize()
            .as_slice()
            .try_into()
            .expect("failed to create hash")
    }

    fn get_latest_block(&self) -> Result<&Block, ()> {
        let height = self.blocks.len();
        if height == 0 {
            return Err(());
        } else {
            Ok(&self.blocks[self.blocks.len() - 1])
        }
    }
    fn generate_next_block(&mut self, block_data: String) {
        let previous_block_result = self.get_latest_block();

        match previous_block_result {
            Ok(previous_block) => {
                let next_height = previous_block.height + 1;
                let next_timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("could not get system time")
                    .as_millis();
                let next_hash = BlockChain::calculate_hash(
                    &block_data,
                    &previous_block.hash,
                    &next_height,
                    &next_timestamp,
                );
                let new_block = Block::new(
                    next_height,
                    previous_block.hash,
                    next_hash,
                    block_data,
                    next_timestamp,
                );
                self.blocks.push(new_block);
            }
            Err(()) => {
                let next_timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("could not get system time")
                    .as_millis();
                let next_hash =
                    BlockChain::calculate_hash(&block_data, &[0; 32], &0, &next_timestamp);
                let new_block = Block::new(0, next_hash, [0; 32], block_data, next_timestamp);
                self.blocks.push(new_block);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut blockchain = BlockChain::new();
    blockchain.generate_next_block("first block".to_string());
    blockchain.generate_next_block("second block".to_string());
    blockchain.generate_next_block("third block".to_string());
    blockchain.generate_next_block("fourth block".to_string());

    println!("{:?}", blockchain);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
