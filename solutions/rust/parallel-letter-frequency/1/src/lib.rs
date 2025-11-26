use std::collections::HashMap;
use std::thread;

/// Based on iteration 6 of rsalmei's solution
/// https://exercism.org/tracks/rust/exercises/parallel-letter-frequency/solutions/rsalmei
///
/// I made several variations to this solution, but the original is the fastest I've found.
/// Congrats rsalmei!
///
/// test bench_large_parallel   ... bench:      96,436 ns/iter (+/- 1,671)
/// test bench_large_sequential ... bench:     197,775 ns/iter (+/- 2,973)
/// test bench_small_parallel   ... bench:       6,329 ns/iter (+/- 123)
/// test bench_small_sequential ... bench:       7,137 ns/iter (+/- 83)
/// test bench_tiny_parallel    ... bench:          34 ns/iter (+/- 0)
/// test bench_tiny_sequential  ... bench:          34 ns/iter (+/- 0)
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter = |input: &[&str]| {
        let mut map = HashMap::new();

        for line in input {
            for c in line
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
            {
                *map.entry(c).or_default() += 1;
            }
        }

        map
    };

    // redirect to the best implementation.

    match input.len() {
        0 => HashMap::new(),

        n if n < 500 => counter(input),

        _ => thread::scope(|s| {
            let mut handles = Vec::with_capacity(worker_count);

            for lines in input.chunks(input.len() / worker_count + 1) {
                handles.push(s.spawn(|| counter(lines)))
            }

            let mut map = handles.pop().unwrap().join().unwrap();

            for res in handles {
                res.join().unwrap().into_iter().for_each(|(k, v)| {
                    *map.entry(k).or_default() += v;
                })
            }

            map
        }),
    }
}

// #[test]
// fn right_number_of_workers() {
//     let input = &[];
//     let worker_count = 1;
//     let expected_worker_count = 1;
//     assert_eq!(
//         number_of_workers(input, worker_count),
//         expected_worker_count
//     );
//
//     let input = &[];
//     let worker_count = 3;
//     let expected_worker_count = 1;
//     assert_eq!(
//         number_of_workers(input, worker_count),
//         expected_worker_count
//     );
//
//     let input = &[];
//     let worker_count = 10;
//     let expected_worker_count = 1;
//     assert_eq!(
//         number_of_workers(input, worker_count),
//         expected_worker_count
//     );
//
//     let input = &["a"];
//     let worker_count = 1;
//     let expected_worker_count = 1;
//     assert_eq!(
//         number_of_workers(input, worker_count),
//         expected_worker_count
//     );
//
//     let input = &["a"];
//     let worker_count = 3;
//     let expected_worker_count = 1;
//     assert_eq!(
//         number_of_workers(input, worker_count),
//         expected_worker_count
//     );
//
//     let input = &["a"];
//     let worker_count = 10;
//     let expected_worker_count = 1;
//     assert_eq!(
//         number_of_workers(input, worker_count),
//         expected_worker_count
//     );
//
//     let input = &[
//         "aAbCfkt",
//         "RoPl",
//         "zXxXaQmNngcDa",
//         "AbCf",
//         "ktRoPlzXx",
//         "XaQmNn",
//         "gcDaA",
//         "bCfktR",
//         "oPl",
//         "zXxXaQm",
//         "NngcD",
//     ];
//     let worker_count = 1;
//     let expected_worker_count = 1;
//     assert_eq!(
//         number_of_workers(input, worker_count),
//         expected_worker_count
//     );
//
//     let input = &[
//         "aAbCfkt",
//         "RoPl",
//         "zXxXaQmNngcDa",
//         "AbCf",
//         "ktRoPlzXx",
//         "XaQmNn",
//         "gcDaA",
//         "bCfktR",
//         "oPl",
//         "zXxXaQm",
//         "NngcD",
//     ];
//     let worker_count = 3;
//     let expected_worker_count = 3;
//     assert_eq!(
//         number_of_workers(input, worker_count),
//         expected_worker_count
//     );
//
//     let input = &[
//         "aAbCfkt",
//         "RoPl",
//         "zXxXaQmNngcDa",
//         "AbCf",
//         "ktRoPlzXx",
//         "XaQmNn",
//         "gcDaA",
//         "bCfktR",
//         "oPl",
//         "zXxXaQm",
//         "NngcD",
//     ];
//     let worker_count = 10;
//     let expected_worker_count = 10;
//     assert_eq!(
//         number_of_workers(input, worker_count),
//         expected_worker_count
//     );
// }
//
// fn number_of_workers(input: &[&str], worker: usize) -> usize {
//     match input.len() as f32 / worker as f32 {
//         x if x <= 1.0 => 1,
//         _ => worker,
//     }
// }
