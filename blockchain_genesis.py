
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

def create_genesis_block():
    return Block(0, str(time.time()), "Bloco Gênese", "0")

if __name__ == "__main__":
    genesis_block = create_genesis_block()
    print(f"Bloco Gênese: {genesis_block.__dict__}")
