pub fn run_simulation(){
    let mut location: f64 = 0.0; // meters 
    let mut velocity: f64 = 0.0; // meters per second 
    let mut acceleration: f64 = 0.0; // meters per second squared 

    let mut up_input_voltage: f64 = 0.0;
    let mut down_input_voltage: f64 = 0.0;

    let mut floor_count: u64 = 0;
    let mut floor_height: f64 = 0.0;
    let mut floor_request: Vec<u64> = Vec::new();

    while floor_request.len() > 0 {
        // Update location, velocity, and acceleration

        // If next floor request in queue is satisfied, then remove from queue

        // Adjust motor control to process next floor request

        // Print realtime statistics

    }

    println!("summary");
}

fn main() {
    run_simulation()
}
