fn divisors(n: u64) -> Vec<u64> {
    (1..(n as f64).sqrt() as u64 + 1)
        .filter(|d| n % d == 0)
        .fold(Vec::new(), |mut acc, d| {
            acc.push(d);

            if n / d != d {
                acc.push(n / d);
            }

            acc
        })
}

fn gcd(l: Vec<u64>) -> u64 {
    let divisors_matrix = l
      .iter()
      .map(|x| divisors(*x))
      .collect::<Vec<Vec<u64>>>();

    let mut common_divisors = divisors_matrix
      .iter()
      .take(1)
      .flat_map(|x| x
        .iter()
        .filter(|x| divisors_matrix.iter().skip(1).all(|y| y.contains(x)))
        .map(|x| *x)
        .collect::<Vec<u64>>()
      )
      .collect::<Vec<u64>>();
    
    common_divisors.sort();

    *common_divisors.last().unwrap_or(&0)
}

pub fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let denom = l
        .iter()
        .fold(1, |acc, (_, x)| acc * x);

    vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn divisors_test_54() {
    let mut res = divisors(54);
    res.sort();
    assert_eq!(vec![1, 2, 3, 6, 9, 18, 27, 54], res);
  }

  #[test]
  fn divisors_test_24() {
    let mut res = divisors(24);
    res.sort();
    assert_eq!(vec![1, 2, 3, 4, 6, 8, 12, 24], res);
  }

  #[test]
  fn gcd_test_0() {
    assert_eq!(0, gcd(vec![]));
  }

  #[test]
  fn gcd_test_54() {
    assert_eq!(54, gcd(vec![54]));
  }

  #[test]
  fn gcd_test_54_24() {
    assert_eq!(6, gcd(vec![54, 24]));
  }
}