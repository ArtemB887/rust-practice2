pub fn sock_merchant(_n: i32, ar: Vec<i32>) -> i32 {
    let mut counts = [0; 101];
    let mut pairs = 0;

    for sock in ar {
        counts[sock as usize] += 1;
        if counts[sock as usize] % 2 == 0 {
            pairs += 1;
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        assert_eq!(sock_merchant(9, vec![10, 20, 20, 10, 10, 30, 50, 10, 20]), 3);
        assert_eq!(sock_merchant(10, vec![1, 1, 3, 1, 2, 1, 3, 3, 3, 3]), 4);
    }
}