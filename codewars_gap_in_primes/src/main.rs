use codewars_gap_in_primes;
use codewars_gap_in_primes::sieve;

fn main() {
    assert_eq!(sieve(2), vec![2]);
    assert_eq!(sieve(3), vec![2,3]);
    assert_eq!(sieve(10), vec![2,3,5,7]);
    assert_eq!(sieve(12), vec![2,3,5,7,11]);
    assert_eq!(sieve(13), vec![2,3,5,7,11,13]);
}