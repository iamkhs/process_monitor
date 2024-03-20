use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
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

// this update_monitor method is updating monitor with the result
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

// this store_monitor method is storing the monitor file on current directory as json file
// with the current_time as filename
fn store_monitor(monitors_data : &Monitors) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let current_time = Utc::now();
    let filename = format!("{}/{}_monitors.json", current_dir.display(), current_time);
    let json_data = serde_json::to_string(&monitors_data).expect("Failed to serialize data to JSON");
    fs::write(filename, json_data).expect("Failed to write to file");
    println!("Successfully stored the monitor");
}

// this is process_monitor method where it is updating and storing monitors
fn process_monitor(monitors_data: &mut Monitors) {
    for monitor in &mut monitors_data.monitors {
        // updating each monitor with result
        update_monitor(monitor);
    }
    // after update all the monitors i am storing them
    store_monitor(monitors_data);

    // wait for 30sec before the next update;
    thread::sleep(Duration::from_secs(30));
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

    // Looping 10 times to process monitors:
    // In each iteration, the `process_monitor` function updates the monitors and stores the updated data.
    // After each update, the loop pauses for 30 seconds.
    // Therefore, the total duration of the loop is 10 * 30 seconds = 5 minutes.
    for _ in 0..10 {
        process_monitor(&mut monitors_data);
    }
    println!("Program terminated successfully.");
}
