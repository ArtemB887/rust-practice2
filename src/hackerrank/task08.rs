// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
pub fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut min = scores[0];
    let mut max = scores[0];
    let mut min_count = 0;
    let mut max_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max {
            max = score;
            max_count += 1;
        } else if score < min {
            min = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        assert_eq!(breaking_records(vec![10, 5, 20, 20, 4, 5, 2, 25, 1]), vec![2, 4]);
        assert_eq!(breaking_records(vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42]), vec![4, 0]);
    }
}