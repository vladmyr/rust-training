/**
 * Your task is to construct a building which will be a pile of n cubes. The 
 * cube at the bottom will have a volume of n^3, the cube above will have volume
 * of (n-1)^3 and so on until the top which will have a volume of 1^3.
 * 
 * You are given the total volume m of the building. Being given m can you find 
 * the number n of cubes you will have to build?
 * 
 * The parameter of the function findNb will be an integer m and you have to 
 * return the integer n such as n^3 + (n-1)^3 + ... + 1^3 = m if such a n exists
 * or -1 if there is no such n.
 */

// Attempt #1
fn find_nb(m: u64) -> i32 {
    let mut sum: u64 = 1;
    let mut n: u64 = 1;

    while sum < m {
        n += 1;
        println!("{}", n);
        sum += n.pow(3);
    }

    if sum == m {
        n as i32
    } else {
        -1
    }
}

// Best solution
// Formula: n^2 + n - 2m^(1/2) = 0
// See: https://brilliant.org/wiki/sum-of-n-n2-or-n3/
fn find_nb_best(m: u64) -> i32 {
    let kk = (4.0 * m as f64).sqrt().sqrt().floor() as u64;
    if 4u64 * m == kk * kk * (kk + 1u64) * (kk + 1u64) {
        kk as i32
    } else {
        -1
    }
}

fn main() {
    // assert_eq!(find_nb(0), -1);
    // assert_eq!(find_nb(1), 1);
    // assert_eq!(find_nb(9), 2);
    assert_eq!(find_nb(4183059834009), 2022);
}
