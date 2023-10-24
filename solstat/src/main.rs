use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig; // Import CommitmentConfig
use solana_sdk::slot_history::Slot;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Load environment variables from .env
    dotenv().ok();

    // Retrieve the Solana endpoint URL from the environment variables
    let solana_url = env::var("SOLANA_ENDPOINT")
        .expect("SOLANA_ENDPOINT is not set in the .env file");

    // Create an RpcClient to interact with the Solana network
    let rpc_client = RpcClient::new(solana_url.to_string());

    // Calculate the slot for the current time minus 72 hours
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;

    // Use CommitmentConfig::finalized() to specify the commitment
    let start_slot = rpc_client.get_slot_with_commitment(CommitmentConfig::finalized()).unwrap();
    // Explicitly cast the result to u64 before assigning to end_slot
    let end_slot = start_slot.saturating_sub((72 * 60 * 60 / 4) as u64);

    // Fetch Solana data within the specified time window
    let mut data_blocks = Vec::new();
    for slot in (end_slot..start_slot).rev() {
        // Use CommitmentConfig::finalized() to specify the commitment
        let slot_data = rpc_client.get_slot_with_commitment(CommitmentConfig::finalized()).unwrap();
        data_blocks.push((slot, slot_data));
    }

    // Partition the data into 3-hour blocks (each block contains data for 3 slots)
    let blocks: Vec<Vec<(Slot, Slot)>> = data_blocks
        .chunks(3)
        .map(|chunk| chunk.iter().cloned().collect())
        .collect();

    for (index, block) in blocks.iter().enumerate() {
        println!("3-Hour Data Block #{}:", index + 1);
        for (slot, slot_data) in block {
            println!("Slot: {}, Slot Data: {:?}", slot, slot_data);
        }
        println!();
    }
}