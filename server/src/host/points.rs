#![allow(dead_code)]

const POINTS_FUNCTION: MathFunction = QUADRATIC;
const MAX_STREAK_BONUS: f64 = 2.0;

// Points functions in order of how punishing they are for slower responses, from most to least.
// They all award at most 1200 and at least 800 points.
const QUADRATIC: MathFunction = |x| 400.0 * x * x - 800.0 * x + 1200.0;
const RECIPROCAL: MathFunction = |x| 800.0 - 57.0 * x + 200.0 / (3.0 * x + 0.5);
const LINEAR: MathFunction = |x| 1200.0 - 400.0 * x;
const CONSTANT: MathFunction = |_| 1200.0;

type MathFunction = fn(f64) -> f64;

pub fn calculate_points(
    start: u128,
    guess_time: u128,
    duration: u128,
    correct: bool,
    streak: usize,
) -> usize {
    if !correct {
        return 0;
    }

    let percentage_progress = (guess_time - start) as f64 / duration as f64;

    (POINTS_FUNCTION(percentage_progress) * calculate_streak_bonus(streak)) as usize
}

pub fn calculate_streak_bonus(streak: usize) -> f64 {
    (1.0 + 0.1 * streak as f64).min(MAX_STREAK_BONUS)
}
