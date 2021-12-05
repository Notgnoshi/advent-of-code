use std::io;
use std::io::BufRead;

use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    // Convert the iterable of lines to an iterable of ints
    let raw_sweeps = lines.map(|line| {
        line.unwrap()
            .trim()
            .parse::<u32>()
            .expect("Failed to parse int")
    });
    // Convert the iterable of ints to an iterable of triple-windows.
    let sweep_windows = raw_sweeps.tuple_windows::<(_, _, _)>();
    // Sum the windows -- a really simple kind of filter.
    let mut filtered_sweeps = sweep_windows.map(|window| window.0 + window.1 + window.2);

    let mut num_increased: u32 = 0;
    if let Some(mut previous_sum) = filtered_sweeps.next() {
        println!("{} (N/A - no previous sum)", previous_sum);
        while let Some(sum) = filtered_sweeps.next() {
            print!("{} ", sum);
            if sum == previous_sum {
                println!("(no change)");
            } else if sum > previous_sum {
                println!("(increased)");
                num_increased += 1;
            } else {
                println!("(decreased)");
            }

            previous_sum = sum;
        }
    }

    println!("num increased sums: {}", num_increased);
}
