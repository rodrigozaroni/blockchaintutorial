
package main

import (
    "crypto/sha256"
    "encoding/hex"
    "fmt"
    "strconv"
    "time"
)

// Estrutura de um bloco
type Block struct {
    Index        int
    Timestamp    string
    Data         string
    PreviousHash string
    Hash         string
}

// Função para calcular o hash de um bloco
func calculateHash(block Block) string {
    record := strconv.Itoa(block.Index) + block.Timestamp + block.Data + block.PreviousHash
    hash := sha256.New()
    hash.Write([]byte(record))
    hashed := hash.Sum(nil)
    return hex.EncodeToString(hashed)
}

// Função para criar um novo bloco
func createBlock(prevBlock Block, data string) Block {
    newBlock := Block{
        Index:        prevBlock.Index + 1,
        Timestamp:    time.Now().String(),
        Data:         data,
        PreviousHash: prevBlock.Hash,
        Hash:         "",
    }
    newBlock.Hash = calculateHash(newBlock)
    return newBlock
}

func main() {
    // Criando o bloco gênese (primeiro bloco)
    genesisBlock := Block{0, time.Now().String(), "Bloco Gênese", "", ""}
    genesisBlock.Hash = calculateHash(genesisBlock)

    fmt.Println("Bloco Gênese:", genesisBlock)
}
