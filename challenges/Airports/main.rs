use std::io;
use std::io::prelude::*;
use std::string::String;
use std::fmt;


struct Flight {
    start: u32,
    finish: u32,
    time: u32,
}

impl fmt::Debug for Flight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Flight {{start: {}, finish: {}, time: {} }}", self.start, self.finish, self.time)
    }
}

fn readall() -> (Vec<u32>, Vec<Vec<u32>>, Vec<Flight>) {
    let stdin = io::stdin();

    let mut value = String::new();
    stdin.lock().read_to_string(&mut value);

    let mut it = value.split_whitespace().map(|e| e.trim().parse::<u32>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut wait_times = Vec::new();
    for _ in 0..n {
        wait_times.push(it.next().unwrap());
    }

    let mut flight_times = Vec::new();
    for _ in 0..n {
        let mut flight = Vec::new();
        for _ in 0..n {
            flight.push(it.next().unwrap());
        }
        flight_times.push(flight);
    }
    let mut flights = Vec::new();
    for _ in 0..m {
        flights.push(
            Flight {
                start: it.next().unwrap(),
                finish: it.next().unwrap(),
                time: it.next().unwrap(),
            }
        );
    }

    (wait_times, flight_times, flights)
}

fn main() {
    let (wait_times, flight_times, flights) = readall();
    let number_airports = wait_times.len();

    let (mut max_flights, mut current_flights) = (0, 0);
    // println!("{:?}", wait_times);
    // println!("{:?}", flight_times);
    // println!("{:?}", flights);
}
