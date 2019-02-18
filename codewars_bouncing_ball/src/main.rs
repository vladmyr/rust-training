/**
 * https://www.codewars.com/kata/5544c7a5cb454edb3c000047/train/rust
 * 
 * A child is playing with a ball on the nth floor of a tall building. The height of this floor, h, is known.
 * He drops the ball out of the window. The ball bounces (for example), to two-thirds of its height (a bounce of 0.66).
 * His mother looks out of a window 1.5 meters from the ground.
 * How many times will the mother see the ball pass in front of her window (including when it's falling and bouncing?
 * Three conditions must be met for a valid experiment:
 * - Float parameter "h" in meters must be greater than 0
 * - Float parameter "bounce" must be greater than 0 and less than 1
 * - Float parameter "window" must be less than h.
 * If all three conditions above are fulfilled, return a positive integer, otherwise return -1.
 * Note: The ball can only be seen if the height of the rebounding ball is stricty greater than the window parameter.
 * Example:
 * h = 3, bounce = 0.66, window = 1.5, result is 3
 * h = 3, bounce = 1, window = 1.5, result is -1 (Condition 2) not fulfilled).
 */

fn main() {
    assert_eq!(bouncing_ball(3.0, 0.66, 1.5), 3);
    assert_eq!(bouncing_ball(30.0, 0.66, 1.5), 15);
    assert_eq!(bouncing_ball(40.0, 0.4, 10.0), 3);
    assert_eq!(bouncing_ball(10.0, 0.6, 10.0), -1);
}

fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h < 0f64 || bounce <= 0f64 || bounce >= 1f64 || h <= window {
        return -1;
    }

    let new_h = h * bounce;
    if new_h < window {
        1
    } else {
        2 + bouncing_ball(new_h, bounce, window)
    }
}

// the best practice solution
fn bouncing_ball_best(h: f64,  bounce: f64,  window: f64) -> i32 {
    let parameters_are_valid = h > 0_f64 && 0_f64 < bounce && bounce < 1_f64 && window < h;
    if !parameters_are_valid {
      -1
    } else {
      //            w = h * b^n
      // log[b] (w/h) = log[b](b^n)
      // log[b] (w/h) = n
      1 + 2 * (window / h).log(bounce) as i32
    }
}

// the most clever solution
fn bouncing_ball_clever(h: f64, bounce: f64, window: f64) -> i32 {
    if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
        -1
    } else {
        (window / h).log(bounce).ceil() as i32 * 2 - 1
    }
}