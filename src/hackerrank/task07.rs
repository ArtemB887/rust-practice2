// https://www.hackerrank.com/challenges/between-two-sets/problem
pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut count = 0;
    let min_val = *a.iter().max().unwrap_or(&0);
    let max_val = *b.iter().min().unwrap_or(&100);

    for x in min_val..=max_val {
        let is_multiple_of_a = a.iter().all(|&i| x % i == 0);
        let is_factor_of_b = b.iter().all(|&i| i % x == 0);

        if is_multiple_of_a && is_factor_of_b {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        assert_eq!(get_total_x(vec![2, 4], vec![16, 32, 96]), 3);
        assert_eq!(get_total_x(vec![3, 4], vec![24, 48]), 2);
    }
}