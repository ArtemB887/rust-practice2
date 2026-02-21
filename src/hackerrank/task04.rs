// https://www.hackerrank.com/challenges/grading/problem

pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade
        } else {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5
            } else {
                grade
            }
        }
    }).collect()
}

#[allow(dead_code)]
pub fn solve(grades: &[i32]) {
    let result = grading_students(grades);
    for grade in result {
        println!("{}", grade);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let input = [73, 67, 38, 33];
        let expected = [75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }
}