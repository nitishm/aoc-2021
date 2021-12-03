use std::{str::SplitWhitespace};

#[derive(Debug,Default)]
struct Location {
     horizontal: i32,
     depth: i32,
     aim: i32
}

impl Location {
     fn answer(&self) -> i32 {
          self.horizontal * self.depth
     }
}

fn main() {

     let mut loc = Location::default();
     
    // Read file
    let input= std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<SplitWhitespace> = input.lines().map(|x| x.split_whitespace()).collect();

    for mut line in lines {
     let v: (&str, i32) = (line.next().unwrap(), line.next().unwrap().parse().unwrap());
     match v.0 {
          "forward" => {
               loc.horizontal = loc.horizontal + v.1;
               loc.depth = loc.depth + (loc.aim * v.1);
          },
          "up" => {
               loc.aim = loc.aim - v.1;
          },
          "down" => {
               loc.aim = loc.aim + v.1;
          }
          _ => {},
     }
    }

    println!("Answer: {}", loc.answer());
}
