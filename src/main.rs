use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
struct Monitor {
    name: Option<String>,
    script: Option<String>,
    result: Option<Result>,
    code: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Result {
    value: i32,
    processed_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Monitors {
    monitors: Vec<Monitor>,
}

// Update the monitor with a random value and current time
fn update_monitor(monitor: &mut Monitor) {
    let random_value = rand::random::<i32>();
    let current_time = Utc::now();
    let result = Result {
        value: random_value,
        processed_at: current_time,
    };
    monitor.result = Some(result);
    println!("Successfully updated monitor");
}

// Store the monitor data into a JSON file
fn store_monitor(monitors_data: &Monitors) {
    let current_time = Utc::now();
    let file_name = current_time.format("%Y-%m-%d_%H-%M-%S.json").to_string();

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let mut file_path = PathBuf::from(current_dir);
    file_path.push(file_name);

    let json_data = serde_json::to_string(&monitors_data).expect("Failed to serialize data to JSON");
    fs::write(&file_path, json_data).expect("Failed to write to file");

    println!("Successfully stored the monitor as {}", file_path.display());
}

// Update and store the monitor data within each minute
fn process_monitor(monitors_data: &mut Monitors) {
    // Run for 5 minutes
    for _ in 0..5 {
        // Update the monitors twice within each minute
        for _ in 0..2 {
            for monitor in &mut monitors_data.monitors {
                // Update each monitor
                update_monitor(monitor);
            }
            // Wait for 15 seconds for next updates
            thread::sleep(Duration::from_secs(15));
        }
        
        // Store the monitors once per minute
        store_monitor(monitors_data);

        // Wait for 30 seconds before the next minute starts
        thread::sleep(Duration::from_secs(30));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = &args[1];
    let file_path = &args[2];

    if flag != "-monitorFile" {
        panic!("Invalid flag! Please use '-monitorFile'");
    }

    // read the json data from file as String
    let contents = fs::read_to_string(file_path).expect("Unable to read the file");
    
    // then converting the string data to Vec<Monitor>;
    let mut monitors_data: Monitors = serde_json::from_str(&contents).expect("Failed to convert data");

    // Process the monitors
    process_monitor(&mut monitors_data);

    println!("Program terminated successfully.");
}
