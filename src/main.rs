use num_format::{Locale, ToFormattedString};
use std::io;

fn main() {
    println!("Fibonnaci number generator");
    // source of Fibonnaci formula: https://math.hmc.edu/funfacts/fibonacci-number-formula/
    loop {
        println!("\nExample input: 32");
        println!("Example output: 2,178,309");
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: f64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result =
            (((1. + 5f64.sqrt()) / 2.).powf(n) - ((1. - 5f64.sqrt()) / 2.).powf(n)) / 5f64.sqrt();

        println!(
            "Fib num #{}: {}",
            n,
            (result as u32).to_formatted_string(&Locale::en)
        );
        break;
    }
}
