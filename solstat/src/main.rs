
use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::time::{SystemTime, UNIX_EPOCH};

fn detect_patterns(data_blocks: &Vec<(u64, u64)>) -> Vec<String> {
    let mut detected_patterns = Vec::new();

    let mut average_block_time = 0.0;
    let mut block_time_sum = 0.0;
    let mut block_count = 0;

    for (slot, slot_data) in data_blocks {
        // Update the average block time
        block_count += 1;
        block_time_sum += *slot_data as f64; // Use * to dereference the u64 slot_data
        average_block_time = block_time_sum / block_count as f64;

        // Detect a pattern when the average block time changes significantly
        if block_count >= 10 {
            let previous_average_block_time = block_time_sum / (block_count - 1) as f64;
            if (average_block_time - previous_average_block_time).abs() > 0.5 {
                detected_patterns.push(format!("Pattern detected: Significant change in average block time at slot {}", slot));
            }
        }
    }

    detected_patterns
}

fn main() {
    // Load environment variables from .env
    dotenv().ok();

    // Retrieve the Solana endpoint URL from the environment variables
    let solana_url = env::var("SOLANA_ENDPOINT")
        .expect("SOLANA_ENDPOINT is not set in the .env file");

    // Create an RpcClient to interact with the Solana network
    let rpc_client = RpcClient::new(solana_url.to_string());

    // Calculate the slot for the current time minus 5 minutes
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;

    // Use CommitmentConfig::finalized() to specify the commitment
    let start_slot = rpc_client.get_slot_with_commitment(CommitmentConfig::finalized()).unwrap();
    // Explicitly cast the result to u64 before assigning to end_slot
    let end_slot = start_slot.saturating_sub((5 * 60 / 4) as u64); // 5 minutes * 60 seconds / 4 seconds per slot

    // Fetch Solana data within the 5-minute window
    let mut data_blocks = Vec::new();
    for slot in (end_slot..start_slot).rev() {
        // Use CommitmentConfig::finalized() to specify the commitment
        let slot_data = rpc_client.get_slot_with_commitment(CommitmentConfig::finalized()).unwrap();
        data_blocks.push((slot, slot_data));
    }

    // Detect patterns in the data
    let detected_patterns = detect_patterns(&data_blocks);

    // Print detected patterns
    for pattern in &detected_patterns {
        println!("{}", pattern);
    }
}
