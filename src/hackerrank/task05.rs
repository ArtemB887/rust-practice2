// https://www.hackerrank.com/challenges/apple-and-orange/problem

pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {
    let apple_count = apples.iter()
        .filter(|&&d| {
            let pos = a + d;
            pos >= s && pos <= t
        })
        .count() as i32;

    let orange_count = oranges.iter()
        .filter(|&&d| {
            let pos = b + d;
            pos >= s && pos <= t
        })
        .count() as i32;

    (apple_count, orange_count)
}

#[allow(dead_code)]
pub fn solve(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let (a_count, o_count) = count_apples_and_oranges(s, t, a, b, apples, oranges);
    println!("{}", a_count);
    println!("{}", o_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let (s, t) = (7, 11);
        let (a, b) = (5, 15);
        let apples = [-2, 2, 1];
        let oranges = [5, -6];
        assert_eq!(count_apples_and_oranges(s, t, a, b, &apples, &oranges), (1, 1));
    }
}