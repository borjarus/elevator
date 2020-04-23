use std::{env, time};
use std::io::{self, Read};
use std::fs::File;
use std::time::Instant;
use std::thread;


pub fn run_simulation(){
    let mut location: f64 = 0.0; // meters 
    let mut velocity: f64 = 0.0; // meters per second 
    let mut acceleration: f64 = 0.0; // meters per second squared 

    let mut up_input_voltage: f64 = 0.0;
    let mut down_input_voltage: f64 = 0.0;

    let mut floor_count: u64 = 0;
    let mut floor_height: f64 = 0.0;
    let mut floor_request: Vec<u64> = Vec::new();

    let buffer = match env::args().nth(1) {
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        },
        None => {
            let fp = "test1.txt";
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        },
        Some(fp) => {
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        }
    };

    for (li,l) in buffer.lines().enumerate() {
        if li == 0 {
            floor_count = l.parse().unwrap();
        } else if li == 1 {
            floor_height = l.parse().unwrap();
        } else {
            floor_request.push(l.parse().unwrap());
        }
    }

    
    
    
    while floor_request.len() > 0 {
        // Update location, velocity, and acceleration
        let mut prev_loop_time = Instant::now();
    
        let now = Instant::now();
        let dt = now.duration_since(prev_loop_time)
                        .as_secs_f64();
        prev_loop_time = now;
    
        thread::sleep(time::Duration::from_millis(10)); 

        location = location + velocity * dt;
        velocity = velocity + acceleration * dt;
        acceleration = {
            let F = (up_input_voltage - down_input_voltage) * 8.0;
            let m = 1200000.0;
            -9.8 + F/m
        };
        

        // If next floor request in queue is satisfied, then remove from queue
        let next_floor = floor_request[0];
        if (location - (next_floor as f64) * floor_height).abs() < 0.01
            && velocity.abs() < 0.01 {
                velocity = 0.0;
                floor_request.remove(0);
            }

        // Adjust motor control to process next floor request

        // Print realtime statistics

    }

    println!("summary");
}

fn main() {
    run_simulation()
}
