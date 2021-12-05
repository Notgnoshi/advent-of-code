use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut previous_measurement: u32;
    if let Some(line) = lines.next() {
        previous_measurement = line
            .expect("Failed to read first line")
            .trim()
            .parse()
            .expect("Failed to parse int from first line");

        println!("{} (N/A - no previous measurement)", previous_measurement);
    } else {
        return;
    }

    let mut num_increased = 0;

    // Read until EOF
    while let Some(line) = lines.next() {
        let sonar_sweep: u32 = line
            .expect("Failed to read line")
            .trim()
            .parse()
            .expect("Failed to parse int");

        print!("{} ", sonar_sweep);
        if sonar_sweep == previous_measurement {
            println!("(no change)");
        } else if sonar_sweep < previous_measurement {
            println!("(decreased)");
        } else {
            println!("(increased)");
            num_increased += 1;
        }

        previous_measurement = sonar_sweep;
    }

    println!("number increased measurements: {}", num_increased);
}
