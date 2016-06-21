use std::io;
use std::io::prelude::*;
use std::cmp::max;
use std::fmt;

struct Wave {
    start: u32,
    fun: u32,
    wait: u32,
}

impl Wave {
    fn end(&self) -> u32 {
        self.start + self.wait
    }

    fn overlap(&self, wave: &Wave) -> bool {
        (self.start < wave.start && wave.start < self.end()) || (wave.start < self.start && self.start < self.end())
    }
}

struct WaveData {
    max_fun: u32,
    next_wave: Option<u32>,
}

impl WaveData {
    fn new(max_fun: u32) -> WaveData {
        WaveData {
            max_fun: max_fun,
            next_wave: None,
        }
    }
}

impl fmt::Debug for WaveData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(n) = self.next_wave {
            write!(f, "WaveData {{ max_fun: {}, next_wave: {} }}", self.max_fun, n)
        }
        else {
            write!(f, "WaveData {{ max_fun: {}, next_wave: {} }}", self.max_fun, "None")
        }
    }
}

fn readall() -> Vec<Wave> {
    let stdin = io::stdin();
    let mut result = Vec::new();

    for line in stdin.lock().lines().map(|x| String::from(x.unwrap())).skip(1) {
        let mut parts = line.split_whitespace().map(|e| e.trim().parse::<u32>().unwrap());
        let wave = Wave {
            start: parts.next().unwrap(),
            fun: parts.next().unwrap(),
            wait: parts.next().unwrap(),
        };
        result.push(wave);
    }
    return result;
}

fn main() {
    let mut waves = readall();
    waves.sort_by_key(|e| e.start);
    let mut waves_values: Vec<WaveData> = Vec::new();
    let mut total_max_fun = 0;

    for (n, wave) in waves.iter().enumerate() {
        let mut max_fun = wave.fun;
        let mut max_fun_wave = None;
        for i in (0..n).rev() {
            if waves[i].overlap(wave) {
                continue;
            }
            else {
                let new_fun = waves_values[i].max_fun + wave.fun;
                if new_fun > max_fun {
                    max_fun = new_fun;
                    max_fun_wave = Some(i as u32);
                }
            }
        }
        total_max_fun = max(total_max_fun, max_fun);
        let mut wave_data = WaveData {
            max_fun: max_fun,
            next_wave: max_fun_wave,
        };
        waves_values.push(wave_data);
    }
    println!("{}", total_max_fun);
}
