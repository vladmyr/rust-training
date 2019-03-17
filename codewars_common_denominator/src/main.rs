/**
 * https://www.codewars.com/kata/common-denominators/train/rust
 * 
 * You will have a list of rationals in the form
 * [ (numer_1, denom_1) , ... ,(numer_n, denom_n) ] 
 * where all numbers are positive ints.
 * You have to produce a result in the form
 * [ (N_1, D) , ... , (N_n, D) ] 
 * in which D is as small as possible and
 * N_1/D == numer_1/denom_1 ... N_n/D == numer_n,/denom_n.
 */

use codewars_common_denominator::convert_fracts;

// clever solution
fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b)} }
fn lcm(a: i64, b: i64) -> i64 { a / gcd(a, b) * b }
fn convert_fracts_clever(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let d = l
      .iter()
      .fold(1, |acc, &(num, den)| lcm(acc, den/gcd(num, den)));
    l
      .iter()
      .map(|&(num, den)| (num*d/den, d))
      .collect()
}

fn main() {
  assert_eq!(
    vec![(6, 12), (4, 12), (3, 12)], 
    convert_fracts_clever(vec![(1, 2), (1, 3), (1, 4)])
  );

  assert_eq!(
    vec![(18078, 34060), (2262, 34060), (25545, 34060)], 
    convert_fracts_clever(vec![(69, 130), (87, 1310), (3, 4)])
  );

  assert_eq!(
    vec![(18078, 34060), (2262, 34060), (25545, 34060)], 
    convert_fracts_clever(vec![(690, 1300), (87, 1310), (30, 40)])
  );
}
