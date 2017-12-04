use std::cmp;

#[allow(dead_code)]
pub const INPUT: i32 = 347991;

#[allow(dead_code)]
pub fn a(input: i32) -> i32 {
    let sqrt = f64::from(input).sqrt() as i32;
    let nearest_sq = sqrt.pow(2);
    if nearest_sq == input {
        sqrt - 1
    } else {
        let focal_a = nearest_sq + 1 + sqrt / 2;
        let focal_b = focal_a + sqrt + sqrt % 2;
        let focal_dist = sqrt / 2 + sqrt % 2;
        focal_dist + cmp::min((input - focal_a).abs(), (focal_b - input).abs())
    }
}

#[allow(dead_code)]
pub fn b(input: u32) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(1), 0);
        assert_eq!(a(12), 3);
        assert_eq!(a(23), 2);
        assert_eq!(a(1024), 31);
    }
}
