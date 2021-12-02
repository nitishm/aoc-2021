use std::ops::Index;

fn main() {
    // Read file
    let values: Vec<i32> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut counter = 0;
    let mut last_window: i32 = -1;

    for (i, _) in values.iter().enumerate() {
        if i > 1 {
            let window = values.index(i - 2) + values.index(i - 1) + values.index(i);
            if last_window > 0 {
                if window > last_window {
                    counter = counter + 1;
                }
            }
            last_window = window;
        }
    }

    println!("Number of times depth increased {}", counter);
}
