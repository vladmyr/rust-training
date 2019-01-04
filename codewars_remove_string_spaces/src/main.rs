/**
 * https://www.codewars.com/kata/remove-string-spaces/rust
 * 
 * Remove String Spaces
 * Simple, remove the spaces from the string, then return the resultant string.
 */

fn no_space(x: String) -> String {
    x.replace(' ', "")
}

// Most clever solution
fn no_space_clever(x: String) -> String {
    x.split_whitespace().collect()
}

// Functional style
fn no_space_fn(x: String) -> String {
    x.chars().filter(|&c| c != ' ').collect()
}

fn main() {
    assert_eq!(no_space(String::from(" ")), "");
    assert_eq!(no_space(String::from("8 j 8   mBliB8g  imjB8B8  jl  B")), "8j8mBliB8gimjB8B8jlB");
    assert_eq!(no_space(String::from("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd")), "88Bifk8hB8BB8BBBB888chl8BhBfd");
    assert_eq!(no_space(String::from("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd")), "88Bifk8hB8BB8BBBB888chl8BhBfd");
    assert_eq!(no_space(String::from("8aaaaa dddd r     ")), "8aaaaaddddr");
    assert_eq!(no_space(String::from("jfBm  gk lf8hg  88lbe8 ")), "jfBmgklf8hg88lbe8");
    assert_eq!(no_space(String::from("8j aam")), "8jaam");

    assert_eq!(no_space_clever(String::from(" ")), "");
    assert_eq!(no_space_clever(String::from("8 j 8   mBliB8g  imjB8B8  jl  B")), "8j8mBliB8gimjB8B8jlB");
    assert_eq!(no_space_clever(String::from("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd")), "88Bifk8hB8BB8BBBB888chl8BhBfd");
    assert_eq!(no_space_clever(String::from("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd")), "88Bifk8hB8BB8BBBB888chl8BhBfd");
    assert_eq!(no_space_clever(String::from("8aaaaa dddd r     ")), "8aaaaaddddr");
    assert_eq!(no_space_clever(String::from("jfBm  gk lf8hg  88lbe8 ")), "jfBmgklf8hg88lbe8");
    assert_eq!(no_space_clever(String::from("8j aam")), "8jaam");

    assert_eq!(no_space_fn(String::from(" ")), "");
    assert_eq!(no_space_fn(String::from("8 j 8   mBliB8g  imjB8B8  jl  B")), "8j8mBliB8gimjB8B8jlB");
    assert_eq!(no_space_fn(String::from("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd")), "88Bifk8hB8BB8BBBB888chl8BhBfd");
    assert_eq!(no_space_fn(String::from("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd")), "88Bifk8hB8BB8BBBB888chl8BhBfd");
    assert_eq!(no_space_fn(String::from("8aaaaa dddd r     ")), "8aaaaaddddr");
    assert_eq!(no_space_fn(String::from("jfBm  gk lf8hg  88lbe8 ")), "jfBmgklf8hg88lbe8");
    assert_eq!(no_space_fn(String::from("8j aam")), "8jaam");
}
