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
    l
      .iter()
      .map(|x| divisors(*x))
      .map(|mut x| { 
        x.sort(); 
        x 
      })
      .collect::<Vec<Vec<u64>>>()
      .map()
      // .fold(1, |acc, m| {

      // })
      ;

    0
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
}