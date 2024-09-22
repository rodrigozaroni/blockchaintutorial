
use sha2::{Sha256, Digest};
use std::fmt;

// Estrutura de um bloco
#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}

// Implementação para calcular o hash de um bloco
impl Block {
    fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

// Função para criar um novo bloco
fn create_block(index: u32, data: String, previous_hash: String) -> Block {
    let timestamp = chrono::Utc::now().to_string();
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

fn main() {
    // Criando o bloco gênese
    let genesis_block = create_block(0, "Bloco Gênese".to_string(), "".to_string());
    println!("Bloco Gênese: {:?}", genesis_block);
}
