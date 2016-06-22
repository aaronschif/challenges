use std::io;
use std::io::prelude::*;
use std::cmp::*;
use std::fmt;
use std::collections::BinaryHeap;

struct Wave {
    start: u32,
    wait: u32,
    fun: u32,
    max_fun_do: Option<u32>,
    ord: u32,
}

impl Wave {
    fn new(start: u32, fun: u32, wait: u32) -> Wave {
        Wave {
            start: start,
            wait: wait,
            fun: fun,
            max_fun_do: None,
            ord: start,
        }
    }

    fn end(&self) -> u32 {
        self.start + self.wait
    }
}

impl fmt::Debug for Wave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n  Wave {{ start: {}, max_fun: {:?}, fun: {}, ord: {} }}", self.start, self.max_fun_do, self.fun, self.ord)
    }
}

impl PartialEq for Wave {
    fn eq(&self, other: &Self) -> bool { self.ord == other.ord }
}

impl PartialOrd for Wave {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.ord == self.ord {
            other.start.partial_cmp(&self.start)
        } else {
            other.ord.partial_cmp(&self.ord)
        }
    }
}

impl Eq for Wave {}

impl Ord for Wave {
    fn cmp(&self, other: &Self) -> Ordering {self.ord.cmp(&other.ord)}
}

fn readall() -> Vec<Wave> {
    let stdin = io::stdin();
    let mut result = Vec::new();

    for line in stdin.lock().lines().map(|x| String::from(x.unwrap())).skip(1) {
        let mut parts = line.split_whitespace().map(|e| e.trim().parse::<u32>().unwrap());
        let wave = Wave::new(
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
        );
        result.push(wave);
    }
    return result;
}

fn main() {
    let mut waves = {
        let mut r = BinaryHeap::new();
        r.extend(readall());
        r
    };

    let mut total_fun = 0;

    while !waves.is_empty() {
        let mut wave = waves.pop().unwrap();
        match wave.max_fun_do {
            None => {
                wave.max_fun_do = Some(total_fun);
                wave.ord = wave.end();
                waves.push(wave);
            }
            Some(fun) => {
                total_fun = max(total_fun, fun + wave.fun);
            }
        }
    }
    println!("{}", total_fun);
}
