use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use itertools::sorted;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    // Leave the bitstring as a string.
    let diagnostic_report = lines.map(|line| line.unwrap().trim().to_string());

    let mut num_values: usize = 0;
    let mut bit_places_set = HashMap::<usize, usize>::new();

    for bitstring in diagnostic_report {
        num_values += 1;
        // Loop over the "bits" backward (from the LSB to the MSB).
        // Don't @ me bro.
        for (idx, bit) in bitstring.chars().rev().enumerate() {
            let num_set = bit_places_set.entry(idx).or_insert(0);
            if bit == '1' {
                *num_set += 1;
            }
        }
    }

    let mut gamma_rate: usize = 0;
    let mut epsilon_rate: usize = 0;
    for idx in sorted(bit_places_set.keys()) {
        let number_bits_set = bit_places_set.get(idx).unwrap();

        let most_common_bit = {
            if number_bits_set > &(num_values / 2) {
                1
            } else {
                0
            }
        };

        gamma_rate |= most_common_bit << idx;
        epsilon_rate |= (!most_common_bit & 0b1) << idx;
    }

    let power_consumption = gamma_rate * epsilon_rate;

    println!("gamma rate: {}", gamma_rate);
    println!("epsilon rate: {}", epsilon_rate);
    println!("power consumption: {}", power_consumption);
}
