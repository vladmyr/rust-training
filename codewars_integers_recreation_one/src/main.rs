/**
 * https://www.codewars.com/kata/sort-numbers/train/rust
 * 
 * Divisors of 42 are : 1, 2, 3, 6, 7, 14, 21, 42. These divisors squared are: 
 * * 1, 4, 9, 36, 49, 196, 441, 1764. The sum of the squared divisors is 2500 
 * which is 50 * 50, a square!

 * Given two integers m, n (1 <= m <= n) we want to find all integers between m 
 * and n whose sum of squared divisors is itself a square. 42 is such a number.

 * The result will be an array of arrays or of tuples (in C an array of Pair) or
 *  a string, each subarray having two elements, first the number whose squared 
 * divisors is a square and then the sum of the squared divisors.
 */

fn divisors(m: &u64) -> Vec<u64> {
    (1..=(*m as f64).sqrt() as u64)
        .filter(|x| m % x == 0)
        .fold(Vec::new(), |mut acc, x| {
            acc.push(x);

            if m / x != x {
                acc.push(m / x);
            }

            acc
        })
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..n)
        .map(|a| (
            a, 
            divisors(&a)
                .iter()
                .map(|b| b * b)
                .fold(0, |acc, c| acc + c)
        ))
        .filter(|(_, a)| (*a as f64).sqrt() % 1_f64 == 0_f64)
        .collect()
}

fn main() {
    let mut divs = divisors(&1);
    divs.sort();
    assert_eq!(vec![1], divs);

    let mut divs = divisors(&3);
    divs.sort();
    assert_eq!(vec![1, 3], divs);

    let mut divs = divisors(&42);
    divs.sort();
    assert_eq!(vec![1, 2, 3, 6, 7, 14, 21, 42], divs);

    let mut divs = divisors(&100);
    divs.sort();
    assert_eq!(vec![1, 2, 4, 5, 10, 20, 25, 50, 100], divs);

    assert_eq!(vec![(1, 1), (42, 2500), (246, 84100)], list_squared(1, 250));
    assert_eq!(vec![(42, 2500), (246, 84100)], list_squared(42, 250));
    assert_eq!(Vec::new() as Vec<(u64, u64)>, list_squared(300, 600));
}
