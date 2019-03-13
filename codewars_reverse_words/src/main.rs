/**
 * https://www.codewars.com/kata/reverse-words/train/rust
 * 
 * Complete the function that accepts a string parameter, and reverses each word
 * in the string. All spaces in the string should be retained.
 */

fn reverse_words(str: &str) -> String {
    String::from(str)
        .split(' ')
        .map(|x| {
            x
                .chars()
                .rev()
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"),"a b c d");
    assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
}
