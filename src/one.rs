fn one(input: &str) -> u32 {
    input
        .chars()
        .chain(input.chars().take(1))
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>()
        .windows(2)
        .map(|a| if a[1] == a[0] { a[0] } else { 0 })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn consecutive() {
        assert_eq!(one("1122"), 3);
    }

    #[test]
    fn equal() {
        assert_eq!(one("1111"), 4);
    }

    #[test]
    fn different() {
        assert_eq!(one("1234"), 0);
    }

    #[test]
    fn loops() {
        assert_eq!(one("91212129"), 9);
    }
}
