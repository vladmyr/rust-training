/**
 * https://www.codewars.com/kata/convert-a-number-to-a-string/rust
 * 
 * Convert a Number to a String!
 */

fn number_to_string(i: i32) -> String {
    format!("{}", i)
}

// best practive
fn number_to_string_best(i: i32) -> String {
    i.to_string()
}

fn main() {
    assert_eq!(number_to_string(0), "0");
    assert_eq!(number_to_string(-89), "-89");
    assert_eq!(number_to_string(179), "179");
}
