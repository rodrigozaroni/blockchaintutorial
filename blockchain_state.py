
import hashlib
import time

class Block:
    def __init__(self, index, timestamp, data, previous_hash):
        self.index = index
        self.timestamp = timestamp
        self.data = data
        self.previous_hash = previous_hash
        self.hash = self.calculate_hash()

    def calculate_hash(self):
        data_to_hash = f"{self.index}{self.timestamp}{self.data}{self.previous_hash}"
        return hashlib.sha256(data_to_hash.encode()).hexdigest()

def create_block(previous_block, data):
    index = previous_block.index + 1
    timestamp = str(time.time())
    return Block(index, timestamp, data, previous_block.hash)

def add_block_to_chain(blockchain, block):
    blockchain.append(block)

if __name__ == "__main__":
    blockchain = [Block(0, str(time.time()), "Bloco Gênese", "0")]

    new_block = create_block(blockchain[0], "Transação 1: Andrej enviou 10 moedas para BabaYaga")
    add_block_to_chain(blockchain, new_block)

    for block in blockchain:
        print(f"Bloco: {block.__dict__}")
