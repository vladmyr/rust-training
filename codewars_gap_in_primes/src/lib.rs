pub fn sieve(n: usize) -> Vec<usize> {
    let n_normalized = n - 1;
    let mut p = 0;
    let mut sieve: Vec<bool> = vec![false; n_normalized];
    let mut result: Vec<usize> = Vec::new();

    while p < n_normalized {
        let mut index = p;

        if sieve[index] == false {
            result.push(index + 2);
         
            while index < n_normalized {
                sieve[index] = true;
                index += p + 2;
            }
        }

        p += 1;
    }

    result
}

pub fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    if g < 2 || m < 3 || n < m {
        return None;
    }

    let pairs: (u64, u64) = sieve(n as usize)
        .iter()
        .filter(|x| (**x as u64) >= m)
        .fold(Vec::new() as Vec<(u64, u64)>, |mut acc, x| {
            match acc.last() {
                None => acc.push((0, *x as u64)),
                Some(y) => acc.push((y.1, *x as u64))
            }

            acc
        })
        // .iter()
        // .skip(1)
        // .filter(|x| x.1 - x.0 != g as u64)
        // .take(1)
        ;

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sieve_test() {
        assert_eq!(sieve(2), vec![2]);
        assert_eq!(sieve(3), vec![2,3]);
        assert_eq!(sieve(10), vec![2,3,5,7]);
        assert_eq!(sieve(12), vec![2,3,5,7,11]);
        assert_eq!(sieve(13), vec![2,3,5,7,11,13]);
    }

    #[test]
    fn basics_gap() {
        assert_eq!(gap(2, 100, 110), Some((101, 103)));
        assert_eq!(gap(4, 100, 110), Some((103, 107)));
        assert_eq!(gap(6, 100, 110), None);
        assert_eq!(gap(8, 300, 400), Some((359, 367)));
    }
}
