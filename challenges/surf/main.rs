use std::io;
use std::io::prelude::*;
use std::cmp::max;
use std::fmt;

struct Wave {
    start: u32,
    wait: u32,
    fun: u32,
    max_fun_do: u32,
    max_fun_dont: u32,
}

impl Wave {
    fn new(start: u32, fun: u32, wait: u32) -> Wave {
        Wave {
            start: start,
            wait: wait,
            fun: fun,
            max_fun_do: fun,
            max_fun_dont: fun,
        }
    }

    fn end(&self) -> u32 {
        self.start + self.wait
    }
}

impl fmt::Debug for Wave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wave {{ start: {}, max_fun: {}, otherfun: {} }}", self.start, self.max_fun_do, self.max_fun_dont)
    }
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
    let mut waves = readall();
    waves.sort_by_key(|e| e.end());

    let mut total_max_fun = 0;

    for n in 0..waves.len() {
        for i in (0..n).rev() {
            if waves[i].end() <= waves[n].start {
                waves[n].max_fun_do = max(waves[n].max_fun_do, waves[i].max_fun_dont + waves[n].fun);
                break;
            }
        }

        total_max_fun = max(total_max_fun, waves[n].max_fun_dont);
        total_max_fun = max(total_max_fun, waves[n].max_fun_do);
        waves[n].max_fun_dont = total_max_fun;
        // println!("{:?}", waves);
    }
    println!("{}", total_max_fun);
    // println!("{}", waves.last().map_or(0, |x| x.max_fun_dont));
}
