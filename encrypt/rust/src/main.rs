use chrono::prelude::*;

fn main() {
    let current_datetime = Local::now().timestamp_micros();
    println!("Current datetime: {}", current_datetime);
}
