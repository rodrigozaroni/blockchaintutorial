
package main

import (
    "crypto/sha256"
    "encoding/hex"
    "fmt"
    "strconv"
    "time"
)

// Estrutura de uma transação
type Transaction struct {
    Sender    string
    Receiver  string
    Amount    int
}

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

// Função para criar um novo bloco com transações
func createBlockWithTransaction(prevBlock Block, transactions []Transaction) Block {
    var txData string
    for _, tx := range transactions {
        txData += fmt.Sprintf("%s enviou %d moedas para %s\n", tx.Sender, tx.Amount, tx.Receiver)
    }
    newBlock := Block{
        Index:        prevBlock.Index + 1,
        Timestamp:    time.Now().String(),
        Data:         txData,
        PreviousHash: prevBlock.Hash,
        Hash:         "",
    }
    newBlock.Hash = calculateHash(newBlock)
    return newBlock
}

func main() {
    genesisBlock := Block{0, time.Now().String(), "Bloco Gênese", "", ""}
    genesisBlock.Hash = calculateHash(genesisBlock)
    Blockchain = append(Blockchain, genesisBlock)

    // Criando algumas transações
    tx1 := Transaction{"Andrej", "BabaYaga", 10}
    tx2 := Transaction{"BabaYaga", "Dead Party", 5}

    // Criando um novo bloco com transações
    newBlock := createBlockWithTransaction(genesisBlock, []Transaction{tx1, tx2})
    addBlock(newBlock)

    fmt.Println("Blockchain com transações:", Blockchain)
}
