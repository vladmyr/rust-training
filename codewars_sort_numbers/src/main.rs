/**
 * https://www.codewars.com/kata/sort-numbers/train/rust
 * 
 * Finish the solution so that it sorts the passed in array of numbers. If the 
 * function passes in an empty array or null/nil value then it should return an empty array.
 */

fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut sorted = arr.clone();
    sorted.sort();
    sorted
}

fn main() {
    assert_eq!(sort_numbers(&vec![1, 2, 3, 10, 5]), vec![1, 2, 3, 5, 10]);
    assert_eq!(sort_numbers(&vec![]), vec![]);
    assert_eq!(sort_numbers(&vec![20, 2, 10]), vec![2, 10, 20]);
    assert_eq!(sort_numbers(&vec![2, 20, 10]), vec![2, 10, 20]);
    assert_eq!(sort_numbers(&vec![2, 10, 20]), vec![2, 10, 20]);
}
