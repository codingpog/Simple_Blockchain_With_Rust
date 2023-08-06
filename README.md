# Blockchain Implementation in Rust

This repository contains a simple implementation of a blockchain in Rust. The blockchain consists of a `Block` struct that represents individual blocks in the chain, and a `WorkQueue` struct that manages parallel mining tasks.

## Table of Contents

- [Introduction](#introduction)
- [Block](#block)
- [WorkQueue](#workqueue)
- [Usage](#usage)
  
## Introduction

This project aims to demonstrate the basic concepts of a blockchain and parallel mining using Rust. It provides a simplified implementation of a blockchain with functionalities like block creation, mining, and validation.

## Block

The `Block` struct represents an individual block in the blockchain. Each block contains the following attributes:

- `prev_hash`: The hash of the previous block in the chain.
- `generation`: The generation number of the block.
- `difficulty`: The difficulty level of mining (number of leading zeros required in the block's hash).
- `data`: The data stored in the block (currently represented as a string).
- `proof`: The proof of work (PoW) value that satisfies the mining difficulty.

The `Block` struct provides methods for block initialization, block mining, and validation.

## WorkQueue

The `WorkQueue` struct is responsible for managing parallel mining tasks. It allows multiple worker threads to work concurrently on mining tasks. The `WorkQueue` is implemented using a Single-Producer Multi-Consumer (SPMC) channel to handle task distribution and result collection.

## Usage

To use this blockchain implementation, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/codingpog/blockchain-rust.git

2. Import the required modules at the beginning of your Rust file.
   ```bash
    use crate::block::Block;
    use crate::queue::WorkQueue;

3. Create and mine blocks
   ```bash
    // Create an initial block with a specified difficulty level.
    let difficulty = 3;
    let initial_block = Block::initial(difficulty);
    
    // Create a new block that follows the previous block.
    let data = "Transaction data here".to_string();
    let next_block = Block::next(&previous_block, data);
    
    // Mine the block using a specified number of worker threads.
    let workers = 4;
    next_block.mine(workers);
    
    // Validate the mined block.
    assert!(next_block.is_valid());

4. Use WorkQueue for parallel mining
   ```bash
   // Create a WorkQueue with a specified number of worker threads.
    let workers = 8;
    let mut queue = WorkQueue::<Block>::new(workers);
    
    // Enqueue blocks for mining.
    queue.enqueue(next_block);
    
    // Receive the valid proof from the first worker that finds it.
    let valid_proof = queue.recv();
    
    // Shutdown the WorkQueue when done.
    queue.shutdown();



