# blockchaintutorial
Codes for Blockchain Tutorial Go (+ versões em Rust e Python)

### Tutorial: Desenvolvendo um Blockchain Básico em Go

Neste tutorial, vamos guiar você através do processo de criação de um blockchain básico em Golang. O código será dividido em três seções principais: **Bloco Gênese**, **Estado Global** e **Transações**. Para facilitar, você pode acessar os códigos diretamente no GitHub nos arquivos mencionados durante o tutorial.

#### Requisitos:

- Go (Golang) instalado em seu sistema.
- Familiaridade básica com Golang.
- Editor de código de sua preferência.
  
#### Seções:

1. **Bloco Gênese** – Criando o primeiro bloco do blockchain
2. **Estado Global** – Modificando o estado global do blockchain com novos blocos
3. **Transações** – Adicionando transações entre usuários no blockchain

---

### 1. Bloco Gênese

O bloco gênese é o primeiro bloco de qualquer blockchain. Ele serve como ponto de partida, e todos os outros blocos se conectam a ele. Aqui, criaremos um bloco com um índice, um timestamp, dados (neste caso, uma string), um hash anterior (que será vazio no bloco gênese) e o hash do bloco atual.

#### Explicação do código:
- **Bloco**: Estrutura que contém as informações de cada bloco, incluindo o índice, timestamp, dados, hash anterior e o hash atual.
- **calculateHash**: Função responsável por calcular o hash do bloco. Usamos SHA-256 para garantir a segurança e imutabilidade.
- **createBlock**: Cria um novo bloco usando os dados e o hash do bloco anterior.

Quando você executa este código, ele cria e imprime o bloco gênese. Esse bloco inicializa a cadeia, permitindo a adição de novos blocos.

---

### 2. Estado Global

Agora que temos o bloco gênese, o próximo passo é manter o estado global do blockchain. Isso significa que, à medida que novos blocos são adicionados, o blockchain deve armazená-los de maneira encadeada.

#### Explicação do código:
- **Blockchain**: Um array que armazena a lista de blocos, representando a cadeia.
- **addBlock**: Função que adiciona um novo bloco à blockchain.
  
Neste exemplo, criamos o bloco gênese e logo em seguida adicionamos um novo bloco que inclui uma transação fictícia ("Andrej enviou 10 moedas para BabaYaga"). A blockchain agora contém dois blocos.

---

### 3. Transações

A verdadeira utilidade de um blockchain está em registrar transações entre usuários. Cada bloco pode conter várias transações e, para isso, criamos uma estrutura específica para encapsular os detalhes de cada transação.

#### Explicação do código:
- **Transaction**: Estrutura que armazena os dados de uma transação, como remetente, destinatário e valor transferido.
- **createBlockWithTransaction**: Função que cria um bloco contendo várias transações e as agrupa em um único bloco.

Neste exemplo, criamos duas transações: uma onde **Andrej** envia 10 moedas para **BabaYaga**, e outra onde **BabaYaga** envia 5 moedas para **Dead Party**. Essas transações são agrupadas e armazenadas no novo bloco, que é então adicionado à blockchain.
