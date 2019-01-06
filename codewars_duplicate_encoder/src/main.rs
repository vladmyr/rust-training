/**
 * https://www.codewars.com/kata/duplicate-encoder/train/rust
 * 
 * The goal of this exercise is to convert a string to a new string where each 
 * character in the new string is '(' if that character appears only once in the
 * original string, or ')' if that character appears more than once in the 
 * original string. Ignore capitalization when determining if a character is a 
 * duplicate.
 * Examples:
 *      "din" => "((("
 *      "recede" => "()()()"
 *      "Success" => ")())())"
 *      "(( @" => "))(("
 */

fn duplicate_encode(word: &str) -> String {
    let mut chars_m: Vec<Option<char>> = word
        .to_lowercase()
        .chars()
        .map(|char| Some(char))
        .collect();
    let mut encoded: Vec<char> = vec![')'; chars_m.len()];

    for i in 0..chars_m.len() {
        match chars_m[i] {
            None => continue,
            Some(chr) => {
                let mut has_multiple_occurrences = false;

                for j in (i + 1)..chars_m.len() {
                    match chars_m[j] {
                        None => continue,
                        Some(chr_subsequent) => {
                            if chr == chr_subsequent {
                                has_multiple_occurrences = true;
                                chars_m[j] = None;
                            }
                        }
                    }
                }

                if !has_multiple_occurrences {
                    encoded[i] = '(';
                }

                chars_m[i] = None;
            }
        }
    }

    encoded.into_iter().collect()
}

// best practice
fn duplicate_encode_best(word: &str) -> String {
  let mut enc = std::collections::HashMap::new();
  for c in word.to_lowercase().chars() {
    *enc.entry(c).or_insert(0) += 1;
  }
  word.to_lowercase().chars().map(|c| match *enc.get(&c).unwrap() {
    1 => '(',
    _ => ')'
  }).collect()
}

// the most clever
fn duplicate_encode_clever(word:&str)->String {
  let lower = String::from(word).to_lowercase();
  lower.chars().map(|c| if lower.find(c) == lower.rfind(c) { '(' } else { ')' }).collect()
}

fn main() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())");
    assert_eq!(duplicate_encode("(( @"), "))((");

    assert_eq!(duplicate_encode_best("din"), "(((");
    assert_eq!(duplicate_encode_best("recede"), "()()()");
    assert_eq!(duplicate_encode_best("Success"), ")())())");
    assert_eq!(duplicate_encode_best("(( @"), "))((");

    assert_eq!(duplicate_encode_clever("din"), "(((");
    assert_eq!(duplicate_encode_clever("recede"), "()()()");
    assert_eq!(duplicate_encode_clever("Success"), ")())())");
    assert_eq!(duplicate_encode_clever("(( @"), "))((");
}
