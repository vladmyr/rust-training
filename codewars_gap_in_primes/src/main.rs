use codewars_gap_in_primes;

fn main() {
    // assert_eq!(codewars_gap_in_primes::sieve(2), vec![2]);
    // assert_eq!(codewars_gap_in_primes::sieve(3), vec![2,3]);
    // assert_eq!(codewars_gap_in_primes::sieve(10), vec![2,3,5,7]);
    // assert_eq!(codewars_gap_in_primes::sieve(12), vec![2,3,5,7,11]);
    // assert_eq!(codewars_gap_in_primes::sieve(13), vec![2,3,5,7,11,13]);

    assert_eq!(codewars_gap_in_primes::gap(2, 100, 110), Some((101, 103)));
    assert_eq!(codewars_gap_in_primes::gap(4, 100, 110), Some((103, 107)));
    assert_eq!(codewars_gap_in_primes::gap(6, 100, 110), None);
    assert_eq!(codewars_gap_in_primes::gap(8, 300, 400), Some((359, 367)));
}