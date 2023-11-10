use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::ops::Range;

fn median(nums: &Vec<i64>) -> Option<f64> {
    // -- Median is not defined for length less than 2.
    if nums.len() < 2 {
        return None;
    }

    // -- Sort the numbers.
    let mut nums = nums.clone();
    nums.sort();
    let nums = nums;

    // -- Handle even length.
    let len = nums.len();
    let a = *nums.get(len / 2)? as f64;

    if len % 2 != 0 {
        return Some(a);
    }

    // -- Handle odd length.
    let b = *nums.get((len / 2) - 1)? as f64;

    Some((a + b) / 2.0)
}

fn mode(nums: &Vec<i64>) -> Option<Vec<i64>> {
    // -- Mode is not defined for length less than 2.
    if nums.len() < 2 {
        return None;
    }

    // -- Count occurences.
    let mut occurences = HashMap::new();

    for n in nums {
        let count = occurences.entry(*n).or_insert(0);
        *count += 1;
    }

    // -- Find the highest number of occurrences.
    let (_, max_times) = occurences.iter().max_by(|(_, a), (_, b)| a.cmp(b))?;

    // -- Return the numbers with the highest number of occurences.
    let mut modes: Vec<_> = occurences
        .iter()
        .filter(|(_, times)| *times == max_times)
        .map(|(n, _)| *n)
        .collect();
    modes.sort();
    Some(modes)
}

fn get_ordered(n: i64) -> Vec<i64> {
    (1..=n).collect()
}

fn get_shuffled(n: i64) -> Vec<i64> {
    let mut nums = get_ordered(n);
    nums.shuffle(&mut rand::thread_rng());
    nums
}

fn get_rand(n: usize, range: &Range<i64>) -> Vec<i64> {
    (1..=n)
        .map(|_| rand::thread_rng().gen_range(range.clone()))
        .collect()
}

fn main() {
    let range = Range { start: 1, end: 6 };
    let test_vecs = [
        ("empty", vec![]),
        ("one number", vec![1]),
        ("two same", vec![1, 1]),
        ("two different", vec![1, 2]),
        ("even ordered", get_ordered(10)),
        ("even shuffled", get_shuffled(10)),
        ("even random", get_rand(10, &range)),
        ("odd  ordered", get_ordered(9)),
        ("odd  shuffled", get_shuffled(9)),
        ("odd  random", get_rand(9, &range)),
    ];

    for (name, nums) in test_vecs {
        println!("---");
        println!("{name}");

        print!("nums   = [");
        for n in &nums {
            print!("{n},");
        }
        println!("]");

        println!(
            "median = {}",
            match median(&nums) {
                Some(n) => n.to_string(),
                None => String::from("DNE"),
            }
        );

        print!("mode   = ");
        match mode(&nums) {
            Some(ms) => {
                print!("[");
                for m in ms {
                    print!("{m},");
                }
                println!("]");
            }
            None => println!("DNE"),
        }
    }
}
