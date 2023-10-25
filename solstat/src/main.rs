use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::time::{SystemTime, UNIX_EPOCH};
use csv::Writer;
use std::fs::File;

fn detect_patterns(data_blocks: &Vec<(u64, u64)>) -> Vec<String> {
    let mut detected_patterns = Vec::new();
    
    for (slot, slot_data) in data_blocks {
        // Detect a pattern when the transaction count exceeds a certain threshold
        if *slot_data > 1000 {
            detected_patterns.push(format!("Pattern detected: High transaction count at slot {}", slot));
        }
        
        // Detect a pattern when there's a sudden increase in transaction count
        if data_blocks.len() >= 2 {
            let previous_slot_data = data_blocks[data_blocks.len() - 2].1;
            if *slot_data > 2 * previous_slot_data {
                detected_patterns.push(format!("Pattern detected: Sudden increase in transaction count at slot {}", slot));
            }
        }
        
        // Detect a pattern when the transaction count is consistently decreasing
        if data_blocks.len() >= 3 {
            let previous_slot_data = data_blocks[data_blocks.len() - 3].1;
            if *slot_data < previous_slot_data {
                detected_patterns.push(format!("Pattern detected: Consistent decrease in transaction count at slot {}", slot));
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

    // Specify the desired 5-minute interval in seconds
    let interval_seconds = 5 * 60;

    // Start collecting data within the 5-minute window
    let start_time = SystemTime::now();
    let mut data_blocks = Vec::new();

    loop {
        // Calculate the slot for the current time minus 5 minutes
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;

        // Use CommitmentConfig::finalized() to specify the commitment
        let start_slot = rpc_client.get_slot_with_commitment(CommitmentConfig::finalized()).unwrap();
        // Explicitly cast the result to u64 before assigning to end_slot
        let end_slot = start_slot.saturating_sub((5 * 60 / 4) as u64); // 5 minutes * 60 seconds / 4 seconds per slot

        // Fetch Solana data within the specified time window
        for slot in (end_slot..start_slot).rev() {
            // Use CommitmentConfig::finalized() to specify the commitment
            let slot_data = rpc_client.get_slot_with_commitment(CommitmentConfig::finalized()).unwrap();
            data_blocks.push((slot, slot_data));
        }

        // Check if the 5-minute interval has passed
        let elapsed_time = start_time.elapsed().unwrap().as_secs();
        if elapsed_time >= interval_seconds {
            break; // Exit the loop after 5 minutes
        }
    }

    // Detect patterns in the collected data
    let detected_patterns = detect_patterns(&data_blocks);
    
    // Write the detected patterns to a log file
    let mut log_writer = Writer::from_path("patterns.log").unwrap();
    for pattern in detected_patterns {
        log_writer.write_record(&[pattern]).unwrap();
    }
    
    // Write the transformed data to a CSV file
    let mut csv_writer = Writer::from_path("solana_data.csv").unwrap();
    csv_writer.write_record(&["Slot", "Data"]).unwrap();
    for (slot, slot_data) in &data_blocks {
        csv_writer.write_record(&[slot.to_string(), slot_data.to_string()]).unwrap();
    }
}

