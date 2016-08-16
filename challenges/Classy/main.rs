use std::io;
use std::io::prelude::*;
use std::cmp::*;
use std::fmt;
use std::string::String;
use std::collections::BinaryHeap;
use std::f64;

#[derive(Debug,PartialEq,PartialOrd)]
struct Person {
    class: f64,
    name: Box<String>,
}

fn readall() -> Vec<Person> {
    let stdin = io::stdin();
    let mut result = Vec::new();

    for line in stdin.lock().lines().skip(1).map(|x| x.unwrap()) {
        let mut iter = line.split_whitespace();
        let mut name = iter.next().unwrap();
        let mut person = Person {
            name: Box::new(String::from(name)), class: 0.0,
        };
        person.name.pop();
        for class in iter {
            match class {
                "lower" => person.class -= 1.0,
                "upper" => person.class += 1.0,
                _ => {},
            }
            person.class /= 10.0;
        }
        person.class += 1.0;
        person.class *= -1.0;
        result.push(person);
    }

    result
}

fn main() {
    let mut values = readall();
    values.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    for value in values.iter() {
        println!("{}", value.name);
    }
}
