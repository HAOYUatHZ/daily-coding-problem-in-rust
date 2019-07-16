#[allow(dead_code)]
pub fn check_sums(a: i32,  b: i32) -> i32 {
    return a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_check_sums() {
        assert_eq!(check_sums(1, 2), 3);
    }
}