
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

fn create_block(index: u32, data: String, previous_hash: String) -> Block {
    let timestamp = Utc::now().to_string();
    let mut block = Block {
        index,
        timestamp,
        data,
        previous_hash,
        hash: String::new(),
    };
    block.hash = block.calculate_hash();
    block
}

fn add_block(blockchain: &mut Vec<Block>, new_block: Block) {
    blockchain.push(new_block);
}

fn main() {
    // Criando o bloco gênese
    let genesis_block = create_block(0, "Bloco Gênese".to_string(), "".to_string());
    let mut blockchain = vec![genesis_block];

    // Adicionando novos blocos
    let new_block = create_block(1, "Transação 1: Andrej enviou 10 moedas para BabaYaga".to_string(), blockchain[0].hash.clone());
    add_block(&mut blockchain, new_block);

    println!("Blockchain: {:?}", blockchain);
}
