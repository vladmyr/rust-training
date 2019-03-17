fn divisors(n: i64) -> Vec<i64> {
    (1..(n as f64).sqrt() as i64 + 1)
        .filter(|d| n % d == 0)
        .fold(Vec::new(), |mut acc, d| {
            acc.push(d);

            if n / d != d {
                acc.push(n / d);
            }

            acc
        })
}

fn gcd(l: &Vec<i64>) -> i64 {
    let mut divisors_iter = l
      .iter()
      .map(|x| divisors(*x));

    let mut common_divisors = divisors_iter
      .next()
      .map(|divisors| divisors_iter
        .fold(divisors, |x, y| x
          .into_iter()
          .filter(|z| y.contains(z))
          .collect::<Vec<i64>>()
        )
      )
      .unwrap_or(Vec::new());
    
    common_divisors.sort();

    *common_divisors.last().unwrap_or(&0)
}

fn lcm_pair(a: &i64, b: &i64) -> i64 {
  let gcd = gcd(&vec![*a, *b]);

  if gcd == 0 {
    gcd
  } else {
    a / gcd * b
  }
}

fn lcm(l: &Vec<i64>) -> i64 {
  let mut iter = l.into_iter();

  iter
    .next()
    .map(|x| iter.fold(*x, |x, y| lcm_pair(&x, &y)))
    .unwrap_or(0)
}

pub fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
  let denoms = l
    .iter()
    .map(|(_, x)| *x)
    .collect::<Vec<i64>>();

  let lcm = lcm(&denoms);
  
  let mut fractions = l
    .iter()
    .map(|(x, y)| (lcm / y * x, lcm))
    .collect::<Vec<(i64, i64)>>();

  let mut fractions_iter = fractions.iter();

  let gcd = gcd(&fractions_iter
    .next()
    .map(|(x, y)| {
      fractions_iter.fold(vec![*x, *y], |mut acc, (x, _)| {
        acc.push(*x);
        acc
      })
    })
    .unwrap_or(Vec::new())
  );

  if gcd > 1 {
    fractions = fractions
      .iter()
      .map(|(x, y)| (x / gcd, y / gcd))
      .collect::<Vec<(i64, i64)>>();
  }

  fractions
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
    assert_eq!(0, gcd(&vec![]));
  }

  #[test]
  fn gcd_test_54() {
    assert_eq!(54, gcd(&vec![54]));
  }

  #[test]
  fn gcd_test_54_24() {
    assert_eq!(6, gcd(&vec![54, 24]));
  }

  #[test]
  fn gcd_test_2_3_4() {
    assert_eq!(1, gcd(&vec![2, 3, 4]));
  }

  #[test]
  fn gcd_test_130_1310_4() {
    assert_eq!(2, gcd(&vec![130, 1310, 4]));
  }

  #[test]
  fn gcd_test_1300_1310_40() {
    assert_eq!(10, gcd(&vec![1300, 1310, 40]));
  }

  #[test]
  fn lcm_test_6_8() {
    assert_eq!(24, lcm(&vec![6, 8]));
  }

  #[test]
  fn lcm_test_2_3_4() {
    assert_eq!(12, lcm(&vec![2, 3, 4]));
  }

  #[test]
  fn convert_fracts_test_1() {
    assert_eq!(
      vec![(6, 12), (4, 12), (3, 12)], 
      convert_fracts(vec![(1, 2), (1, 3), (1, 4)])
    );
  }

  #[test]
  fn convert_fracts_test_2() {
    assert_eq!(
      vec![(18078, 34060), (2262, 34060), (25545, 34060)], 
      convert_fracts(vec![(69, 130), (87, 1310), (3, 4)])
    );
  }

  #[test]
  fn convert_fracts_test_3() {
    assert_eq!(
      vec![(18078, 34060), (2262, 34060), (25545, 34060)], 
      convert_fracts(vec![(690, 1300), (87, 1310), (30, 40)])
    );
  }
}