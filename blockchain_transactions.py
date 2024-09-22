
import hashlib
import time

class Transaction:
    def __init__(self, sender, receiver, amount):
        self.sender = sender
        self.receiver = receiver
        self.amount = amount

class Block:
    def __init__(self, index, timestamp, transactions, previous_hash):
        self.index = index
        self.timestamp = timestamp
        self.transactions = transactions
        self.previous_hash = previous_hash
        self.hash = self.calculate_hash()

    def calculate_hash(self):
        data_to_hash = f"{self.index}{self.timestamp}{self.transactions}{self.previous_hash}"
        return hashlib.sha256(data_to_hash.encode()).hexdigest()

def create_block_with_transactions(previous_block, transactions):
    index = previous_block.index + 1
    timestamp = str(time.time())
    return Block(index, timestamp, transactions, previous_block.hash)

def add_block_to_chain(blockchain, block):
    blockchain.append(block)

if __name__ == "__main__":
    blockchain = [Block(0, str(time.time()), [], "0")]

    tx1 = Transaction("Andrej", "BabaYaga", 10)
    tx2 = Transaction("BabaYaga", "Dead Party", 5)

    new_block = create_block_with_transactions(blockchain[0], [tx1.__dict__, tx2.__dict__])
    add_block_to_chain(blockchain, new_block)

    for block in blockchain:
        print(f"Bloco: {block.__dict__}")
