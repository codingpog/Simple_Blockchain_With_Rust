use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::sync;
//use hex_literal::hex;

type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]
pub struct Block {
    prev_hash: Hash,
    generation: u64,
    difficulty: u8,
    data: String,
    proof: Option<u64>,
}

impl Block {
    pub fn initial(difficulty: u8) -> Block {
         // create and return a new initial block
        let prev_hash = Hash::default();
        let generation = 0;
        let data = String::new();
        Block{prev_hash, generation, difficulty, data, proof: None,}
    }

    pub fn next(previous: &Block, data: String) -> Block {
        // create and return a block that could follow `previous` in the chain
        let prev_hash = previous.hash();
        let generation = previous.generation + 1;
        Block{prev_hash, generation, difficulty: previous.difficulty, data, proof: None,}
    }

    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        // todo() return the hash string this block would have if we set the proof to `proof`.
        let mut hash_string = String::new();
        write!(&mut hash_string, "{:x}", self.prev_hash).unwrap();
        write!(&mut hash_string, ":{}:", self.generation).unwrap();
        write!(&mut hash_string, "{}:", self.difficulty).unwrap();
        write!(&mut hash_string, "{}:", self.data).unwrap();
        write!(&mut hash_string, "{}", proof).unwrap();
        hash_string
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        //todo!(); // return the block's hash as it would be if we set the proof to `proof`.

        //create a sha256 object
        let mut hasher = Sha256::new();
    
        hasher.update(self.hash_string_for_proof(proof).as_bytes());

        //read hash digest and consume hasher
        hasher.finalize()

        // let mut output = String::new();
        // write!(&mut output, "{:02x}", result).unwrap();
        // output
    }

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(self: &mut Block, proof: u64) {
        self.proof = Some(proof);
    }

    pub fn is_valid_for_proof(&self, proof: u64) -> bool {
        // would this block be valid if we set the proof to `proof`?

        let hash = self.hash_for_proof(proof);
        let n_bytes = self.difficulty/8;
        let n_bits = self.difficulty%8;

        for i in (hash.len() - n_bytes as usize)..hash.len() {
            if hash[i as usize] != 0u8 {
                return false;
            }
        }

        if n_bits > 0 {
            if hash[hash.len() - n_bytes as usize - 1] & ((1 << n_bits) - 1) != 0 {
                return false;
            }
        }

        true
    }

    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    // Mine in a very simple way: check sequentially until a valid hash is found.
    // This doesn't *need* to be used in any way, but could be used to do some mining
    // before your .mine is complete. Results should be the same as .mine (but slower).
    pub fn mine_serial(self: &mut Block) {
        let mut p = 0u64;
        while !self.is_valid_for_proof(p) {
            p += 1;
        }
        self.proof = Some(p);
    }

    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, chunks: u64) -> u64 {
        // With `workers` threads, check proof values in the given range, breaking up
	// into `chunks` tasks in a work queue. Return the first valid proof found.
        // HINTS:
        // - Create and use a queue::WorkQueue.
        // - Use sync::Arc to wrap a clone of self for sharing.
        //todo!();
        let mut q = WorkQueue::<MiningTask>::new(workers);
        let chunks_length = (end - start +1)/chunks;
        let mut start_chunk = start;
        let mut end_chunk = start + chunks_length;

        // while loop to split into chunks of equal length to send to workers
        let mut i = start;
        while i <= end {
            let block_clone = sync::Arc::<Block>::new(self.clone());// to share between threads safely
            q.enqueue(MiningTask::new(block_clone, start_chunk, end_chunk));
            start_chunk += chunks_length + 1;
            end_chunk += chunks_length + 1;
            i += chunks_length + 1;
        }

        // the remaining proofs for the last chunk
        let remaining_proofs: u64 = (end - start + 1)%chunks;
        if remaining_proofs != 0 {
            start_chunk = end.saturating_sub(remaining_proofs);
            end_chunk = end;
            let block_clone = sync::Arc::<Block>::new(self.clone());
            q.enqueue(MiningTask::new(block_clone, start_chunk, end_chunk));
        }

        let valid_proof = q.recv();
        q.shutdown();
        valid_proof
    }

    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}

struct MiningTask {
    block: sync::Arc<Block>,
    //todo!(); // more fields as needed
    start: u64,
    end: u64,
}

impl MiningTask {
    //todo!(); // implement MiningTask::new(???) -> MiningTask
    pub fn new(block: sync::Arc<Block>, start: u64, end: u64) -> MiningTask {
        MiningTask{
            block, 
            start, 
            end,
        }
    }
}

impl Task for MiningTask {
    type Output = u64;

    fn run(&self) -> Option<u64> {
        //todo!(); // what does it mean to .run?
        for proof in self.start..=self.end {
            if self.block.is_valid_for_proof(proof) {
                return Some(proof);
            }
        }
        return None;


    }
}
