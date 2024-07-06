
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Block {
    id: u64,
    timestamp: u128,
    data: String,
    validator_id: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(id: u64, data: String, validator_id: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = Block::calculate_hash(id, &data, &validator_id, timestamp, &previous_hash);
        Block {
            id,
            timestamp,
            data,
            validator_id,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(id: u64, data: &str, validator_id: &str, timestamp: u128, previous_hash: &str) -> String {
        format!("{:x}", md5::compute(format!("{}{}{}{}{}", id, data, validator_id, timestamp, previous_hash)))
    }
}

struct Validator {
    id: String,
    stake: u64,
    delegated_stake: u64,
    rotation_period: u64,
    last_block_validated: u64,
    reputation: f64,
}

struct TokenHolder {
    id: String,
    stake: u64,
    delegated_to: Option<String>,
}

struct Blockchain {
    blocks: Vec<Block>,
    pending_blocks: HashSet<Block>,
    validators: HashMap<String, Validator>,
    token_holders: HashMap<String, TokenHolder>,
    current_period: u64,
    finality_threshold: u64,
    security_measures: SecurityMeasures,
}

struct SecurityMeasures {
    malicious_activity_log: HashSet<String>,
    validator_penalties: HashMap<String, u64>,
}

impl Blockchain {
    fn new(finality_threshold: u64) -> Self {
        Blockchain {
            blocks: Vec::new(),
            pending_blocks: HashSet::new(),
            validators: HashMap::new(),
            token_holders: HashMap::new(),
            current_period: 0,
            finality_threshold,
            security_measures: SecurityMeasures {
                malicious_activity_log: HashSet::new(),
                validator_penalties: HashMap::new(),
            },
        }
    }

    // Fungsi untuk memilih validator berdasarkan stake dan reputasi
    fn select_validator(&self) -> Option<&Validator> {
        let mut rng = thread_rng();
        let validators: Vec<&Validator> = self.validators.values().collect();
        validators.choose_weighted(&mut rng, |validator| (validator.stake + validator.delegated_stake) as f64 * validator.reputation).ok()
    }

    // Fungsi untuk memvalidasi dan menambahkan blok ke dalam blockchain
    fn validate_block(&mut self, block: Block) {
        if let Some(validator) = self.validators.get_mut(&block.validator_id) {
            validator.stake += 10; // Reward
            validator.last_block_validated = self.current_period;
            validator.reputation += 0.1; // Increase reputation
            self.blocks.push(block.clone());
            self.pending_blocks.clear(); // Reset pending blocks on successful validation
            self.check_finality();
        }
    }

    // Fungsi untuk menambahkan blok yang menunggu validasi
    fn add_pending_block(&mut self, block: Block) {
        self.pending_blocks.insert(block);
    }

    // Fungsi untuk merotasi validator berdasarkan periode
    fn rotate_validators(&mut self) {
        self.current_period += 1;
        for validator in self.validators.values_mut() {
            if self.current_period - validator.last_block_validated >= validator.rotation_period {
                validator.stake -= 1; // Penalti kecil untuk yang tidak terpilih
                validator.reputation -= 0.1; // Decrease reputation
            }
        }
    }

    // Fungsi untuk mengecek finalitas blok
    fn check_finality(&mut self) {
        let mut counter = HashMap::new();
        for block in self.blocks.iter().rev().take(self.finality_threshold as usize) {
            *counter.entry(&block.validator_id).or_insert(0) += 1;
        }
        for (validator_id, count) in counter {
            if count > self.finality_threshold / 2 {
                println!("Finality reached for validator: {}", validator_id);
            }
        }
    }

    // Fungsi utama untuk menjalankan algoritma HybridStake
    fn run_hybrid_stake(&mut self) {
        // Update delegated stakes
        for holder in self.token_holders.values() {
            if let Some(delegated_to) = &holder.delegated_to {
                if let Some(validator) = self.validators.get_mut(delegated_to) {
                    validator.delegated_stake += holder.stake;
                }
            }
        }

        // Pilih validator berdasarkan stake dan reputasi
        if let Some(selected_validator) = self.select_validator() {
            let previous_hash = if self.blocks.is_empty() {
                String::new()
            } else {
                self.blocks.last().unwrap().hash.clone()
            };
            let block = Block::new(
                self.blocks.len() as u64,
                "Sample Block Data".to_string(),
                selected_validator.id.clone(),
                previous_hash,
            );
            self.validate_block(block);
        }

        // Simulasikan periode rotasi validator
        self.rotate_validators();
    }

    // Fungsi untuk menambah validator
    fn add_validator(&mut self, id: String, stake: u64, rotation_period: u64) {
        self.validators.insert(
            id.clone(),
            Validator {
                id,
                stake,
                delegated_stake: 0,
                rotation_period,
                last_block_validated: 0,
                reputation: 1.0,
            },
        );
    }

    // Fungsi untuk menambah token holder
    fn add_token_holder(&mut self, id: String, stake: u64, delegated_to: Option<String>) {
        self.token_holders.insert(
            id.clone(),
            TokenHolder {
                id,
                stake,
                delegated_to,
            },
        );
    }
}

fn main() {
    let mut blockchain = Blockchain::new(5);

    // Inisialisasi validator dan token holder
    blockchain.add_validator("Validator1".to_string(), 100, 10);
    blockchain.add_validator("Validator2".to_string(), 200, 10);
    blockchain.add_validator("Validator3".to_string(), 150, 10);

    blockchain.add_token_holder("Holder1".to_string(), 50, Some("Validator1".to_string()));
    blockchain.add_token_holder("Holder2".to_string(), 80, Some("Validator2".to_string()));
    blockchain.add_token_holder("Holder3".to_string(), 70, Some("Validator3".to_string()));

    // Jalankan algoritma HybridStake
    for _ in 0..20 {
        blockchain.run_hybrid_stake();
    }

    // Tampilkan blok yang sudah divalidasi
    for block in &blockchain.blocks {
        println!("Block ID: {}, Validator: {}, Hash: {}", block.id, block.validator_id, block.hash);
    }
}
