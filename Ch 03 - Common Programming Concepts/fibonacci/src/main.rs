use clap::Parser;
use ibig::{ubig, UBig};
use std::time::Instant;

fn fibonacci_iterative_overflow_unsafe(n: u64) -> u64 {
    let mut curr = 1;
    let mut prev = 1;

    for _ in 2..n {
        let tmp = curr;
        curr += prev;
        prev = tmp;
    }

    curr
}

fn fibonacci_iterative_overflow_safe(n: u64) -> Option<u64> {
    let mut curr: u64 = 1;
    let mut prev = 1;

    for _ in 2..n {
        let tmp = curr;
        curr = curr.checked_add(prev)?;
        prev = tmp;
    }

    Some(curr)
}

fn fibonacci_iterative_big(n: u64) -> UBig {
    let mut curr = ubig!(1);
    let mut prev = ubig!(1);

    for _ in 2..n {
        let tmp = curr.clone();
        curr += prev;
        prev = tmp;
    }

    curr
}

fn fibonacci_recursive_overflow_unsafe(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    fibonacci_recursive_overflow_unsafe(n - 1) + fibonacci_recursive_overflow_unsafe(n - 2)
}

fn fibonacci_recursive_overflow_safe(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }

    if n == 1 || n == 2 {
        return Some(1);
    }

    let left = fibonacci_recursive_overflow_safe(n - 1)?;
    let right = fibonacci_recursive_overflow_safe(n - 2)?;

    Some(left + right)
}

fn fibonacci_recursive_big(n: UBig) -> UBig {
    if n == ubig!(0) {
        return ubig!(0);
    }

    if n == ubig!(1) || n == ubig!(2) {
        return ubig!(1);
    }

    fibonacci_recursive_big(n.clone() - 1) + fibonacci_recursive_big(n - 2)
}

#[derive(Debug, clap::Parser)]
#[command(name = "fibonacci")]
struct Fibonacci {
    #[arg(required = true)]
    n: u64,

    #[arg(long)]
    no_recursive: bool,
}

fn main() {
    let args = Fibonacci::parse();

    let n = args.n;
    let no_recursive = args.no_recursive;

    // -- Iterative Overflow Unsafe.
    let now = Instant::now();
    let overflow_unsafe_iterative_result = fibonacci_iterative_overflow_unsafe(n);
    let overflow_unsafe_iterative_elapsed = now.elapsed();

    println!(
        "overflow unsafe iterative: {} in {:.2?}",
        overflow_unsafe_iterative_result, overflow_unsafe_iterative_elapsed
    );

    // -- Iterative Overflow Safe.
    let now = Instant::now();
    let overflow_safe_iterative_result = match fibonacci_iterative_overflow_safe(n) {
        Some(x) => x.to_string(),
        None => "overflow".into(),
    };
    let overflow_safe_iterative_elapsed = now.elapsed();

    println!(
        "overflow safe   iterative: {} in {:.2?}",
        overflow_safe_iterative_result, overflow_safe_iterative_elapsed
    );

    // -- Iterative BigNum.
    let now = Instant::now();
    let big_iterative_result = fibonacci_iterative_big(n);
    let big_iterative_elapsed = now.elapsed();

    println!(
        "big iterative:             {} in {:.2?}",
        big_iterative_result, big_iterative_elapsed
    );

    if !no_recursive {
        // -- Resursive Overflow Unsafe.
        let now = Instant::now();
        let overflow_unsafe_recursive_result = fibonacci_recursive_overflow_unsafe(n);
        let overflow_unsafe_recursive_elapsed = now.elapsed();

        println!(
            "overflow unsafe recursive: {} in {:.2?}",
            overflow_unsafe_recursive_result, overflow_unsafe_recursive_elapsed
        );

        // -- Resursive Overflow Safe.
        let now = Instant::now();
        let overflow_safe_recursive_result = match fibonacci_recursive_overflow_safe(n) {
            Some(x) => x.to_string(),
            None => "overflow".into(),
        };
        let overflow_safe_recursive_elapsed = now.elapsed();

        println!(
            "overflow safe   recursive: {} in {:.2?}",
            overflow_safe_recursive_result, overflow_safe_recursive_elapsed
        );

        // -- Resursive BigNum.
        let now = Instant::now();
        let big_recursive_result = fibonacci_recursive_big(UBig::from(n));
        let big_recursive_elapsed = now.elapsed();

        println!(
            "big recursive:             {} in {:.2?}",
            big_recursive_result, big_recursive_elapsed
        );
    }
}
