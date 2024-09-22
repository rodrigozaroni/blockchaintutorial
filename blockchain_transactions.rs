
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u32,
}

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

fn create_block_with_transactions(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Block {
    let timestamp = Utc::now().to_string();
    let mut tx_data = String::new();
    for tx in &transactions {
        tx_data += &format!("{} enviou {} moedas para {}
", tx.sender, tx.amount, tx.receiver);
    }
    let mut block = Block {
        index,
        timestamp,
        data: tx_data,
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
    let genesis_block = create_block_with_transactions(0, vec![], "".to_string());
    let mut blockchain = vec![genesis_block];

    // Criando algumas transações
    let tx1 = Transaction { sender: "Andrej".to_string(), receiver: "BabaYaga".to_string(), amount: 10 };
    let tx2 = Transaction { sender: "BabaYaga".to_string(), receiver: "Dead Party".to_string(), amount: 5 };

    // Criando um novo bloco com transações
    let new_block = create_block_with_transactions(1, vec![tx1, tx2], blockchain[0].hash.clone());
    add_block(&mut blockchain, new_block);

    println!("Blockchain com transações: {:?}", blockchain);
}
