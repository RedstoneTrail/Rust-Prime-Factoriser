use num_bigint::{BigInt, ToBigInt};
use std::io;
use std::io::Write;

fn main() {
    // print!("\n\nPlease enter a number to prime factorise.\n\n> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("end of playtest.");

    let number_to_factorise: BigInt = input.trim().parse().expect("invalid input");

    if number_to_factorise == BigInt::from(1) {
        println!("");
    } else {
        let result = prime_factorise(&number_to_factorise);

        println!(
            "{}",
            result
                .into_iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
    };
}

fn prime_factorise(number: &BigInt) -> Vec<BigInt> {
    let mut number = number.clone();
    let mut result = Vec::new();
    if check_prime(&number) {
        vec![number]
    } else {
        while !check_prime(&number) {
            let mut count = BigInt::from(2);
            while &number % &count != BigInt::from(0) {
                count += 1;
            }
            number /= &count;
            result.push(count);
        }
        result.push(number);
        result
    }
}

fn check_prime(number: &BigInt) -> bool {
    if number == &BigInt::from(1) {
        return false;
    }
    let mut loop_count: BigInt = BigInt::from(1u32);
    while loop_count < number.sqrt().to_bigint().unwrap() {
        loop_count = &loop_count + BigInt::from(1u32);
        if number % &loop_count == BigInt::from(0u32) {
            return false;
        }
    }
    true
}
